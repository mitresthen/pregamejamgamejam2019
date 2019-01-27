use std::collections::BTreeMap;

use game_object::{GameObject, EventQueue, EventReceiver, EventType, EventMailbox};
use bounding_box::BoundingBox;
use drawable::DrawContext;
use vector::Vec2;
use rect::Rect2D;
use Engine;

pub type SceneObjectId = i32;

pub struct Scene {
    objects: BTreeMap<SceneObjectId, Box<GameObject>>,
    current_id: SceneObjectId,
    event_queue: EventQueue,
    pending_raycasts: Vec<(Vec2, Vec2, SceneObjectId)>
}

pub trait LevelCollider {
    fn get_collision_vector(&self, bounding_box: BoundingBox) -> Option<Vec2>;

    fn get_collision_vector_points(&self, points : Vec<Vec2>) -> Option<Vec2>;
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: BTreeMap::new(),
            current_id: 0,
            event_queue: EventQueue::new(),
            pending_raycasts: Vec::new()
        }
    }

    pub fn get(&self, id: SceneObjectId) -> Option<&Box<GameObject>> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: SceneObjectId) -> Option<&mut Box<GameObject>> {
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

        let mut objects_with_distance : Vec<(f32, &mut Box<GameObject>)> =
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

        objects_with_distance.sort_by(|(dA, _oA), (dB, _oB)| dA.partial_cmp(dB).unwrap());

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

        let mut objects_with_distance : Vec<(f32, &mut Box<GameObject>)> =
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

        objects_with_distance.sort_by(|(dA, _oA), (dB, _oB)| dA.partial_cmp(dB).unwrap());

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

    pub fn do_level_collision(&mut self, collider: &LevelCollider) {
        for (origin, target, object_id) in self.pending_raycasts.drain(..) {
            let mut points = Vec::new();
            points.push(target);
            points.push(origin);

            println!("Raycast {:?} -> {:?}", origin, target);

            if target.valid() && origin.valid() {
                let success : bool = collider.get_collision_vector_points(points).is_none();

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
                    if let Some(axis) = collider.get_collision_vector(bounding_box) {
                        let velocity = physical_object.get_velocity_mut();
                        *velocity = axis * 100.0;
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
        match (event_type) {
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

    pub fn update(&mut self, engine: &mut Engine, collider: Option<&LevelCollider>, dt: f32) {
        {
            let mut collision_pairs : Vec<(SceneObjectId, SceneObjectId, Vec2)> = Vec::new();

            {
                let mut it = self.objects.iter();

                while let Some((id_a, object_a)) = it.next() {
                    let mut jt = it.clone();

                    while let Some((id_b, object_b)) = jt.next() {

                        if let Some(physical_object_a) = object_a.get_physical_object() {
                            if let Some(physical_object_b) = object_b.get_physical_object() {
                                if let Some(bounding_box_a) = physical_object_a.get_bounding_box() {
                                    if let Some(bounding_box_b) = physical_object_b.get_bounding_box() {
                                        if let Some((axis, _)) = bounding_box_a.sat_overlap(bounding_box_b) {
                                            collision_pairs.push((*id_a, *id_b, axis));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for (ob_a, ob_b, axis) in collision_pairs.drain(..) {
                {
                    let physical_object_a = self.objects.get_mut(&ob_a).unwrap().get_physical_object_mut().unwrap();
                    let velocity_a = physical_object_a.get_velocity_mut();
                    *velocity_a = axis * -220.0;
                    self.event_queue.submit_event(
                        EventType::Collide { force: axis },
                        EventReceiver::Addressed { object_id: ob_a }
                    );
                }
                {
                    let physical_object_b = self.objects.get_mut(&ob_a).unwrap().get_physical_object_mut().unwrap();
                    let velocity_b = physical_object_b.get_velocity_mut();
                    *velocity_b = axis * 220.0;
                    self.event_queue.submit_event(
                        EventType::Collide { force: axis },
                        EventReceiver::Addressed { object_id: ob_b }
                    );
                }
            }
        }

        if let Some(level_collider) = collider {
            self.do_level_collision(level_collider);
        }


        for (id, object) in self.objects.iter_mut() {
            object.update(engine, &mut self.event_queue.bind_to_sender(*id), dt);
        }


        while let Some(event) = self.event_queue.poll() {
            match (event.receiver) {
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
                    self.objects.get_mut(&object_id).unwrap().on_event(event.event_type, event.sender);
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

    pub fn remove_object(&mut self, objectId: SceneObjectId){
        println!("Attempting to delet object");
        self.objects.remove(&objectId);
    }

    pub fn get_objects_in_rect(&self, rect: Rect2D) -> Vec<&Box<GameObject>> {
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
