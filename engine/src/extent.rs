use offset::Offset;

#[derive(Debug, Clone, Copy)]
pub struct Extent {
    pub width: i32,
    pub height: i32
}

impl Extent {
    pub fn new(width: i32, height: i32) -> Extent {
        Extent { width, height }
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
