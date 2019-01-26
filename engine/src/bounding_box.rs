use vector::Vec2;
use std::vec::Vec;
use std::f32;

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    width: f32, 
    height: f32,
    pub centre: Vec2
}

#[derive(Clone, Copy, Debug)]
struct Range {
    min: f32,
    max: f32
}

impl Range {
    pub fn new() -> Range {
        Range {
            min: f32::MAX,
            max: f32::MIN
        }
    }

    pub fn update(&mut self, value: f32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }

    pub fn overlap(&self, other: &Range) -> Range {
        Range {
            min: self.min.max(other.min),
            max: self.max.min(other.max)
        }
    }

    pub fn length(&self) -> f32 {
        self.max - self.min
    }

    pub fn center(&self) -> f32 {
        (self.min + self.max) * 0.5
    }
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
            let ab_a = ab;
            edges.push(ab);
            //println!("Creating edge {:#?}", ab_a);
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
        let my_edges = self.get_edges();
        let other_edges = other.get_edges();
        let mut all_axes: Vec<Vec2> = Vec::new();
        all_axes.extend(&my_edges);
        all_axes.extend(&other_edges);

        use std::f32;

        let mut min_overlap = f32::MAX;
        let mut axis_of_overlap = None;
        for axis in all_axes {
            let normal = axis.perpendicular().normalize();

            let mut a_range = Range::new();
            let mut b_range = Range::new();
            for a_point in self.get_points() {
                let dot_product = a_point.dot_product(normal);
                a_range.update(dot_product);
            }
            for b_point in other.get_points() {
                let dot_product = b_point.dot_product(normal);
                b_range.update(dot_product);
            }

            let current_overlap = a_range.overlap(&b_range).length();
            if current_overlap < 0.0 {
                return None
            }
            if current_overlap <= min_overlap
            {
                min_overlap = current_overlap;

                let a_center = a_range.center();
                let b_center = b_range.center();

                let factor = if a_center < b_center { -1.0 } else { 1.0 };

                axis_of_overlap = Some((normal * factor, current_overlap));
            }
        }
        axis_of_overlap
    }
}
