use crate::scene::Force;
use crate::vector::Vec2;

pub struct RadialForce {
    position: Vec2,
    strength: f32,
}

impl RadialForce {
    pub fn new(position: Vec2, strength: f32) -> RadialForce {
        RadialForce {
            position,
            strength,
        }
    }
}

impl Force for RadialForce {
    fn calculate_force_on_object(&self, position: Vec2, inv_mass: f32) -> Vec2{
        let difference = self.position - position;
        let distance_sq = difference.len_sq();
        if distance_sq > 0.01 && inv_mass > 0.0 {
            let axis = difference.normalize();

            let amount = self.strength * (1.0 / (distance_sq * inv_mass));
            println!("Radial force = {}", amount);
            axis * amount
        } else {
            Vec2::new()
        }
    }
}

