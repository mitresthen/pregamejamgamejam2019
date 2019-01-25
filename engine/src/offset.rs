use std::ops::Add;

use extent::Extent;

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub x: i32,
    pub y: i32
}

impl Offset {
    pub fn new() -> Offset {
        Offset { x: 0, y: 0 }
    }
}

impl Add<Extent> for Offset {
    type Output = Offset;

    fn add(self, other: Extent) -> Offset {
        Offset {
            x: self.x + other.width,
            y: self.y + other.height,
        }
    }
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, other: Offset) -> Offset {
        Offset {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
