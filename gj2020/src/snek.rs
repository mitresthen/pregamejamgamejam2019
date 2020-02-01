use engine::prelude::*;


pub struct Snek {
    controller: AxisController,
    sprite: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    direction: i32,
    collision_size: Vec2,
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
                sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                collision_size: Vec2::from_coords(200.0, 200.0),
                direction: 0,
            };

        Ok(snek)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position);
    }
}

impl GameObject for Snek {
    fn update(&mut self, ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        let controller_input = self.controller.poll(ctx);
        let y_val = controller_input.y;

        let gravity_force = Vec2 { x: 0.0, y: 0.5 };

        let preserved_y = self.velocity.y;
        self.velocity.approach(controller_input * 400.0, 400.0 * dt);
        self.velocity.y = preserved_y;

        self.velocity = self.velocity + (gravity_force * (200.0 * dt));
        if self.velocity.y >= 0.01 && y_val < -0.5
        {
            self.velocity.y = y_val * 400.0
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
        self.sprite.step_time(dt * self.velocity.len() * 0.02);

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
