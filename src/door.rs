use engine::prelude::*;

use audio_library::AudioLibrary;
use engine::game_object::{Item, Items};


enum DoorState {
    ClosedAndLocked,
    ClosedAndUnlocked,
    RequestingKey(SceneObjectId),
    Unlocking,
    Open,
    Deleted,
}

pub struct Door {
    sprite: AnimatedSprite,
    lock_sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    state: DoorState
}

impl Door {
    pub fn new(ctx: &mut Engine, texture: Texture) -> Door {
        let lock_texture =
            {
                ctx.get_texture_registry().load("assets/images/item_keyhole.png").unwrap()
            };

        Door {
            sprite: AnimatedSprite::new(Extent::new(120, 360), texture).unwrap(),
            lock_sprite: AnimatedSprite::new(Extent::new(120, 120), lock_texture).unwrap(),
            transform: Transform::new(),
            velocity: Vec2::new(),
            state: DoorState::ClosedAndUnlocked
        }
    }

    pub fn with_key_requirement(mut self) -> Door {
        self.state = DoorState::ClosedAndLocked;
        self
    }
}

impl PhysicalObject for Door {
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

    fn get_collision_shape(&self) -> Option<Rc<dyn CollisionShape>> {
        let size = self.sprite.calculate_size() * 0.5;
        Some(Rc::new(SquareShape::from_aabb(Rect2D::centered_square(size.x) + self.transform.get_translation())))
    }

    fn get_inv_mass(&self) -> f32 { 0.0 }
}

impl GameObject for Door {
    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, _dt: f32) -> bool {
        match self.state {
            DoorState::Open => {
                event_mailbox.submit_event(
                    EventType::DeleteMe,
                    EventReceiver::Scene
                );

                ctx.play_sound(AudioLibrary::DoorOpen1).unwrap();

                self.state = DoorState::Deleted;
            },
            DoorState::RequestingKey(from_whom) => {
                event_mailbox.submit_event(
                    EventType::RequestItem { item: Item { item: Items::Key } },
                    EventReceiver::Addressed { object_id: from_whom }
                );
                self.state = DoorState::ClosedAndLocked
            },
            DoorState::Unlocking => {
                ctx.play_sound(AudioLibrary::MetallicHit).unwrap();
                self.state = DoorState::ClosedAndUnlocked;
            }
            _ => { }
        }

        let mut sprite_transform = self.transform.clone();
        sprite_transform.translate(Vec2::from_coords(0.0, -120.0));
        self.sprite.set_transform(&sprite_transform);
        self.lock_sprite.set_transform(&sprite_transform);
        true
    }

    fn render(&self, ctx: &mut DrawContext) {
        self.sprite.draw(ctx);

        match self.state {
            DoorState::ClosedAndLocked |
            DoorState::RequestingKey(_) => {
                self.lock_sprite.draw(ctx);
            },
            _ => {}
        }
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { Some(self) }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { Some(self) }

    fn on_event(&mut self, event: EventType, sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Interact => {
                if let DoorState::ClosedAndLocked = self.state {
                    self.state = DoorState::RequestingKey(sender.unwrap());
                } else if let DoorState::ClosedAndUnlocked = self.state {
                    self.state = DoorState::Open;
                }
                true
            },
            EventType::SendItem { item: Item { item: Items::Key} } => {
                self.state = DoorState::Unlocking;
                true
            },
            _ => {
                false
            }
        }
    }
}
