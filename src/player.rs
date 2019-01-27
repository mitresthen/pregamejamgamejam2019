use engine::prelude::*;

use std::collections::HashSet;
use engine::game_object::{Item, Items};

use audio_library::AudioLibrary;

pub struct Player {
    controller: AxisController,
    interact_trigger: Trigger,
    inventory_trigger: Trigger,
    sprite: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    direction: i32,
    collision_size: f32,
    requesting_position: Vec<SceneObjectId>,
    items: HashSet<Item>,
    outgoing_events: Vec<(EventType, EventReceiver)>,
    nope: bool
}

impl Player {
    pub fn new(ctx: &mut Engine) -> Result<Player, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/image/mainChar-1x2.png")?;
        //let texture = tr.load("assets/image/red_rider.png")?;

        let walk_texture = texture.sub_texture(Offset::from_coords(120, 0), Extent::new(120 * 2, 240 * 4))?;
        let mut walk_sprite = AnimatedSprite::new(Extent::new(120, 240), walk_texture)?;

        let idle_texture = texture.sub_texture(Offset::from_coords(0, 0), Extent::new(120 * 1, 240 * 4))?;
        let mut idle_sprite = AnimatedSprite::new(Extent::new(120, 240), idle_texture)?;

        let mut sprite = AggregatedAnimatedSprite::new();
        sprite.add(idle_sprite);
        sprite.add(walk_sprite);

        let mut player =
            Player {
                controller: AxisController::new(
                    Keycode::Up,
                    Keycode::Down,
                    Keycode::Left,
                    Keycode::Right,
                ),
                interact_trigger: Trigger::new(Keycode::Space),
                inventory_trigger: Trigger::new(Keycode::I),
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                direction: 1,
                collision_size: 80.0,
                requesting_position: Vec::new(),
                items: HashSet::new(),
                outgoing_events: Vec::new(),
                nope: false
            };

        player.transform.set_scale(1.0);

        // Enable this line to test keys easily
        //player.items.insert(Item { item: Items::Key});

        Ok(player)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for Player {

    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut EventMailbox, dt: f32) -> bool {
        let target_velocity =
            self.controller.poll(ctx) * 400.0;

        for (event_type, event_receiver) in self.outgoing_events.drain(..) {
            event_mailbox.submit_event(event_type, event_receiver);
        }

        if self.nope {
            ctx.play_sound(AudioLibrary::Nope);
            self.nope = false;
        }

        if self.interact_trigger.poll(ctx) {
            
            println!("Submitting loot event");
            event_mailbox.submit_event(
                EventType::Interact,
                EventReceiver::Nearby {
                    origin: self.transform.get_translation(),
                    max_distance: Some(140.0)
                }
            );
        }

        if self.inventory_trigger.poll(ctx) {
            println!("inventory contains: {:#?}", self.items);


        }
        for object_id in self.requesting_position.drain(..) {
            let p = self.transform.get_translation();

            event_mailbox.submit_event(
                EventType::ProbeReply { p },
                EventReceiver::Addressed { object_id }
            )
        }

        self.velocity.approach(target_velocity, 400.0 * dt);
        self.transform.translate(self.velocity * dt);

        let mut is_walking = false;
        if target_velocity.len() > 0.1 {
            self.direction =
                if target_velocity.x.abs() > target_velocity.y.abs() {
                    if target_velocity.x > 0.0 { 1 } else { 3 }
                } else {
                    if target_velocity.y > 0.0 { 2 } else { 0 }
                };

            is_walking = true;
        }

        let mut mode = self.direction;

        if is_walking {
            mode += 4;
        }

        let mut sprite_transform = self.transform.clone();
        let collision_height = self.collision_size;
        let sprite_size = self.sprite.calculate_size();

        sprite_transform.translate(
            Vec2::from_coords(
                0.0,
                (sprite_size.y - collision_height) * -0.5
            )
        );


        self.sprite.set_mode(mode);
        self.sprite.set_transform(&sprite_transform);
        self.sprite.step_time(dt * self.velocity.len() * 0.02);

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
        println!("PLAYER: {:?}", event);
        match event {
            EventType::Probe { hint } => {
                if hint != "player" {
                    return false;
                }

                if let Some(s) = sender {
                    self.requesting_position.push(s);
                }
                true
            },
            EventType::Loot { item } => {
                self.items.insert(item);
                true
            },
            EventType::RequestItem { item: which_item  } => {
                if self.items.remove(&which_item) {
                    self.outgoing_events.push((
                        EventType::SendItem { item: which_item },
                        EventReceiver::Addressed { object_id: sender.unwrap() }
                    ));
                } else {
                    self.nope = true;
                };
                true
            }
            _ => { false }
        }
    }
}

impl PhysicalObject for Player {
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
        let bounding_box =
            BoundingBox::new(
                self.collision_size,
                self.collision_size,
                self.transform.get_translation()
            );

        Some(bounding_box)
    }
}

