use vector::Vec2;
use std::vec::Vec;
use std::f32;

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

pub fn get_edges(points: &Vec<Vec2>) -> Vec<Vec2> {
    let mut edges = Vec::new();
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i+1) % points.len()];
        let ab = b-a;
        edges.push(ab);
    }
    edges
}

pub fn sat_overlap(figure_a: &Vec<Vec2> , figure_b: &Vec<Vec2>) -> Option<(Vec2, f32)> {
    let figure_a_edges = get_edges(&figure_a);
    let figure_b_edges = get_edges(&figure_b);

    let mut all_axes: Vec<Vec2> = Vec::new();
    all_axes.extend(&figure_a_edges);
    all_axes.extend(&figure_b_edges);
    use std::f32;
    let mut min_overlap = f32::MAX;
    let mut axis_of_overlap = None;
    for axis in all_axes {
        let normal = axis.perpendicular().normalize();
        let mut a_range = Range::new();
        let mut b_range = Range::new();
        for a_point in figure_a.iter() {
            let dot_product = a_point.dot_product(normal);
            a_range.update(dot_product);
        }
        for b_point in figure_b.iter() {
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
