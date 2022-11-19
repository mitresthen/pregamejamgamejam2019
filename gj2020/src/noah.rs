use std::rc::Rc;
use engine::prelude::*;

pub struct Noah {
    controller: AxisController,
    interact_trigger: Trigger,
    jump_trigger: Trigger,
    sprite: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    direction: i32,
    collision_size: Vec2,
    jump_timer: f32,
    shape: Rc<dyn CollisionShape>,
}

impl Noah {
    pub fn new(ctx: &mut Engine) -> Result<Noah, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/images/Noah/noah.png")?;

        let walk_texture = texture.sub_texture(Offset::from_coords(0, 0), Extent::new(240 * 3, 480 * 2))?;
        let walk_sprite = AnimatedSprite::new(Extent::new(240, 480), walk_texture)?;

        let mut sprite = AggregatedAnimatedSprite::new();
        sprite.add(walk_sprite);

        let collision_size = Vec2::from_coords(200.0, 80.0);
        let rect = Rect2D::centered_rectangle(collision_size);
        let square = BevelShape::from_aabb(rect, rect.width() / 3.0);

        let noah =
            Noah {
                controller: AxisController::new(
                    Keycode::Up,
                    Keycode::Down,
                    Keycode::Left,
                    Keycode::Right
                ),
                interact_trigger: Trigger::new(Keycode::Space),
                jump_trigger: Trigger::new(Keycode::J),
                sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                collision_size,
                direction: 0,
                jump_timer: 0.0,
                shape: Rc::new(square),
            };

        Ok(noah)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.transform.set_scale(scale);
    }
}

impl GameObject for Noah {
    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        let mut target_velocity = self.controller.poll(ctx) * 400.0;

        target_velocity.y = 0.0;

        self.jump_timer -= dt;

        let old_velocity_y = self.velocity.y;
        self.velocity.approach(target_velocity, 400.0 * dt);
        self.velocity.y = old_velocity_y;

        let is_walking =
            if target_velocity.len() > 0.1 {
                self.direction =
                    if target_velocity.x.abs() > target_velocity.y.abs() {
                        i32::from(target_velocity.x > 0.0)
                    } else {
                        1
                    };

                true
            } else {
                false
            };

        if self.jump_trigger.poll(ctx) && self.jump_timer > 0.0 {
            self.velocity.y -= 400.0;
            self.jump_timer = -1.0;
        }

        let mode = self.direction + if is_walking { 2 } else { 0 };

        let mut sprite_transform = self.transform.clone();
        let collision_height = self.collision_size.y;
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

        if self.interact_trigger.poll(ctx) {

            println!("Submitting interact event");
            event_mailbox.submit_event(
                EventType::Interact,
                EventReceiver::Nearby {
                    origin: self.transform.get_translation(),
                    max_distance: Some(240.0)
                }
            );
        }

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
                if force.y < -0.9 {
                    self.jump_timer = 0.01;
                }
                true
            },
            _ => { false }
        }
    }
}

impl PhysicalObject for Noah {
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

    fn get_friction(&self) -> f32 { 0.0 }

    fn get_inv_mass(&self) -> f32 { 5.0 }

    fn get_collision_shape(&self) -> Option<Rc<dyn CollisionShape>> {
        Some(self.shape.clone())
    }

    fn get_src_mask(&self) -> u32 { 1 }

    fn get_dst_mask(&self) -> u32 { 0 }
}
