use vector::Vec2;

use super::Error;

#[derive(Clone, Copy)]
pub struct BoundingBox {
    width: i32, 
    height: i32,
    pub centre: Vec2
}

impl BoundingBox {
    pub fn new(bb_width: i32, bb_height: i32, bb_centre: Vec2) -> Result<BoundingBox, Error> {
        let bb = 
            BoundingBox {
                width: bb_width,
                height: bb_height,
                centre: bb_centre
            };
        Ok(bb)
    }

    pub fn contains(&self, point: Vec2) -> bool {
        (point.x-self.centre.x).abs() <= (self.width/2) as f32 && (point.y - self.centre.y) <= (self.height/2) as f32
    }

    pub fn overlaps(&self, other: BoundingBox) -> bool {
        (self.centre.x - other.centre.x).abs() <= ((self.width/2) + (other.width/2)) as f32 &&
            (self.centre.y - other.centre.y).abs() <= ((self.height/2) + (other.height/2)) as f32
    }
}
