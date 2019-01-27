use engine::prelude::*;

use std::f32;
use rand::Rng;
use rand;

use engine::game_object::Item;
use engine::game_object::Items;


pub struct FuseBox {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    delete_me: bool,
    active: bool
}

impl FuseBox {
    pub fn new(ctx: &mut Engine) -> Result<FuseBox, Error> {
        let tr = ctx.get_texture_registry();
        let texture_on = tr.load("assets/image/wallTile_Blue_fuseBox_ON.png")?;
        let mut sprite = AnimatedSprite::new(Extent::new(120, 360), texture_on)?;

        let mut fuse_box =
            FuseBox {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                delete_me: false,
                active: true
            };
        fuse_box.transform.set_scale(1.0);

        Ok(fuse_box)
    }

    pub fn toggle_texture(&mut self, ctx: &mut Engine) {
        let tr = ctx.get_texture_registry();

        println!("Toggeling texture to {:#?}", self.active);

        if self.active {
            let texture_on = tr.load("assets/image/wallTile_Blue_fuseBox_ON.png");
            let mut sprite = AnimatedSprite::new(Extent::new(120, 360), texture_on.unwrap());
            self.sprite = sprite.unwrap();
        }
        else{
            let texture_off = tr.load("assets/image/wallTile_Blue_fuseBox_OFF.png");
            let mut sprite = AnimatedSprite::new(Extent::new(120, 360), texture_off.unwrap());
            self.sprite = sprite.unwrap();
        }
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for FuseBox {

    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut EventMailbox, dt: f32) -> bool {
        if !self.active {
            self.toggle_texture(ctx);
            self.active = true;
        }

        self.sprite.set_transform(&self.transform);
        self.sprite.step_time(dt * self.velocity.len() * 0.05);

        true
    }

    fn render(&self, ctx: &mut DrawContext) {
        self.sprite.draw(ctx)
    }

    fn get_physical_object(&self) -> Option<&PhysicalObject> {
        Some(self)
    }

    fn get_physical_object_mut(&mut self) -> Option<&mut PhysicalObject> {
        Some(self)
    }

    fn on_event(&mut self, event: EventType, sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Interact => {
                println!("Someone tried to switch off the fuse");
                self.active = false;
                true
            },
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for FuseBox {
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
        let size = self.sprite.calculate_size() * 0.5;

        let bounding_box =
            BoundingBox::new(
                size.x,
                size.y,
                self.transform.get_translation()
            );

        Some(bounding_box)
    }
}

