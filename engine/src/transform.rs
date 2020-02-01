use vector::Vec2;

#[derive(Clone, Debug)]
pub struct Transform {
    translation: Vec2,
    scale: f32,
    rotation: f32
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
        let scaled = p * self.scale;

        let cos = self.rotation.cos();
        let sin = self.rotation.sin();

        let rotated =
            Vec2 {
                x: (cos * scaled.x) + (sin * scaled.y),
                y: (sin * -scaled.x) + (cos * scaled.y),
            };

        rotated + self.translation
    }

    pub fn transform_point_inv(&self, p: Vec2) -> Vec2 {
        let translated = p - self.translation;

        let cos = self.rotation.cos();
        let sin = self.rotation.sin();

        let rotated =
            Vec2 {
                x: (cos * translated.x) + (sin * -translated.y),
                y: (sin * translated.x) + (cos * translated.y),
            };

        rotated * (1.0 / self.scale)
    }

    pub fn transform_vector(&self, v: Vec2) -> Vec2 {
        let scaled = v * self.scale;

        let cos = self.rotation.cos();
        let sin = self.rotation.sin();

        Vec2 {
            x: (cos * scaled.x) + (sin * scaled.y),
            y: (sin * -scaled.x) + (cos * scaled.y),
        }
    }

    pub fn set_translation(&mut self, p: Vec2) {
        self.translation = p;
    }

    pub fn get_translation(&self) -> Vec2 {
        self.translation
    }

    pub fn set_rotation(&mut self, r: f32) {
        self.rotation = r;
    }

    pub fn get_rotation(&self) -> f32 {
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
