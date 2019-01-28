use engine::prelude::*;

use audio_library::AudioLibrary;

use std::f32;


pub struct Alex {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    prompted_for_response: bool,
    sound_channel: usize,
}

impl Alex {
    pub fn new(ctx: &mut Engine) -> Result<Alex, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/image/Alexa_version1.png")?;

        let sprite = AnimatedSprite::new(Extent::new(120, 240), texture)?;

        let mut alex =
            Alex {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                prompted_for_response: false,
                sound_channel: 0,
            };
        alex.transform.set_scale(1.0);

        Ok(alex)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for Alex {

    fn update(&mut self, ctx: &mut Engine, _event_mailbox: &mut EventMailbox, dt: f32) -> bool {
        if self.prompted_for_response {
            let id = ctx.replace_sound(AudioLibrary::AccidentSong, self.sound_channel, 0).unwrap();
            ctx.play(id);
            self.prompted_for_response = false;
        }
        let mut sprite_transform = self.transform.clone();
        sprite_transform.translate(Vec2::from_coords(0.0, -60.0));
        self.sprite.set_transform(&sprite_transform);

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

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Interact => {
                self.prompted_for_response = true;
                true
            },
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for Alex {
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

