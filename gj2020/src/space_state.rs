use engine::prelude::*;
use celestial_body::*;
use smooth_transform::*;
use engine::game_object::EventQueue;

pub struct SpaceState {
    bodies: Vec::<CelestialBody>,
    fixed: Vec::<bool>,
    event_queue: EventQueue,
    camera: SmoothTransform,
    return_to_state: Option<Box<dyn GameState>>,
}

impl SpaceState {
    pub fn new(ctx: &mut Engine, return_to_state: Box<dyn GameState>) -> Result<Box<dyn GameState>, Error> {
        let mut bodies = Vec::<CelestialBody>::new();
        let tr = ctx.get_texture_registry();
        let mut sun_sprite = StaticSprite::new(480, 480, tr.load("assets/images/Planets/Sun.png")?)?;
        let mut planet_sprite = StaticSprite::new(240, 240, tr.load("assets/images/Planets/Swirly.png")?)?;
        let mut planet2_sprite = StaticSprite::new(240, 240, tr.load("assets/images/Planets/Earth.png")?)?;
        let mut planet3_sprite = StaticSprite::new(240, 240, tr.load("assets/images/Planets/Dino.png")?)?;
        sun_sprite.set_scale(2.0);
        planet_sprite.set_scale(2.0);
        planet2_sprite.set_scale(1.0);
        planet3_sprite.set_scale(0.7);
        let mut sun = CelestialBody::new(ctx, sun_sprite, 30.0)?;
        let mut planet = CelestialBody::new(ctx, planet_sprite, 5.0)?;
        let mut planet2 = CelestialBody::new(ctx, planet2_sprite, 1.0)?;
        let mut planet3 = CelestialBody::new(ctx, planet3_sprite, 0.3)?;
        planet2.init_orbit(&mut sun, 1.7, true, Some(Polar2::deg(2500.0, 180.0)));
        planet3.init_orbit(&mut sun, 0.8, false, Some(Polar2::deg(3000.0, 270.0)));
        planet.init_orbit(&mut planet2, 0.2, false, Some(Polar2::deg(500.0, 45.0)));
        bodies.push(sun);
        bodies.push(planet);
        bodies.push(planet2);
        bodies.push(planet3);
        let mut state = SpaceState {
            bodies: bodies,
            fixed: vec![false, false, false],
            event_queue: EventQueue::new(),
            camera: SmoothTransform::new(&Transform {
                translation: Vec2::new(),
                scale: 1.0,
                angle: 0.0,
            }, 2.0, 1.0),
            return_to_state: Some(return_to_state),
        };

        let state = MessageState::new(
                ctx,
                Box::new(state),
                Animation::PopInAndOut,
                ProceedMode::Click, 
                "assets/images/space_mission.png"
            )?;

        return Ok(state);
    }

    fn get_closest_body(&self, position: Vec2, max_dist: f32, skip_first: bool) -> Option<usize> {
        let mut min_distance = max_dist;
        let mut first: Option<usize> = None;
        let mut second: Option<usize> = None;
        for (i, body) in self.bodies.iter().enumerate() {
            let distance = (position - body.get_position()).len();
            if distance < min_distance {
                second = first;
                first = Some(i);
                min_distance = distance;
            }
        }
        match skip_first {
            false => first,
            true => second,
        }
    }
}

impl GameState for SpaceState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        if ctx.key_is_down(Keycode::Q) {
            ctx.reset_sound()?;
            let mut hub_state = Some(self.return_to_state.take().unwrap());
            let transition_state = TransitionState::new(self, move |_, _| Ok(hub_state.take().unwrap()));
            return Ok(Box::new(transition_state));
        }

        if self.fixed.iter().all(|x| *x) {
            ctx.reset_sound()?;
            let mut hub_state = Some(self.return_to_state.take().unwrap());
            let mut message_state = Some(MessageState::new(
                ctx,
                hub_state.take().unwrap(),
                Animation::PopInAndOut,
                ProceedMode::Click,
                "assets/images/space_enough.png"
            )?);
            let transition_state = TransitionState::new(self, move |_, _| Ok(message_state.take().unwrap()));
            return Ok(Box::new(transition_state));
        }

        let mut physics = Vec::<CelestialBodyPhysics>::new();
        for body in &self.bodies {
            physics.push(body.get_physics());
        }
        for body in &mut self.bodies {
            body.gravitate(&physics, dt);
            body.update(ctx, &mut self.event_queue, dt);
        }
        Ok(self)
    }

    fn draw(&mut self, engine: &mut Engine, dt: f32) -> Result<(), Error> {
        let mut ctx = engine.get_draw_context();
        let mut maxdist: f32 = 1.0;
        let origin = self.bodies[0].get_position();
        for body in &self.bodies {
            body.render(&mut ctx);
            maxdist = f32::max(maxdist, (origin - body.get_position()).len());
        }
        let mut camera = &mut self.camera;
        camera.set_pan_target(origin);
        camera.set_scale_target(maxdist / 333.0);
        camera.update(dt);
        engine.set_camera(camera.get());
        Ok(())
    }

    fn on_mouse_button_down(&mut self, ctx: &mut Engine, x: i32, y: i32, button: MouseButton) -> Result<(), Error>
    {
        use std::f32;
        let click_pos = ctx.get_mouse_position().position;
        let index = self.get_closest_body(click_pos, 500.0, false);
        let mut other_index: Option<usize> = None;
        match index {
            Some(i) => {
                println!("Clicked {:?}", index);
                let position = self.bodies[i].get_position();
                other_index = self.get_closest_body(position, f32::INFINITY, true);
            }
            None => println!("Clicked nothing ({:?})", click_pos)
        }
        match (index, other_index) {
            (Some(i), Some(j)) => {
                self.fixed[i-1] = true;
                let (mut body, mut other_body) = self.bodies.get_two_mut(i, j);
                body.stop();
                body.init_orbit(&mut other_body, 1.0, true, None);
            }
            (_, _) => ()
        }
        
        Ok(())
    }
}

use std::cmp::Ordering;
pub trait SliceExt {
    type Item;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item);
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item) {
        match index0.cmp(&index1) {
            Ordering::Less => {
                let mut iter = self.iter_mut();
                let item0 = iter.nth(index0).unwrap();
                let item1 = iter.nth(index1 - index0 - 1).unwrap();
                (item0, item1)
            }
            Ordering::Equal => panic!("[T]::get_two_mut(): received same index twice ({})", index0),
            Ordering::Greater => {
                let mut iter = self.iter_mut();
                let item1 = iter.nth(index1).unwrap();
                let item0 = iter.nth(index0 - index1 - 1).unwrap();
                (item0, item1)
            }
        }
    }
}
