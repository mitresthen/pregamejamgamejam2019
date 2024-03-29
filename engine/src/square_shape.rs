use crate::prelude::*;

pub struct SquareShape {
    points: [Vec2; 4],
    axes: [Vec2; 2],
}

impl SquareShape {
    pub fn from_aabb(rect: Rect2D) -> SquareShape {
        SquareShape {
            points: [
                Vec2::from_coords(rect.min.x, rect.min.y),
                Vec2::from_coords(rect.max.x, rect.min.y),
                Vec2::from_coords(rect.max.x, rect.max.y),
                Vec2::from_coords(rect.min.x, rect.max.y),
            ],
            axes: [
                Vec2::from_coords(1.0, 0.0),
                Vec2::from_coords(0.0, 1.0),
            ]
        }
    }
}

impl CollisionShape for SquareShape {
    fn get_points(&self) -> &[Vec2] { &self.points }

    fn get_axes(&self) -> &[Vec2] { &self.axes }
}
