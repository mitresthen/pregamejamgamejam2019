use std::collections::BTreeMap;

use game_object::{GameObject, EventQueue, EventReceiver, EventType};
use bounding_box::BoundingBox;
use drawable::DrawContext;
use vector::Vec2;
use rect::Rect2D;
use Engine;

pub type SceneObjectId = i32;

pub struct Scene {
    objects: BTreeMap<SceneObjectId, Box<GameObject>>,
    current_id: SceneObjectId,
    event_queue: EventQueue
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
            event_queue: EventQueue::new()
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

            if object.on_event(event, sender) {
                // Object handlet the event. 
                break;
            }
        }
    }

    pub fn broadcast_event(&mut self, event: EventType, sender: Option<SceneObjectId>) {
        for (_id, object) in self.objects.iter_mut() {
            object.on_event(event, sender);
        }
    }

    pub fn do_level_collision(&mut self, collider: &LevelCollider) {
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

    pub fn update(&mut self, engine: &mut Engine, dt: f32) {
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
