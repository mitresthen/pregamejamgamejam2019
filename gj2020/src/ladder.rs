extern crate rand;

use std::rc::Rc;
use engine::prelude::*;
use self::rand::Rng;

pub struct Ladder {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    inv_mass: f32,
    shape: Rc<CollisionShape>,
}


impl Ladder {
    pub fn new(ctx: &mut Engine) -> Result<Ladder, Error> {
        let sprite;
        {
            let tr = ctx.get_texture_registry();
            let texture_on = tr.load("assets/images/ladder.png")?;
            sprite = AnimatedSprite::new(Extent::new(60, 240), texture_on)?;
        }

        let size = sprite.calculate_size();
        let shape = SquareShape::from_aabb(Rect2D::centered_rectangle(size));

        let mut ladder =
            Ladder {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                inv_mass: 0.0,
                shape: Rc::new(shape),
            };
            ladder.transform.set_scale(1.0);

        Ok(ladder)
    }

    

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn set_transform(&mut self, input_transform: Transform) {
        self.transform = input_transform;
        self.sprite.set_transform(&self.transform);
    }
}

impl GameObject for Ladder {
    fn update(&mut self, ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        

        true
    }

    fn render(&self, ctx: &mut DrawContext) {
        self.sprite.draw(ctx)
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> {
        Some(self)
    }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> {
        Some(self)
    }

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        true
    }
}

impl PhysicalObject for Ladder {
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

    fn get_inv_mass(&self) -> f32 { self.inv_mass }

    fn get_collision_shape(&self) -> Option<Rc<dyn CollisionShape>> {
        Some(self.shape.clone())
    }
}
