use offset::Offset;
use vector::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Extent {
    pub width: i32,
    pub height: i32
}

impl Extent {
    pub fn new(width: i32, height: i32) -> Extent {
        Extent { width, height }
    }

    pub fn to_vec(self) -> Vec2 {
        Vec2::from_coords(self.width as f32, self.height as f32)
    }
}

impl From<Offset> for Extent {
    fn from(offset: Offset) -> Extent {
        Extent {
            width: offset.x,
            height: offset.y
        }
    }
}
