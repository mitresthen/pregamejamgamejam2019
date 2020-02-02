use std::fmt;
use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};
use std::convert::{From, Into};

use rand::Rng;
use rand;

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2(x: {}, y:{})", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new() -> Vec2 {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn random() -> Vec2 {
        let mut created_vec = Vec2::new();
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen();
        let y: f32 = rng.gen();

        created_vec.x = x;
        created_vec.y = y;
        created_vec.normalize()
    }

    pub fn valid(&self) -> bool {
        use std::f32;
        self.x != f32::NAN && self.y != f32::NAN
    }

    pub fn len(&self) -> f32 {
        self.len_sq().sqrt()
    }

    pub fn len_sq(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn shifted(&self, x: f32, y: f32) -> Vec2 {
        Vec2 { x: self.x + x, y: self.y + y }
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

    pub fn rotated(&self, angle: f32) -> Vec2 {
        let new_x = self.x * angle.cos() - self.y * angle.sin();
        let new_y = self.x * angle.sin() + self.y * angle.cos();
        Vec2 {
            x: new_x,
            y: new_y
        }
    }

    pub fn round(&self) -> Vec2 {
        Vec2 { x: self.x.round(), y: self.y.round() }
    }

    pub fn dot_product(&self, other: Vec2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn normalize(&self) -> Vec2 {
        if self.len() == 0.0 {
            return Vec2::new()
        }
        self.clone() * (1.0 / self.len())
    }

    pub fn perpendicular(&self) -> Vec2 {
        Vec2 {
            x: self.y * -1.0,
            y: self.x
        }
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

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
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

impl Add<Polar2> for Vec2 {
    type Output = Vec2;
    fn add(self, other: Polar2) -> Vec2 {
        let v: Vec2 = other.into();
        self.add(v)
    }
}


const PI: f32 = 3.14159265359;
const TWO_PI: f32 = PI * 2.0;
const PI_BY_TWO: f32 = PI / 2.0;
const DEG_TO_RAD: f32 = TWO_PI / 360.0;
const RAD_TO_DEG: f32 = 360.0 / TWO_PI;

#[derive(Clone, Debug, Copy, PartialEq, Serialize, Deserialize)]
pub struct Polar2 {
    pub r: f32,
    pub t: f32
}

impl fmt::Display for Polar2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Polar2(r: {}, p:{})", self.t, self.t)
    }
}

impl Polar2 {
    pub fn new() -> Polar2 {
        Polar2 { r: 0.0, t: 0.0 }
    }

    pub fn deg(r: f32, d: f32) -> Polar2 {
        Polar2 {
            r: r,
            t: d * DEG_TO_RAD,
        }
    }

    pub fn random() -> Polar2 {
        Polar2 {
            r: 1.0,
            t: (rand::random::<f32>() % TWO_PI) - PI,
        }
    }

    pub fn valid(&self) -> bool {
        use std::f32;
        self.r != f32::NAN && self.t != f32::NAN
    }

    pub fn len(&self) -> f32 {
        self.r
    }

    pub fn len_sq(&self) -> f32 {
        self.r * self.r
    }

    pub fn rotated(&self, angle: f32) -> Polar2 {
        Polar2 {
            r: self.r,
            t: self.t + angle,
        }
    }

    pub fn normalize(&self) -> Polar2 {
        Polar2 {
            r: 1.0,
            t: self.t,
        }
    }

    pub fn perpendicular(&self) -> Polar2 {
        Polar2 {
            r: self.r,
            t: self.t + PI_BY_TWO,
        }
    }
}

impl From<Vec2> for Polar2 {
    fn from(v: Vec2) -> Polar2 {
        use std::f32;
        Polar2 {
            r: v.len(),
            t: f32::atan2(v.y, v.x),
        }
    }
}

impl Into<Vec2> for Polar2 {
    fn into(self) -> Vec2 {
        use std::f32;
        Vec2 {
            x: self.r * f32::sin(self.t),
            y: self.r * f32::cos(self.t),
        }
    }
}
