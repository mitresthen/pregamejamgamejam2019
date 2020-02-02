use engine::prelude::*;

pub struct SmoothTransform {
	pan_speed: f32,
	scale_speed: f32,
	current: Transform,
	target: Transform,
}

impl SmoothTransform {
	pub fn new(transform: &Transform, pan_speed: f32, scale_speed: f32) -> SmoothTransform {
		SmoothTransform {
			current: transform.clone(),
			target: transform.clone(),
			pan_speed: pan_speed,
			scale_speed: scale_speed,
		}
	}

	pub fn set_pan_target(&mut self, target: Vec2) {
		self.target.set_translation(target);
	}

	pub fn set_scale_target(&mut self, scale: f32) {
		self.target.set_scale(scale);
	}

	pub fn set_speed(&mut self, pan: f32, scale: f32) {
		self.pan_speed = pan;
		self.scale_speed = scale;
	}

	pub fn update(&mut self, _dt: f32) {
		let mut b = self.pan_speed * _dt;
		if b > 1.0 {
			b = 1.0;
		}
		let a = 1.0 - b;
		self.current.set_translation(self.current.get_translation() * a + self.target.get_translation() * b);
		self.current.set_scale(self.current.get_scale() * a + self.target.get_scale() * b);
	}

	pub fn get(&self) -> Transform {
		self.current.clone()
	}
}
