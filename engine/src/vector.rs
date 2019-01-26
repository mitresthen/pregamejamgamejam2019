use std::ops::{Add, Sub, Mul};

#[derive(Clone, Debug, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn len(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    pub fn from_coords(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn approach(&mut self, target: Vec2, acceleration: f32) {
        let diff = target - *self;
        let l = diff.len();

        if l < acceleration {
            self.x = target.x;
            self.y = target.y;
        } else {
            *self = *self + (diff * (acceleration / l));
        }
    }

    pub fn round(&self) -> Vec2 {
        Vec2 { x: self.x.round(), y: self.y.round() }
    }

    pub fn dot_product(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y + other.y
    }

    pub fn normal_vector(&self) -> Vec2 {
        let mut norm = 
            Vec2 {
                x: self.y * -1.0,
                y: self.x
            };
        let scaling_for_unit = norm.len()/self.len();
        norm.x *= scaling_for_unit;
        norm.y *= scaling_for_unit;
        norm
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }
}

