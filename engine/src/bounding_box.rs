use vector::Vec2;
use std::vec::Vec;
use std::f32;


use sat_collider::sat_overlap;

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    width: f32, 
    height: f32,
    pub centre: Vec2
}

impl BoundingBox {
    pub fn new(bb_width: f32, bb_height: f32, bb_centre: Vec2) -> BoundingBox {
        BoundingBox {
            width: bb_width,
            height: bb_height,
            centre: bb_centre
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        (point.x-self.centre.x).abs() <= (self.width/2.0) as f32 && (point.y - self.centre.y) <= (self.height/2.0) as f32
    }

    pub fn overlaps(&self, other: BoundingBox) -> bool {
        (self.centre.x - other.centre.x).abs() <= ((self.width/2.0) + (other.width/2.0)) as f32 &&
            (self.centre.y - other.centre.y).abs() <= ((self.height/2.0) + (other.height/2.0)) as f32
    }

    pub fn get_points(&self) -> Vec<Vec2> {
        let ul = Vec2 { x: self.centre.x - self.width/2.0, y: self.centre.y - self.height/2.0 };
        let ur = Vec2 { x: self.centre.x + self.width/2.0, y: self.centre.y - self.height/2.0 };
        let ll = Vec2 { x: self.centre.x - self.width/2.0, y: self.centre.y + self.height/2.0 };
        let lr = Vec2 { x: self.centre.x + self.width/2.0, y: self.centre.y + self.height/2.0 };
        let mut points = Vec::new();
        points.push(ul);
        points.push(ur);
        points.push(lr);
        points.push(ll);
        points
    }

    pub fn get_edges(&self) -> Vec<Vec2> {
        let points = self.get_points();
        let mut edges = Vec::new();
        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i+1) % points.len()];
            let ab = b-a;
            edges.push(ab);
        }

        edges

    }

    pub fn get_upper_left(&self) -> Vec2 {
        Vec2 {
            x: self.centre.x - self.width / 2.0,
            y: self.centre.y - self.height / 2.0
        }
    }

    pub fn get_lower_right(&self) -> Vec2 {
        Vec2 {
            x: self.centre.x + self.width / 2.0,
            y: self.centre.y + self.height / 2.0
        }
    }

    pub fn sat_overlap(&self, other: BoundingBox) -> Option<(Vec2, f32)> {
        sat_overlap(&self.get_points(), &other.get_points())
    }

    pub fn sat_overlap_points(&self, other_points: &Vec<Vec2>) -> Option<(Vec2, f32)> {
        sat_overlap(&self.get_points(), &other_points)
    }
}
