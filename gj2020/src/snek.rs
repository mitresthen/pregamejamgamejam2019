use engine::prelude::*;


pub struct Snek {
    controller: AxisController,
    interact_trigger: Trigger,
    sprite: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    direction: i32,
    collision_size: Vec2,
    just_colided: i32,
    just_jumped: i32,
    left_jumps: i32,
}

impl Snek {
    pub fn new(ctx: &mut Engine) -> Result<Snek, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/images/Snek/snek.png")?;

        let walk_texture = texture.sub_texture(Offset::from_coords(240, 0), Extent::new(240 * 2, 240 * 4))?;
        let walk_sprite = AnimatedSprite::new(Extent::new(240, 240), walk_texture)?;

        let idle_texture = texture.sub_texture(Offset::from_coords(0, 0), Extent::new(240 * 1, 240 * 4))?;
        let idle_sprite = AnimatedSprite::new(Extent::new(240, 240), idle_texture)?;

        let mut sprite = AggregatedAnimatedSprite::new();
        sprite.add(idle_sprite);
        sprite.add(walk_sprite);

        let snek =
            Snek {
                controller: AxisController::new(
                    Keycode::Up,
                    Keycode::Down,
                    Keycode::Left,
                    Keycode::Right
                ),
                interact_trigger: Trigger::new(Keycode::Space),
                sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                collision_size: Vec2::from_coords(200.0, 200.0),
                direction: 0,
                just_colided: 0,
                just_jumped: 0,
                left_jumps: 2,
            };

        Ok(snek)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position);
    }
}

impl GameObject for Snek {
    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        if self.just_jumped != 0
        {
            self.just_jumped -= 1;
        }
        let controller_input = self.controller.poll(ctx);
        let y_val = controller_input.y;

        let gravity_force = Vec2 { x: 0.0, y: 0.5 };

        let preserved_y = self.velocity.y;
        self.velocity.approach(controller_input * 400.0, 400.0 * dt);
        self.velocity.y = preserved_y;

        self.velocity = self.velocity + (gravity_force * (400.0 * dt));
        if self.velocity.y >= 0.5 && y_val < 0.0
        {
            if self.just_colided > 0
            {
                self.just_colided -= 1;
            } else if self.left_jumps > 0 && self.just_jumped == 0 {
                self.left_jumps -= 1;
                self.just_jumped += 1;
                self.velocity.y = y_val * 500.0;
            }
        }

        let is_walking =
            if controller_input.len() > 0.1 {
                self.direction =
                    if controller_input.x.abs() > controller_input.y.abs() {
                        if controller_input.x > 0.0 { 1 } else { 3 }
                    } else {
                        if controller_input.y > 0.0 { 2 } else { 0 }
                    };
                true
            } else {
                false
            };

        let mode = self.direction + if is_walking { 4 } else { 0 };

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
        self.sprite.step_time(dt * self.velocity.len() * 0.002);

        if self.interact_trigger.poll(ctx) {

            println!("Submitting interact event");
            event_mailbox.submit_event(
                EventType::Interact,
                EventReceiver::Nearby {
                    origin: self.transform.get_translation(),
                    max_distance: Some(360.0)
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
                self.just_colided = 32;
                self.velocity.x = self.velocity.x + force.x * 150.0;
                self.velocity.y = if self.velocity.y <= 0.0
                {
                    self.just_colided = 0;
                    self.left_jumps = 2;
                    0.0
                } else {
                    self.velocity.y
                };
                true
            }
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for Snek {
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

    fn get_bounding_box(&self) -> Option<Box<dyn CollisionShape>> {
        let rect = Rect2D::centered_rectangle(self.collision_size);
        let square = SquareShape::from_aabb(rect + self.transform.get_translation());

        Some(Box::new(square))
    }
}
