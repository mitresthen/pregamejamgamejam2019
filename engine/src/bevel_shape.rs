use crate::prelude::*;

use std::ops::Add;

pub struct BevelShape {
    points: [Vec2; 8],
    axes: [Vec2; 4],
}

impl BevelShape {
    pub fn from_aabb(rect: Rect2D, length: f32) -> BevelShape {
        let ll = Vec2::from_coords(rect.min.x, rect.min.y);
        let lr = Vec2::from_coords(rect.max.x, rect.min.y);
        let ur = Vec2::from_coords(rect.max.x, rect.max.y);
        let ul = Vec2::from_coords(rect.min.x, rect.max.y);

        let llu = ll.add(Vec2::from_coords(0.0, length));
        let llr = ll.add(Vec2::from_coords(length, 0.0));
        let lrl = lr.add(Vec2::from_coords(-length, 0.0));
        let lru = lr.add(Vec2::from_coords(0.0, length));
        let urd = ur.add(Vec2::from_coords(0.0, -length));
        let url = ur.add(Vec2::from_coords(-length, 0.0));
        let ulr = ul.add(Vec2::from_coords(length, 0.0));
        let uld = ul.add(Vec2::from_coords(0.0, -length));
        BevelShape {
            points: [
                llu,
                llr,
                lrl,
                lru,
                urd,
                url,
                ulr,
                uld
            ],
            axes: [
                Vec2::from_coords(1.0, 0.0),
                Vec2::from_coords(0.0, 1.0),
                Vec2::from_coords(1.0, 1.0).normalize(),
                Vec2::from_coords(1.0, -1.0).normalize()
            ]
        }
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

impl CollisionShape for BevelShape {
    fn get_points(&self) -> &[Vec2] { &self.points }

    fn get_axes(&self) -> &[Vec2] { &self.axes }
}
