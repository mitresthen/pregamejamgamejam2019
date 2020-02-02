use vector::Vec2;

#[derive(Clone, Copy)]
pub struct Rect2D {
    pub min: Vec2,
    pub max: Vec2
}

impl Rect2D {
    pub fn new(min: Vec2, max: Vec2) -> Rect2D {
        Rect2D { min, max }
    }

    pub fn centered_square(size: f32) -> Rect2D {
        Rect2D {
            min: Vec2::from_coords(-size, -size) * 0.5,
            max: Vec2::from_coords(size, size) * 0.5,
        }
    }

    pub fn centered_rectangle(size: Vec2) -> Rect2D {
        Rect2D {
            min: Vec2::from_coords(-size.x, -size.y) * 0.5,
            max: Vec2::from_coords(size.x, size.y) * 0.5,
        }
    }

    pub fn empty() -> Rect2D {
        use std::f32;
        Rect2D {
            min: Vec2 { x: f32::MAX, y: f32::MAX },
            max: Vec2 { x: f32::MIN, y: f32::MIN }
        }
    }

    pub fn expand(&mut self, p: Vec2) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);

        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
    }

    pub fn wrap(&self, v: Vec2) -> Vec2 {
        let mut result = v;
        let size = self.max - self.min;

        while result.x > self.max.x { result.x -= size.x }
        while result.x < self.min.x { result.x += size.x }
        while result.y > self.max.y { result.y -= size.y }
        while result.y < self.min.y { result.y += size.y }

        result
    }

    pub fn contains(&self, v: Vec2) -> bool {
        (v.x >= self.min.x) &&
        (v.x <= self.max.x) &&
        (v.y >= self.min.y) &&
        (v.y <= self.max.y)
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn is_clicked(&self, v: Vec2) -> bool{
        v.x > self.min.x &&
        v.y > self.min.y &&
        v.x < self.max.x &&
        v.y < self.max.y
    }

    pub fn width(&self) -> f32 {
        return self.max.x - self.min.x;
    }

    pub fn height(&self) -> f32 {
        return self.max.y - self.min.y;
    }

    pub fn set_height(&mut self, height: f32) {
        self.max.y = height;
    }
}

impl std::ops::Add<Vec2> for Rect2D {
    type Output = Rect2D;
    fn add(self, other: Vec2) -> Rect2D {
        Rect2D {
            min: self.min + other,
            max: self.max + other,
        }
    }
}

impl std::ops::Mul<f32> for Rect2D {
    type Output = Rect2D;
    fn mul(self, other: f32) -> Rect2D {
        Rect2D {
            min: self.min * other,
            max: self.max * other,
        }
    }
}

impl std::ops::SubAssign<Vec2> for Rect2D {
    fn sub_assign(&mut self, other: Vec2){
        self.min -= other;
        self.max -= other;
    }
}

impl std::ops::AddAssign<Vec2> for Rect2D {
    fn add_assign(&mut self, other: Vec2){
        self.min += other;
        self.max += other;
    }
}

impl std::ops::MulAssign<f32> for Rect2D {
    fn mul_assign(&mut self, other: f32){
        self.min *= other;
        self.max *= other;
    }
}
