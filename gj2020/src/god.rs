use engine::prelude::*;


pub struct God {
    controller: AxisController,
    interact_trigger: Trigger,
    sprite: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    direction: i32,
    collision_size: Vec2,
}

impl God {
    pub fn new(ctx: &mut Engine) -> Result<God, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/images/God/god.png")?;

        let walk_texture = texture.sub_texture(Offset::from_coords(240, 0), Extent::new(240 * 2, 480 * 4))?;
        let walk_sprite = AnimatedSprite::new(Extent::new(240, 480), walk_texture)?;

        let idle_texture = texture.sub_texture(Offset::from_coords(0, 0), Extent::new(240 * 1, 480 * 4))?;
        let idle_sprite = AnimatedSprite::new(Extent::new(240, 480), idle_texture)?;

        let mut sprite = AggregatedAnimatedSprite::new();
        sprite.add(idle_sprite);
        sprite.add(walk_sprite);

        let god = 
            God {
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
                collision_size: Vec2::from_coords(200.0, 80.0),
                direction: 0,
            };

        Ok(god)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.transform.set_scale(scale);
    }
}

impl GameObject for God {
    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        let target_velocity = self.controller.poll(ctx) * 400.0;
        self.velocity.approach(target_velocity, 400.0 * dt);

        let is_walking =
            if target_velocity.len() > 0.1 {
                self.direction =
                    if target_velocity.x.abs() > target_velocity.y.abs() {
                        if target_velocity.x > 0.0 { 1 } else { 3 }
                    } else {
                        if target_velocity.y > 0.0 { 2 } else { 0 }
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
}

impl PhysicalObject for God {
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

    fn get_inv_mass(&self) -> f32 { 1.0 }

    fn get_bounding_box(&self) -> Option<Box<dyn CollisionShape>> {
        let rect = Rect2D::centered_rectangle(self.collision_size);
        let square = SquareShape::from_aabb(rect + self.transform.get_translation());

        Some(Box::new(square))
    }
}
