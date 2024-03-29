use engine::prelude::*;
use celestial_body::*;
use smooth_transform::*;
use engine::game_object::EventQueue;

pub struct SpaceState {
    background: StaticSprite,
    bodies: Vec::<CelestialBody>,
    fixed: Vec::<bool>,
    fixed_timer: f32,
    event_queue: EventQueue,
    camera: SmoothTransform,
    return_to_state: Option<Box<dyn GameState>>,
}

impl SpaceState {
    pub fn create(ctx: &mut Engine, return_to_state: Box<dyn GameState>) -> Result<Box<dyn GameState>, Error> {
        let mut bodies = Vec::<CelestialBody>::new();
        let tr = ctx.get_texture_registry();
        let background = StaticSprite::new(1200, 1200, tr.load("assets/images/starrySky.png")?)?;
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
        planet.init_orbit(&mut sun, 3.0, false, Some(Polar2::deg(1000.0, 270.0)));
        planet2.init_orbit(&mut sun, 0.2, true, Some(Polar2::deg(2500.0, 215.0)));
        planet3.init_orbit(&mut sun, 0.4, false, Some(Polar2::deg(3000.0, 320.0)));
        bodies.push(sun);
        bodies.push(planet);
        bodies.push(planet2);
        bodies.push(planet3);
        let state = SpaceState {
            bodies,
            fixed: vec![false, false, false],
            fixed_timer: 2.0,
            background,
            event_queue: EventQueue::new(),
            camera: SmoothTransform::new(&Transform {
                translation: Vec2::new(),
                scale: 1.0,
                angle: 0.0,
            }, 2.0, 1.0),
            return_to_state: Some(return_to_state),
        };

        let state = MessageState::create(
                ctx,
                Box::new(state),
                // Animation::PopInAndOut,
                ProceedMode::Click,
                "assets/images/space_mission.png"
            )?;

        Ok(state)
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

    fn get_most_forceful_body(&self, position: Vec2, skip_first: bool) -> Option<usize> {
        let mut max_force = 0.0;
        let mut first: Option<usize> = None;
        let mut second: Option<usize> = None;
        for (i, body) in self.bodies.iter().enumerate() {
            let force = body.get_force(position);
            if force > max_force {
                second = first;
                first = Some(i);
                max_force = force;
            }
        }
        match skip_first {
            false => first,
            true => second,
        }
    }

    fn draw_bg(&mut self, ctx: &mut Engine) {
        let camera = ctx.get_camera();
        let x_factor = ctx.get_width() as f32 / 1280_f32;
        let y_factor = ctx.get_height() as f32 / 720_f32;
        let mut _factor = 1.0;
        if x_factor < y_factor {
            _factor = x_factor;
        }
        else {
            _factor = y_factor;
        }
        self.background.set_position(ctx.screen_to_world((ctx.get_width()/2) as i32, (ctx.get_height()/2) as i32));
        self.background.set_scale(_factor * camera.get_scale());
        self.background.draw(&mut ctx.get_draw_context());
    }

    fn draw_fg(&mut self, engine: &mut Engine) {
        let ctx = &mut engine.get_draw_context();
        for body in &self.bodies {
            body.render(ctx);
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

        let mut dt = dt;
        if self.fixed.iter().all(|x| *x) {
            if self.fixed_timer <= 0.0 {
                ctx.reset_sound()?;
                let mut hub_state = Some(self.return_to_state.take().unwrap());
                let mut message_state = Some(MessageState::create(
                    ctx,
                    hub_state.take().unwrap(),
                    // Animation::PopInAndOut,
                    ProceedMode::Click,
                    "assets/images/space_enough.png"
                )?);
                let transition_state = TransitionState::new(self, move |_, _| Ok(message_state.take().unwrap()));
                return Ok(Box::new(transition_state));
            }
            self.fixed_timer -= dt;
            dt *= 10.0;
        }

        let mut maxdist: f32 = 1.0;
        let origin = self.bodies[0].get_position();
        for body in &self.bodies {
            maxdist = f32::max(maxdist, (origin - body.get_position()).len());
        }
        let camera = &mut self.camera;
        camera.set_pan_target(origin);
        camera.set_scale_target(maxdist / 333.0);
        camera.update(dt);
        ctx.set_camera(camera.get());

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

    fn draw(&mut self, engine: &mut Engine, _dt: f32) -> Result<(), Error> {
        self.draw_bg(engine);
        self.draw_fg(engine);

        Ok(())
    }

    fn on_mouse_button_down(&mut self, ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton) -> Result<(), Error>
    {
        let click_pos = ctx.get_mouse_position().position;
        let index = self.get_closest_body(click_pos, 500.0, false);
        let mut other_index: Option<usize> = None;
        match index {
            Some(i) => {
                println!("Clicked {:?}", index);
                let position = self.bodies[i].get_position();
                other_index = self.get_most_forceful_body(position, true);
            }
            None => println!("Clicked nothing ({:?})", click_pos)
        }

        if let (Some(i), Some(j)) = (index, other_index) {
            self.fixed[i-1] = true;
            let (body, other_body) = self.bodies.get_two_mut(i, j);
            body.stop();
            body.init_orbit(other_body, 1.0, true, None);
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
