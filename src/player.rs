use engine::prelude::*;

pub struct Player {
    controller: AxisController,
    sprite: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    direction: i32,
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
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                direction: 1,
            };

        player.transform.set_scale(1.0);

        Ok(player)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for Player {

    fn update(&mut self, ctx: &Engine, dt: f32) -> bool {
        let target_velocity =
            self.controller.poll(ctx) * 240.0;

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

        self.sprite.set_mode(mode);
        self.sprite.set_transform(&self.transform);
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

