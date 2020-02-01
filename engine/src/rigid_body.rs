use crate::prelude::*;

pub struct RigidBody {
    texture: Texture,
    transform: Transform,
    velocity: Vec2,
    inv_mass: f32,
    inv_inertia: f32,
    spin: f32,
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
            inv_inertia: 0.0,
            spin: 0.0,
            velocity: Vec2::from_coords(0.0, 0.0),
            bounding_box,
        }
    }

    pub fn set_mass(&mut self, mass: f32) {
        self.inv_mass = 1.0 / mass;
    }

    pub fn set_inertia(&mut self, inertia: f32) {
        self.inv_inertia = 1.0 / inertia;
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position);
    }

    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.transform.set_angle(angle);
    }

    pub fn set_spin(&mut self, spin: f32) {
        self.spin = spin;
    }
}

impl PhysicalObject for RigidBody {
    fn get_transform(&self) -> &Transform { &self.transform }

    fn get_transform_mut(&mut self) -> &mut Transform { &mut self.transform }

    fn get_velocity(&self) -> &Vec2 { &self.velocity }

    fn get_velocity_mut(&mut self) -> &mut Vec2 { &mut self.velocity }

    fn get_bounding_box(&self) -> Option<Box<dyn CollisionShape>> {
        let mut collision_shape = SquareShape::from_aabb(self.bounding_box);

        collision_shape.transform(&self.transform);

        Some(Box::new(collision_shape))
    }

    fn should_block(&self) -> bool { true }

    fn get_inv_mass(&self) -> f32 { self.inv_mass }

    fn get_rotatable(&self) -> Option<&dyn Rotatable> { Some(self) }

    fn get_rotatable_mut(&mut self) -> Option<&mut dyn Rotatable> { Some(self) }
}

impl Rotatable for RigidBody {
    fn get_spin(&self) -> f32 { self.spin }

    fn get_spin_mut(&mut self) -> &mut f32 { &mut self.spin }

    fn get_inv_inertia(&self) -> f32 { self.inv_inertia }
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
