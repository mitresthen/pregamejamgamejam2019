use crate::prelude::*;

pub struct RayShape {
    points: [Vec2; 2],
    axis: [Vec2; 1],
}

impl RayShape {
    pub fn new(origin: Vec2, target: Vec2) -> RayShape {
        RayShape {
            points: [origin, target],
            axis: [(origin - target).normalize().perpendicular()]
        }
    }
}

impl CollisionShape for RayShape {
    fn get_points(&self) -> &[Vec2] { &self.points }

    fn get_axes(&self) -> &[Vec2] { &self.axis }
}

