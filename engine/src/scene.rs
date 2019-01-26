use std::collections::BTreeMap;

use game_object::GameObject;
use drawable::DrawContext;
use Engine;

pub type SceneObjectId = i32;

pub struct Scene {
    objects: BTreeMap<SceneObjectId, Box<GameObject>>,
    current_id: SceneObjectId
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: BTreeMap::new(),
            current_id: 0
        }
    }

    pub fn get(&self, id: SceneObjectId) -> Option<&Box<GameObject>> {
        self.objects.get(&id)
    }

    pub fn get_mut(&mut self, id: SceneObjectId) -> Option<&mut Box<GameObject>> {
        self.objects.get_mut(&id)
    }

    pub fn update(&mut self, engine: &mut Engine, dt: f32) {
        for (_id, object) in self.objects.iter_mut() {
            object.update(engine, dt);
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
}
