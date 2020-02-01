use crate::prelude::*;

pub struct RigidBody {
    texture: Texture,
    transform: Transform,
    velocity: Vec2,
    inv_mass: f32,
    bounding_box: Rect2D
}

impl RigidBody {
    pub fn new(texture: Texture) -> RigidBody {
        let sx = texture.extent().width as f32 * 0.5;
        let sy = texture.extent().height as f32 * 0.5;

        let bounding_box =
            Rect2D {
                min: Vec2::from_coords(-sx, -sy),
                max: Vec2::from_coords(sx, sy)
            };

        RigidBody {
            texture,
            transform: Transform::new(),
            inv_mass: 0.0,
            velocity: Vec2::from_coords(0.0, 0.0),
            bounding_box,
        }
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.inv_mass = 1.0 / mass;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position);
    }
}

impl PhysicalObject for RigidBody {
    fn get_transform(&self) -> &Transform { &self.transform }

    fn get_transform_mut(&mut self) -> &mut Transform { &mut self.transform }

    fn get_velocity(&self) -> &Vec2 { &self.velocity }

    fn get_velocity_mut(&mut self) -> &mut Vec2 { &mut self.velocity }

    fn get_bounding_box(&self) -> Option<Box<dyn CollisionShape>> {
        let collision_shape = SquareShape::from_aabb(self.bounding_box + self.transform.get_translation());

        Some(Box::new(collision_shape))
    }

    fn should_block(&self) -> bool { false }

    fn get_inv_mass(&self) -> f32 { self.inv_mass }
}

impl GameObject for RigidBody {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, _dt: f32) -> bool {
        true
    }

    fn render(&self, ctx: &mut DrawContext) {
        ctx.draw(&self.texture, &self.transform); 
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { Some(self) }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { Some(self) }
}
