extern crate rand;

use std::rc::Rc;
use engine::prelude::*;
use self::rand::Rng;

pub struct Ocean {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    inv_mass: f32,
    shape: Rc<dyn CollisionShape>,
    change: f32
}


impl Ocean {
    pub fn new(ctx: &mut Engine) -> Result<Ocean, Error> {
        let sprite;
        {
            let tr = ctx.get_texture_registry();
            let texture_on = tr.load("assets/images/ocean.png")?;
            sprite = AnimatedSprite::new(Extent::new(1600,1200), texture_on)?;
        }

        let mut size = sprite.calculate_size();
        let shape = SquareShape::from_aabb(Rect2D::centered_rectangle(size));
        let mut transform = Transform::new();
        transform.set_translation(Vec2{x:0.0, y:1950.0});

        let mut ocean =
            Ocean {
                sprite: sprite,
                transform: transform,
                velocity: Vec2::new(),
                inv_mass: 0.0,
                shape: Rc::new(shape),
                change: 0.0
            };

        Ok(ocean)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn set_transform(&mut self, input_transform: Transform) {
        self.transform = input_transform;
        self.sprite.set_transform(&self.transform);
    }
}

impl GameObject for Ocean {
    fn update(&mut self, _ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {

        let _factor = _ctx.get_camera().get_scale() * _ctx.get_width() as f32 / 1600 as f32;
        let mut transform = Transform::new();
        let mut translation = _ctx.screen_to_world((_ctx.get_width()/2) as i32, (_ctx.get_height()/2) as i32);
        translation.y = self.transform.get_translation().y - self.change*dt;
        transform.set_translation(translation);
        transform.set_scale(_factor);
        self.set_transform(transform);

        if(self.sprite.get_position().y < 365.0) {
            event_mailbox.submit_event(
                EventType::BoatSunk,
                EventReceiver::Scene
            );
        }

        return true;
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
        match event {
            EventType::OceanRiseRate{rate} => {
                self.change = rate*42.0;
            },
            _ => {} 
        }
       true
    }
}

impl PhysicalObject for Ocean {
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

    fn get_src_mask(&self) -> u32 { 1 }

    fn get_dst_mask(&self) -> u32 { 1 }
}
