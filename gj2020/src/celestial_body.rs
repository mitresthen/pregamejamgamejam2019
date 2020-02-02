use engine::prelude::*;
use audio_library::AudioLibrary;

//const GRAV_CONST : f64 = 6.67408e-11;
const GRAV_CONST : f64 = 1.0e2;
const MASS_SCALE : f64 = 1.0e6;

#[derive(Copy, Clone)]
pub struct CelestialBodyPhysics {
	mass: f64,
	position: Vec2,
}

pub struct CelestialBody {
	sprite: StaticSprite,
	position: Vec2,
	velocity: Vec2,
	mass: f64,
}

impl CelestialBody {
	pub fn new(ctx: &mut Engine, sprite: StaticSprite, mass: f64, scale: f32, position: Vec2) -> Result<CelestialBody, Error> {
		ctx.replace_sound(AudioLibrary::Space, 0, -1)?;
		let body = CelestialBody {
			position: position,
			velocity: Vec2::new(),
			sprite: sprite,
			mass: mass * MASS_SCALE,
		};
		Ok(body)
	}

	pub fn get_physics(&self) -> CelestialBodyPhysics {
		CelestialBodyPhysics {
			position: self.position,
			mass: self.mass,
		}
	}

	pub fn gravitate(&mut self, bodies: &Vec::<CelestialBodyPhysics>, dt: f32) {
		for body in bodies {
			if body.position != self.position {
				let self_to_body = body.position - self.position;
				let dist_sq = self_to_body.len_sq();
				let direction = self_to_body.normalize();
				let force = GRAV_CONST * (body.mass * self.mass) / dist_sq as f64;
				let vel = direction * (dt as f64 * force / self.mass) as f32;
				self.push(vel);
			}
		}
	}

	pub fn place(&mut self, position: Vec2) {
		self.position = position;
	}

	pub fn push(&mut self, impulse: Vec2) {
		self.velocity += impulse;
	}

	pub fn init_orbit(&mut self, other: &mut CelestialBody, eccentricity: f64, ccw: bool) {
		let self_ratio = self.mass / (self.mass + other.mass);
		let other_ratio = other.mass / (self.mass + other.mass);
		let barycenter = self.position + (other.position - self.position) * other_ratio as f32;
		let bary_to_self = self.position - barycenter;
		let bary_to_other = other.position - barycenter;
		let barymass_self = self.mass * self_ratio * self_ratio;
		let barymass_other = other.mass * other_ratio * other_ratio;
		let k = GRAV_CONST * 2.0 * eccentricity / (eccentricity + 1.0);
		let mut self_speed = (barymass_other * k / bary_to_self.len() as f64).sqrt();
		let mut other_speed = (barymass_self * k / bary_to_other.len() as f64).sqrt();
		if ccw {
			self_speed = -self_speed;
			other_speed = -other_speed;
		}
		self.push(bary_to_self.perpendicular().normalize() * self_speed as f32);
		other.push(bary_to_other.perpendicular().normalize() * other_speed as f32);
	}
}

impl GameObject for CelestialBody {
	fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
		self.position += self.velocity * dt;
		self.sprite.set_position(self.position);
		true
	}

	fn render(&self, _ctx: &mut DrawContext) {
        self.sprite.draw(_ctx);
    }
}
