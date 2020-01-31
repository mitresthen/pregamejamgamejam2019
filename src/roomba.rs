use engine::prelude::*;

use std::f32;
use rand::Rng;
use rand;

use audio_library::AudioLibrary;


#[derive(PartialEq)]
enum RoombaState {
    Random,
    Searching,
    Investigating(Vec2),
    Tracking(Vec2),
    Attacking
}

pub struct Roomba {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    mode: RoombaState,
    aggro: f32,
    suck: bool
}

impl Roomba {
    pub fn new(ctx: &mut Engine) -> Result<Roomba, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/image/Electronics_Roomba.png")?;

        let sprite = AnimatedSprite::new(Extent::new(120, 120), texture)?;

        let mut roomba =
            Roomba {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                mode: RoombaState::Random,
                aggro: -1.0,
                suck: false
            };

        let vel = Vec2::random()*250.0;
        roomba.velocity = vel;


        roomba.transform.set_scale(1.0);

        Ok(roomba)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for Roomba {

    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {

        if self.mode == RoombaState::Searching {
            event_mailbox.submit_event(
                EventType::Probe { hint: "player".to_string() },
                EventReceiver::Broadcast
            );
            self.mode = RoombaState::Random;
        }

        if self.mode == RoombaState::Attacking {
            let origin = self.transform.get_translation();

            event_mailbox.submit_event(
                EventType::Attack { damage: 100.0 },
                EventReceiver::Nearest { origin, max_distance: Some(120.0) }
            );

            self.mode = RoombaState::Searching;
        }

        if let RoombaState::Investigating(target) = self.mode {
            let origin = self.transform.get_translation();
            event_mailbox.submit_event(
                EventType::RayCast { origin, target },
                EventReceiver::Scene
            );
            self.mode = RoombaState::Random;
        }

        let aggro_tolerance = 0.5;

        if let RoombaState::Tracking(target) = self.mode {
            self.aggro += dt;

            if self.aggro > aggro_tolerance && self.aggro - dt <= aggro_tolerance {
                ctx.play_sound(AudioLibrary::HooverStart).unwrap();
            }

            let distance = target - self.transform.get_translation();
            if distance.len() < 60.0 {
                self.mode = RoombaState::Searching;
            } else {
                let direction = distance.normalize();
                let target_velocity = direction * 340.0;
                self.velocity.approach(target_velocity, 340.0 * dt);
            }
        }
        if self.suck {
            println!("Roomba attempting to suck");
            let origin = self.transform.get_translation();
            event_mailbox.submit_event(
                EventType::Suck,
                EventReceiver::Nearby { origin, max_distance: Some(120.0) }
            );
            self.suck = false;
        }

        if self.mode == RoombaState::Random {
            self.aggro -= dt;

            if self.aggro < -aggro_tolerance && self.aggro + dt >= -aggro_tolerance {
                ctx.play_sound(AudioLibrary::HooverStop).unwrap();
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

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> {
        Some(self)
    }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> {
        Some(self)
    }

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Collide { force } => {
                let mut rng = rand::thread_rng();
                let angle: f32 = rng.gen();
                let angle = angle % f32::consts::PI;
                self.velocity = force.rotated(angle) * 150.0;
                self.suck = true;

                if let RoombaState::Tracking(_) = self.mode {
                    self.mode = RoombaState::Attacking;
                } else {
                    if self.aggro > 0.0 {
                        self.aggro = 0.0;
                    }
                    self.mode = RoombaState::Searching;
                }
                true
            },
            EventType::ProbeReply { p: position } => {
                self.mode = RoombaState::Investigating(position);
                true
            },
            EventType::RayCastReply { success, target } => {
                if success {
                    self.mode = RoombaState::Tracking(target);
                    if self.aggro < 0.0 {
                        self.aggro = 0.0;
                    }
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

