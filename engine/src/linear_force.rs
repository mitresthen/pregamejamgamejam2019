use crate::scene::Force;
use crate::vector::Vec2;

pub struct LinearForce {
    direction: Vec2
}

impl LinearForce {
    pub fn new(direction: Vec2) -> LinearForce {
        LinearForce {
            direction
        }
    }
}

impl Force for LinearForce {
    fn calculate_force_on_object(&self, position: Vec2, inv_mass: f32) -> Vec2{
        if inv_mass > 0.0 {
            self.direction * (1.0 / inv_mass)
        } else {
            Vec2::from_coords(0.0, 0.0)
        }
    }
}
