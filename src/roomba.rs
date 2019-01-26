use engine::prelude::*;

pub struct Roomba {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
}

impl Player {
    pub fn new(ctx: &mut Engine) -> Result<Player, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/image/red_rider.png")?;

        let mut sprite = AnimatedSprite::new(Extent::new(120, 240), texture)?;

        let mut roomba =
            Roomba {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new()
            };

        roomba.transform.set_scale(1.0);

        Ok(roomba)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for Roomba {

    fn update(&mut self, ctx: &Engine, dt: f32, target: Vec2) -> bool {
        let target_velocity = 240.0;
        self.velocity.approach(target_velocity, 240.0 * dt);
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

