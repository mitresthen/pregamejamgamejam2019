use std::collections::BTreeMap;

use game_object::{
    GameObject,
    EventQueue,
    EventReceiver,
    EventType,
    EventMailbox,
    CollisionShape
};

use ray_shape::RayShape;
use drawable::DrawContext;
use vector::Vec2;
use rect::Rect2D;
use Engine;

pub type SceneObjectId = i32;

#[derive(Clone, Copy)]
pub struct SceneForceId {
    id: usize,
}

pub trait Force {
    fn calculate_force_on_object(&self, position: Vec2, inv_mass: f32) -> Vec2;
}

pub struct Scene {
    objects: BTreeMap<SceneObjectId, Box<dyn GameObject>>,
    current_id: SceneObjectId,
    event_queue: EventQueue,
    pending_raycasts: Vec<(Vec2, Vec2, SceneObjectId)>,
    forces: Vec<Box<dyn Force>>,
}

pub trait LevelCollider {
    fn get_collision_vector(&self, collision_shape: &dyn CollisionShape) -> Option<Vec2>;
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: BTreeMap::new(),
            current_id: 0,
            event_queue: EventQueue::new(),
            pending_raycasts: Vec::new(),
            forces: Vec::new(),
        }
    }

    pub fn add_force<T: Force + 'static>(&mut self, force: T) -> SceneForceId {
        let id = SceneForceId { id: self.forces.len() };

        self.forces.push(Box::new(force));

        return id;
    }

    pub fn remove_force(&mut self, id: SceneForceId) {
        self.forces.remove(id.id);
    }

    pub fn get(&self, id: SceneObjectId) -> Option<&Box<dyn GameObject>> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: SceneObjectId) -> Option<&mut Box<dyn GameObject>> {
        self.objects.get_mut(&id)
    }

    pub fn dispatch_nearest_event(
        &mut self,
        origin: Vec2,
        max_distance: Option<f32>,
        event: EventType,
        sender: Option<SceneObjectId>
    ) {
        use std::f32;

        let mut objects_with_distance : Vec<(f32, &mut Box<dyn GameObject>)> =
            self.objects.iter_mut().map(
                |(_id, ob)| {
                    let distance = 
                        if let Some(pob) = ob.get_physical_object() {
                            let position = pob.get_transform().get_translation();
                            (position - origin).len()
                        } else {
                            f32::MAX
                        };

                    (distance, ob)
                }
            ).collect();

        objects_with_distance.sort_by(|(d_a, _o_a), (d_b, _o_b)| d_a.partial_cmp(d_b).unwrap());

        let mut it = objects_with_distance.iter_mut();

        while let Some((distance, object)) = it.next() {
            if *distance > max_distance.unwrap_or(f32::MAX) {
                println!("Event lost because max distance was reached: distance={}", distance);
                break;
            }

            if object.on_event(event.clone(), sender) {
                // Object handlet the event. 
                break;
            }
        }
    }

    pub fn dispatch_nearby_event(
        &mut self,
        origin: Vec2,
        max_distance: Option<f32>,
        event: EventType,
        sender: Option<SceneObjectId>
    ) {
        use std::f32;

        let mut objects_with_distance : Vec<(f32, &mut Box<dyn GameObject>)> =
            self.objects.iter_mut().map(
                |(_id, ob)| {
                    let distance = 
                        if let Some(pob) = ob.get_physical_object() {
                            let position = pob.get_transform().get_translation();
                            (position - origin).len()
                        } else {
                            f32::MAX
                        };

                    (distance, ob)
                }
            ).collect();

        objects_with_distance.sort_by(|(d_a, _o_a), (d_b, _o_b)| d_a.partial_cmp(d_b).unwrap());

        let mut it = objects_with_distance.iter_mut();

        while let Some((distance, object)) = it.next() {
            if *distance > max_distance.unwrap_or(f32::MAX) {
                println!("Event lost because max distance was reached: distance={}", distance);
                break;
            }

            object.on_event(event.clone(), sender);
        }
    }

    pub fn broadcast_event(&mut self, event: EventType, sender: Option<SceneObjectId>) {
        for (_id, object) in self.objects.iter_mut() {
            object.on_event(event.clone(), sender);
        }
    }

    pub fn do_level_collision(&mut self, collider: &dyn LevelCollider) {
        for (origin, target, object_id) in self.pending_raycasts.drain(..) {

            println!("Raycast {:?} -> {:?}", origin, target);

            let ray = RayShape::new(target, origin);
            if target.valid() && origin.valid() {
                let success : bool = collider.get_collision_vector(&ray).is_none();

                self.event_queue.submit_event(
                    EventType::RayCastReply { success, target },
                    EventReceiver::Addressed { object_id }
                );
            } else {
                let success = false;
                self.event_queue.submit_event(
                    EventType::RayCastReply { success, target },
                    EventReceiver::Addressed { object_id }
                );
            }
        }

        for (_id, object) in self.objects.iter_mut() {
            let mut maybe_axis = None;
            if let Some(physical_object) = object.get_physical_object_mut() {
                if let Some(bounding_box) = physical_object.get_bounding_box() {
                    if let Some(axis) = collider.get_collision_vector(bounding_box.as_ref()) {
                        let velocity = physical_object.get_velocity_mut();
                        let perp = axis.perpendicular();

                        *velocity = (axis * 100.0) + (perp * perp.dot_product(*velocity));
                        maybe_axis = Some(axis);
                    }
                }
            }

            if let Some(axis) = maybe_axis {
                object.on_event(EventType::Collide { force: axis }, None);
            }
        }
    }

    pub fn handle_scene_event(&mut self, event_type: EventType, sender: Option<SceneObjectId>) {
        match event_type {
            EventType::RayCast { origin, target } => {
                if let Some(s) = sender {
                    self.pending_raycasts.push((origin, target, s));
                } else {
                    println!("Got RayCast request without sender id");
                }
            },
            EventType::DeleteMe => {
                self.remove_object(sender.unwrap());
            },
            _ => { }
        }
    }

    pub fn update(&mut self, engine: &mut Engine, collider: Option<&dyn LevelCollider>, dt: f32) {
        {
            let mut collision_pairs : Vec<(SceneObjectId, SceneObjectId, Vec2, bool)> = Vec::new();

            for (_, o) in self.objects.iter_mut( ) {
                if let Some(po) = o.get_physical_object_mut() {
                    for f in self.forces.iter() {
                        let position = po.get_transform().get_translation();
                        let inv_mass = po.get_inv_mass();
                        let force = f.calculate_force_on_object(position, inv_mass);

                        let acceleration = force * inv_mass;

                        *po.get_velocity_mut() = *po.get_velocity() + acceleration * dt;
                    }
                }
            }

            {
                let mut it = self.objects.iter();

                while let Some((id_a, object_a)) = it.next() {
                    let mut jt = it.clone();

                    while let Some((id_b, object_b)) = jt.next() {

                        if let Some(physical_object_a) = object_a.get_physical_object() {
                            if let Some(physical_object_b) = object_b.get_physical_object() {
                                if let Some(bounding_box_a) = physical_object_a.get_bounding_box() {
                                    if let Some(bounding_box_b) = physical_object_b.get_bounding_box() {
                                        if let Some(result) = bounding_box_a.sat_collide(bounding_box_b.as_ref()) {
                                            let should_block =
                                                physical_object_a.should_block() &&
                                                physical_object_b.should_block();

                                            collision_pairs.push((*id_a, *id_b, result.axis, should_block));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }


            for (ob_a, ob_b, axis, should_block) in collision_pairs.drain(..) {

                if should_block
                {
                    let perp = axis.perpendicular();

                    let physical_object_a = self.objects.get(&ob_a).unwrap().get_physical_object().unwrap();
                    let physical_object_b = self.objects.get(&ob_b).unwrap().get_physical_object().unwrap();

                    let velocity_a = physical_object_a.get_velocity();
                    let velocity_b = physical_object_b.get_velocity();

                    let velo_axis_a = axis.dot_product(*velocity_a);
                    let velo_axis_b = axis.dot_product(*velocity_b);

                    let inv_mass_a = physical_object_a.get_inv_mass();
                    let inv_mass_b = physical_object_b.get_inv_mass();

                    let diff = velo_axis_a - velo_axis_b;

                    if (inv_mass_a > 0.0 || inv_mass_b > 0.0) && diff < 0.0 {
                        // (ma * va) + (mb * vb) = (ma * v) + (mb * v)
                        // (ma * va) + (mb * vb) = v * (ma + mb)
                        // (ma * va) + (mb * vb) = v * (ma + mb) | / (ma * mb)
                        // ((1/mb) * va) + ((1/ma) * vb) = v * ((1 / mb) + (1 / ma))
                        //
                        let v = ((inv_mass_b * velo_axis_a) + (inv_mass_a * velo_axis_b)) / (inv_mass_b + inv_mass_a);

                        let delta_v_a = v - velo_axis_a;
                        let delta_v_b = v - velo_axis_b;

                        {
                            let physical_object_a = self.objects.get_mut(&ob_a).unwrap().get_physical_object_mut().unwrap();
                            let velocity_a = physical_object_a.get_velocity_mut();
                            *velocity_a = (axis * delta_v_a) + (perp * perp.dot_product(*velocity_a));
                        }

                        {
                            let physical_object_b = self.objects.get_mut(&ob_b).unwrap().get_physical_object_mut().unwrap();
                            let velocity_b = physical_object_b.get_velocity_mut();
                            *velocity_b = (axis * delta_v_b) + (perp * perp.dot_product(*velocity_b));
                        }
                    }
                }

                self.event_queue.submit_event(
                    EventType::Collide { force: axis },
                    EventReceiver::Addressed { object_id: ob_a }
                );

                self.event_queue.submit_event(
                    EventType::Collide { force: axis },
                    EventReceiver::Addressed { object_id: ob_b }
                );
            }
        }

        if let Some(level_collider) = collider {
            self.do_level_collision(level_collider);
        }

        for (id, object) in self.objects.iter_mut() {
            object.update(engine, &mut self.event_queue.bind_to_sender(*id), dt);
        }

        for (_, o) in self.objects.iter_mut( ) {
            if let Some(po) = o.get_physical_object_mut() {
                let translate = *po.get_velocity() * dt;
                po.get_transform_mut().translate(translate);
            }
        }

        while let Some(event) = self.event_queue.poll() {
            match event.receiver {
                EventReceiver::Nearest { origin, max_distance } => {
                    self.dispatch_nearest_event(origin, max_distance, event.event_type, event.sender)
                },
                EventReceiver::Broadcast => {
                    self.broadcast_event(event.event_type, event.sender)
                },
                EventReceiver::Scene => {
                    self.handle_scene_event(event.event_type, event.sender);
                },
                EventReceiver::Addressed { object_id } => {
                    if self.objects.contains_key(&object_id) {
                        self.objects.get_mut(&object_id).unwrap().on_event(event.event_type, event.sender);
                    }
                },
                EventReceiver::Nearby { origin, max_distance } => {
                    self.dispatch_nearby_event(origin, max_distance, event.event_type, event.sender)
                }
            }
        }
    }

    pub fn render(&self, engine: &mut Engine) {
        let screen_bounds = engine.get_screen_bounds();

        let mut ctx =
            DrawContext::new(
                &mut engine.canvas,
                &mut engine.texture_registry,
                &engine.camera,
                screen_bounds
            );

        for (_id, object) in self.objects.iter() {
            object.render(&mut ctx);
        }
    }

    pub fn add_object<T: GameObject>(&mut self, object: T) -> SceneObjectId {
        let new_id = self.current_id;
        self.current_id += 1;
        self.objects.insert(new_id, Box::new(object));
        new_id
    }

    pub fn remove_object(&mut self, object_id: SceneObjectId){
        println!("Attempting to delete object");
        if self.objects.contains_key(&object_id) {
            self.objects.remove(&object_id);
        }
    }

    pub fn get_objects_in_rect(&self, rect: Rect2D) -> Vec<&Box<dyn GameObject>> {
        let mut result = Vec::new();
        for (_id, object) in self.objects.iter() {
            if let Some(physical_object) = object.get_physical_object() {
                let translation =
                    physical_object
                        .get_transform()
                        .get_translation();

                if rect.contains(translation) {
                    result.push(object);
                }
            }
        }

        result
    }
}
