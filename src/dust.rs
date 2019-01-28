use engine::prelude::*;

use std::f32;

pub struct Dust {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    delete_me: bool
}

impl Dust {
    pub fn new(ctx: &mut Engine) -> Result<Dust, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/image/item_Dust.png")?;

        let sprite = AnimatedSprite::new(Extent::new(120, 120), texture)?;

        let mut dust =
            Dust {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                delete_me: false,
            };
        dust.transform.set_scale(1.0);

        Ok(dust)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for Dust {

    fn update(&mut self, _ctx: &mut Engine, event_mailbox: &mut EventMailbox, dt: f32) -> bool {
        if self.delete_me {
            
            event_mailbox.submit_event(
                EventType::FreeFromDust,
                EventReceiver::Nearest {  
                    origin: self.transform.get_translation(),
                    max_distance: Some(120.0)
                }
            );

            event_mailbox.submit_event(
                EventType::DeleteMe,
                EventReceiver::Scene
            );

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

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Suck => {
                self.delete_me = true;
                true
            },
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for Dust {
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

    fn should_block(&self) -> bool { false }
}

