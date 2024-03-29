use engine::prelude::*;

use std::f32;

use engine::game_object::Item;
use engine::game_object::Items;


pub struct Key {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    delete_me: bool,
    free_for_grabs: bool
}

impl Key {
    pub fn new(ctx: &mut Engine) -> Result<Key, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/images/item_Key.png")?;

        let sprite = AnimatedSprite::new(Extent::new(120, 120), texture)?;

        let mut key =
            Key {
                sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                delete_me: false,
                free_for_grabs: false
            };
        key.transform.set_scale(1.0);

        Ok(key)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for Key {

    fn update(&mut self, _ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        if self.delete_me {
            event_mailbox.submit_event(
                    EventType::Loot { item: Item{
                        item: Items::Key
                    }},
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

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> {
        Some(self)
    }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> {
        Some(self)
    }

    fn on_event(&mut self, event: EventType, _: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Interact => {
                if self.free_for_grabs{
                    println!("Free key grabbed");
                    self.delete_me = true;
                    true
                }
                else
                {
                    println!("Attempted grabbing dusty key");
                    false
                }
            },
            EventType::FreeFromDust => {
                println!("Key became free from dust");
                self.free_for_grabs = true;
                true
            },
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for Key {
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

    fn should_block(&self) -> bool { false }
}
