use std::cell::RefCell;
use std::rc::Rc;

use engine::prelude::*;

pub struct MinigameTrigger {
    value: Rc<RefCell<bool>>
}

impl Clone for MinigameTrigger {
    fn clone(&self) -> Self {
        MinigameTrigger {
            value: Rc::clone(&self.value)
        }
    }
}

impl MinigameTrigger {
    fn new() -> MinigameTrigger {
        MinigameTrigger {
            value: Rc::new(RefCell::new(false))
        }
    }

    fn trigger(&mut self) {
        *self.value.borrow_mut() = true;
    }

    pub fn is_triggered(&self) -> bool {
        let mut value_mut = self.value.borrow_mut();

        if *value_mut {
            *value_mut = false;
            return true;
        }

        return false;
    }
}

pub struct Minigame {
    texture: Texture,
    transform: Transform,
    velocity: Vec2,
    trigger: MinigameTrigger
}

impl Minigame {
    pub fn new(ctx: &mut Engine, texture: Texture, position: Vec2) -> Minigame {
        let mut transform = Transform::new();
        transform.set_translation(position);

        Minigame {
            texture,
            transform,
            velocity: Vec2::from_coords(0.0, 0.0),
            trigger: MinigameTrigger::new(),
        }
    }

    pub fn get_trigger(&self) -> MinigameTrigger {
        self.trigger.clone()
    }
}

impl GameObject for Minigame {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        self.transform.translate(self.velocity * dt);
        return true;
    }

    fn render(&self, ctx: &mut DrawContext) {
        ctx.draw(&self.texture, &self.transform);
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { Some(self) }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { Some(self) }

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Interact => {
                self.trigger.trigger();
                true
            },
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for Minigame {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn get_velocity(&self) -> &Vec2 {
        &self.velocity
    }

    fn get_velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }

    fn get_bounding_box(&self) -> Option<BoundingBox> {
        let bounding_box =
            BoundingBox::new(
                240.0,
                240.0,
                self.transform.get_translation()
            );

        Some(bounding_box)
    }

    fn get_inv_mass(&self) -> f32 { 0.0 }
}
