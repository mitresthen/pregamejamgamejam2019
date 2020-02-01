use engine::prelude::*;

use std::f32;

use AudioLibrary;


pub struct FuseBox {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    active: bool,
    audio_channel: usize,
}

impl FuseBox {
    pub fn new(ctx: &mut Engine) -> Result<FuseBox, Error> {
        let sprite;
        {
            let tr = ctx.get_texture_registry();
            let texture_on = tr.load("assets/images/wallTile_Blue_fuseBox_ON.png")?;
            sprite = AnimatedSprite::new(Extent::new(120, 360), texture_on)?;
        }
        let channel = ctx.prepare_sound(AudioLibrary::Victory)?;

        let mut fuse_box =
            FuseBox {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                active: true,
                audio_channel: channel,
            };
        fuse_box.transform.set_scale(1.0);

        Ok(fuse_box)
    }

    pub fn toggle_texture(&mut self, ctx: &mut Engine) {
        let tr = ctx.get_texture_registry();

        println!("Toggeling texture to {:#?}", self.active);

        if self.active {
            let texture_on = tr.load("assets/images/wallTile_Blue_fuseBox_ON.png");
            let sprite = AnimatedSprite::new(Extent::new(120, 360), texture_on.unwrap());
            self.sprite = sprite.unwrap();
        }
        else{
            let texture_off = tr.load("assets/images/wallTile_Blue_fuseBox_OFF.png");
            let sprite = AnimatedSprite::new(Extent::new(120, 360), texture_off.unwrap());
            self.sprite = sprite.unwrap();
        }
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for FuseBox {

    fn update(&mut self, ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        if !self.active {
            self.toggle_texture(ctx);
            ctx.play(self.audio_channel);
            self.active = true;
        }

        let mut sprite_transform = self.transform.clone();
        sprite_transform.translate(Vec2::from_coords(0.0, -120.0));
        self.sprite.set_transform(&sprite_transform);
        self.sprite.step_time(dt * self.velocity.len() * 0.05);

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

