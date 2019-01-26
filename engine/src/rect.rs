use vector::Vec2;

pub struct Rect2D {
    pub min: Vec2,
    pub max: Vec2
}

impl Rect2D {
    pub fn new(min: Vec2, max: Vec2) -> Rect2D {
        Rect2D { min, max }
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

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn is_clicked(&self, v: Vec2) -> bool{
        v.x > self.min.x &&
        v.y > self.min.y &&
        v.x < self.max.x &&
        v.y < self.max.y
    }
}
