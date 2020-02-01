use vector::Vec2;

#[derive(Clone, Debug)]
pub struct Transform {
    translation: Vec2,
    scale: f32,
    rotation: f64
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            translation: Vec2::new(),
            scale: 1.0,
            rotation: 0.0
        }
    }

    pub fn transform_point(&self, p: Vec2) -> Vec2 {
        (p * self.scale) + self.translation
    }

    pub fn transform_point_inv(&self, p: Vec2) -> Vec2 {
        (p - self.translation) * (1.0 / self.scale)
    }

    pub fn set_translation(&mut self, p: Vec2) {
        self.translation = p;
    }

    pub fn get_translation(&self) -> Vec2 {
        self.translation
    }

    pub fn set_rotation(&mut self, r: f64) {
        self.rotation = r;
    }

    pub fn get_rotation(&self) -> f64 {
        self.rotation
    }

    pub fn set_scale(&mut self, s: f32) {
        self.scale = s;
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn translate(&mut self, p: Vec2) {
        self.translation = self.translation + p;
    }

}
