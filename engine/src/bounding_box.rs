use vector::Vec2;
use std::vec::Vec;
use std::ops::Range;

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

    pub fn get_edges(&self) -> Vec<Vec2> {
        let points = self.get_points();
        let mut edges = Vec::new();
        for i in 0..points.len() {
            let a = points[i];
            let b = points[(i+1) % points.len()];
            let ab = b-a;
            let ab_a = ab;
            edges.push(ab);
            //println!("Creating edge {:#?}", ab_a);
        }

        edges

    }

    pub fn sat_overlap(&self, other: BoundingBox) -> Option<Vec2> {
        let my_edges = self.get_edges();
        let other_edges = other.get_edges();
        let mut all_axes: Vec<Vec2> = Vec::new();
        all_axes.extend(&my_edges);
        all_axes.extend(&other_edges);
        fn update_range(existing_range: Range<f32>, inserted_value: f32) -> Range<f32> {
            let new_range = 
                Range{
                    start: existing_range.start.min(inserted_value),
                    end: existing_range.end.max(inserted_value) 
                };
            new_range
        }

        fn overlap_len(range_a: Range<f32>, range_b: Range<f32>) -> f32 {
            range_a.end.min(range_b.end) - range_a.start.max(range_b.start)
        }

        use std::f32;

        let mut min_overlap = f32::MAX;
        let mut axis_of_overlap = None;
        for axis in all_axes {
            let normal = axis.normal_vector();
            let mut a_range = Range{
                start: f32::MAX, 
                end: f32::MIN};
            let mut b_range = Range{
                start: f32::MAX, 
                end: f32::MIN};
            for a_point in self.get_points() {
                let dot_product = a_point.dot_product(normal);
                a_range = update_range(a_range, dot_product);
            }
            for b_point in other.get_points() {
                let dot_product = b_point.dot_product(normal);
                b_range = update_range(b_range, dot_product);
            }
            let current_overlap = overlap_len(a_range, b_range);
            if current_overlap < 0.0 {
                return None
            }
            if current_overlap <= min_overlap 
            {
                min_overlap = current_overlap;
                axis_of_overlap = Some(normal);
            }
        }
        axis_of_overlap
    }
}
