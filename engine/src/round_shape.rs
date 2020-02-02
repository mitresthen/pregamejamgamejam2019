use crate::prelude::*;

pub struct RoundShape {
    points: Vec<Vec2>,
    axes: Vec<Vec2>,
}

impl RoundShape {
    pub fn from_aabb(radius: f32, point_count: usize) -> RoundShape {
        let mut points = Vec::new();
        for i in 0..point_count {
            let angle = ((i as f32) / (point_count as f32)) * std::f32::consts::PI * 2.0;

            points.push(Vec2::from_coords(angle.cos(), angle.sin()) * radius);
        }

        let mut axes = Vec::new();
        for i in 0..point_count {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            let axis = (p1 - p2).normalize();
            axes.push(axis);
        }

        RoundShape { points, axes }
    }

    pub fn transform(&mut self, transform: &Transform) {
        for p in self.points.iter_mut() {
            *p = transform.transform_point(*p);
        }

        for a in self.axes.iter_mut() {
            *a = transform.transform_vector(*a).normalize();
        }
    }
}

impl CollisionShape for RoundShape {
    fn get_points(&self) -> &[Vec2] { &self.points }

    fn get_axes(&self) -> &[Vec2] { &self.axes }
}

