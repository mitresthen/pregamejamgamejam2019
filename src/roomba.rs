use engine::prelude::*;

enum MovementMode {
    Tracking,
    Random,
}

pub struct Roomba {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    mode: MovementMode
}

impl Roomba {
    pub fn new(ctx: &mut Engine) -> Result<Roomba, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/image/Electronics_Roomba.png")?;

        let mut sprite = AnimatedSprite::new(Extent::new(120, 120), texture)?;

        let mut roomba =
            Roomba {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                mode: MovementMode::Random
            };
        let vel = Vec2::random();
        println!("Setting roomba velocity {:#?}", vel);
        roomba.velocity = vel;


        roomba.transform.set_scale(1.0);

        Ok(roomba)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn new_random_direction(&mut self) {
        self.velocity = Vec2::random();
    }
}

impl GameObject for Roomba {

    fn update(&mut self, ctx: &Engine, dt: f32) -> bool {
        let target_velocity = Vec2::new();
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

    fn on_event(&mut self, event: GameEvent) {
        println!("Roomba handling event {:#?}", event);
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

