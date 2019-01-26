use vector::Vec2;
use std::vec::Vec;

use super::Error;

#[derive(Clone, Copy)]
pub struct BoundingBox {
    width: f32, 
    height: f32,
    pub centre: Vec2
}

impl BoundingBox {
    pub fn new(bb_width: f32, bb_height: f32, bb_centre: Vec2) -> Result<BoundingBox, Error> {
        let bb = 
            BoundingBox {
                width: bb_width,
                height: bb_height,
                centre: bb_centre
            };
        Ok(bb)
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

    pub fn SAT_overlap(&self, other: BoundingBox) -> bool {
        true
    }
}
