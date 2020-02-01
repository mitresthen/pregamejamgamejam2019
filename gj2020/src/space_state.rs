use engine::prelude::*;
use celestial_body::*;
use engine::game_object::EventQueue;

pub struct SpaceState {
    bodies: Vec::<CelestialBody>,
    event_queue: EventQueue,
}

impl SpaceState {
    pub fn new(_ctx: &mut Engine) -> Result<SpaceState, Error> {
        let mut bodies = Vec::<CelestialBody>::new();
        let tr = _ctx.get_texture_registry();
        let sun_sprite = StaticSprite::new(480, 480, tr.load("assets/images/Planets/Sun.png")?)?;
        let planet_sprite = StaticSprite::new(240, 240, tr.load("assets/images/Planets/Astroid.png")?)?;
        let planet2_sprite = StaticSprite::new(240, 240, tr.load("assets/images/Planets/Dino.png")?)?;
        let mut sun = CelestialBody::new(_ctx, sun_sprite, 30.0, 1.0, Vec2 { x: 0.0, y: 0.0})?;
        let mut planet = CelestialBody::new(_ctx, planet_sprite, 1.0, 1.0, Vec2 { x: 2000.0, y: 0.0})?;
        let mut planet2 = CelestialBody::new(_ctx, planet2_sprite, 0.4, 0.7, Vec2 { x: 0.0, y: 1200.0})?;
        planet.init_orbit(&mut sun, 1.0, false);
        planet2.init_orbit(&mut sun, 2.0, true);
        bodies.push(sun);
        bodies.push(planet);
        bodies.push(planet2);
        let state = SpaceState {
            bodies: bodies,
            event_queue: EventQueue::new(),
        };
        return Ok(state);
    }
}

impl GameState for SpaceState {
    fn update(mut self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        let mut physics = Vec::<CelestialBodyPhysics>::new();
        for body in &self.bodies {
            physics.push(body.get_physics());
        }
        for body in &mut self.bodies {
            body.gravitate(&physics, _dt);
            body.update(_ctx, &mut self.event_queue, _dt);
        }
        Ok(self)
    }
    fn draw(&mut self, _engine: &mut Engine, _dt: f32) -> Result<(), Error> {
        let mut _ctx = _engine.get_draw_context();
        for body in &self.bodies {
            body.render(&mut _ctx);
        }
        _engine.set_camera_zoom(10.0);
        Ok(())
    }
}
