use vector::Vec2;

#[derive(Clone, Debug)]
pub struct Transform {
    pub translation: Vec2,
    pub scale: f32,
    pub angle: f32
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            translation: Vec2::new(),
            scale: 1.0,
            angle: 0.0
        }
    }

    pub fn transform_point(&self, p: Vec2) -> Vec2 {
        let scaled = p * self.scale;

        let cos = self.angle.cos();
        let sin = self.angle.sin();

        let rotated =
            Vec2 {
                x: (cos * scaled.x) + (sin * scaled.y),
                y: (sin * -scaled.x) + (cos * scaled.y),
            };

        rotated + self.translation
    }

    pub fn transform_point_inv(&self, p: Vec2) -> Vec2 {
        let translated = p - self.translation;

        let cos = self.angle.cos();
        let sin = self.angle.sin();

        let rotated =
            Vec2 {
                x: (cos * translated.x) + (sin * -translated.y),
                y: (sin * translated.x) + (cos * translated.y),
            };

        rotated * (1.0 / self.scale)
    }

    pub fn transform_vector(&self, v: Vec2) -> Vec2 {
        let scaled = v * self.scale;

        let cos = self.angle.cos();
        let sin = self.angle.sin();

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

    pub fn set_angle(&mut self, r: f32) {
        self.angle = r;
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn get_angle_mut(&mut self) -> &mut f32 {
        &mut self.angle
    }

    pub fn set_scale(&mut self, s: f32) {
        self.scale = s;
    }

    pub fn get_scale(&self) -> f32 {
        self.scale
    }

    pub fn translate(&mut self, p: Vec2) {
        self.translation += p;
    }

    pub fn interpolate(&self, other: &Transform, f: f32) -> Transform {
        let mut t = Transform::new();
        t.set_translation((self.translation * (1.0 - f)) + (other.translation * f));
        t.set_angle((self.angle * (1.0 - f)) + (other.angle * f));
        t.set_scale((self.scale * (1.0 - f)) + (other.scale * f));
        t
    }

}
