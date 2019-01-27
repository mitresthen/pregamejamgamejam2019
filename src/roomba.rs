use engine::prelude::*;

use std::f32;
use rand::Rng;
use rand;


#[derive(PartialEq)]
enum MovementMode {
    Random,
    Searching,
    Investigating(Vec2),
    Tracking(Vec2),
}

pub struct Roomba {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    mode: MovementMode,
}

impl Roomba {
    pub fn new(ctx: &mut Engine) -> Result<Roomba, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/image/Electronics_Roomba.png")?;

        let mut sprite = AnimatedSprite::new(Extent::new(120, 120), texture)?;

        let mut roomba =
            Roomba {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                mode: MovementMode::Random
            };
        let vel = (Vec2::random()*250.0);
        roomba.velocity = vel;


        roomba.transform.set_scale(1.0);

        Ok(roomba)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn new_random_direction(&mut self) {
        self.velocity = Vec2::random();
    }
}

impl GameObject for Roomba {

    fn update(&mut self, ctx: &Engine, event_mailbox: &mut EventMailbox, dt: f32) -> bool {

        if self.mode == MovementMode::Searching {
            event_mailbox.submit_event(
                EventType::Probe { hint: "player".to_string() },
                EventReceiver::Broadcast
            );
            self.mode = MovementMode::Random;
        }

        if let MovementMode::Investigating(target) = self.mode {
            let origin = self.transform.get_translation();
            event_mailbox.submit_event(
                EventType::RayCast { origin, target },
                EventReceiver::Scene
            );
            self.mode = MovementMode::Random;
        }

        if let MovementMode::Tracking(target) = self.mode {
            let distance = (target - self.transform.get_translation());
            if distance.len() < 60.0 {
                self.mode = MovementMode::Searching;
            } else {
                let direction = distance.normalize();
                let target_velocity = direction * 240.0;
                self.velocity.approach(target_velocity, 240.0 * dt);
            }
        }

        self.transform.translate(self.velocity * dt);
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
            EventType::Collide { force } => {
                let mut rng = rand::thread_rng();
                let angle: f32 = rng.gen();
                let angle = angle % f32::consts::PI;
                self.velocity = force.rotated(angle)*250.0;
                self.mode = MovementMode::Searching;
                true
            },
            EventType::ProbeReply { p: position } => {
                self.mode = MovementMode::Investigating(position);
                true
            },
            EventType::RayCastReply { success, target } => {
                if success {
                    self.mode = MovementMode::Tracking(target);
                }
                true
            }
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for Roomba {
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

