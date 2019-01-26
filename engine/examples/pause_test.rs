extern crate engine;

use engine::prelude::*;
use std::vec::Vec;

pub struct ExampleGame{
    player_object: MovableObject,
    autonomous_moving_objects: Vec<MovableObject>,
    pause_sprite: StaticSprite,
    title_screen: SplashScreen,
}

impl GameInterface for ExampleGame {
    fn get_title() -> &'static str {
        "ExampleGame"
    }

    fn get_title_screen(&self) -> Option<SplashScreen> {
        Some(self.title_screen.clone())
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let tr = ctx.get_texture_registry();

        let mut title_background = StaticSprite::new(640, 480, tr.load("assets/title_background.png")?)?;

        let mut title_sprite = StaticSprite::new(128, 128, tr.load("assets/title.png")?)?;

        let title_screen =
            SplashScreen {
                background: title_background,
                foreground: title_sprite,
            };

        let mut pause_sprite = StaticSprite::new(128, 64, tr.load("assets/paused.png")?)?;

        let mut sprite = AnimatedSprite::new(32, tr.load("assets/characters.png")?)?;
        sprite.set_scale(4.0);
        sprite.set_position(Vec2::from_coords(100.0, 100.0));

        // ctx.play_sound("../src/resources/music/personal_space.wav")?;

        let mainchar = MovableObject::new(sprite, 400.0).unwrap();

        let mut game_objects: Vec<MovableObject> = Vec::new();

        let roombatexture = tr.load("assets/characters.png")?;
        let mut roombasprite = AnimatedSprite::new(32, roombatexture)?;
        roombasprite.set_scale(4.0);
        roombasprite.set_position(Vec2::from_coords(100.0, 100.0));

        let roomba = MovableObject::new(roombasprite, 420.0).unwrap();
        game_objects.push(roomba);

        let game =
            ExampleGame
            {
                player_object: mainchar,
                autonomous_moving_objects: game_objects,
                title_screen: title_screen,
                pause_sprite: pause_sprite,
            };

        Ok(game)
    }

    fn update_gameplay(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        {
            let speed = 400.0;
            let mut new_speed = Vec2::new();

            if ctx.key_is_down(Keycode::Up) {
                new_speed.y = -speed;
            }
            if ctx.key_is_down(Keycode::Down) {
                new_speed.y = speed;
            }
            if ctx.key_is_down(Keycode::Left) {
                new_speed.x = -speed;
            }
            if ctx.key_is_down(Keycode::Right) {
                new_speed.x = speed;
            }

            self.player_object.set_target_velocity(new_speed);

            self.player_object.update(dt);
        }

        for object in self.autonomous_moving_objects.iter_mut() {
            let player_pos = self.player_object.get_position();
            let speed = 300.0;
            let mut new_speed = Vec2::new();
            let direction = self.player_object.get_position() -object.get_position();
            let velocity_scaling= (direction.len()/speed).abs();
            let target_vel = direction*velocity_scaling;
            object.set_target_velocity(target_vel);
            object.update(dt);
        }

        for object in self.autonomous_moving_objects.iter_mut() {
            let overlap = object.overlaps(self.player_object.bounding_box);
            // println!("Overlap: {:?}", overlap);
        }

        Ok(true)
    }

    fn draw_gameplay(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        {
            ctx.draw(&self.player_object.animated_sprite);
            // println!("Drew char at {}x{}", &self.player_object.get_position().y, &self.player_object.get_position().y)
        }

        for object in self.autonomous_moving_objects.iter_mut() {
            ctx.draw(&object.animated_sprite);
        }

        Ok(true)
    }

    fn draw_main_menu(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        Ok(true)
    }

    fn draw_pause_menu(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        ctx.draw(&self.pause_sprite);

        Ok(true)
    }

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode, is_repeated: bool) -> Result<bool, Error> {
        if ctx.state.gameplay_displayed
        {
            if keycode == Keycode::P && !is_repeated {
                ctx.invert_paused_state();
                return Ok(true);
            }
        }
        if ctx.state.is_on(TITLE_STATE)
        {
            ctx.end_title_screen();
            return Ok(true);
        }


        Ok(true)
    }

    fn on_key_up(&mut self, ctx: &mut Engine, keycode: Keycode) -> Result<bool, Error> {
        self.on_key_down(ctx, keycode, true)
    }
}

fn main() {
    Engine::execute::<ExampleGame>(1920, 1680).unwrap();
}
