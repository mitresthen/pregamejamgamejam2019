extern crate engine;

use engine::prelude::*;
use std::vec::Vec;

pub struct ExampleGame{
    player_object: MovableObject,
    autonomous_moving_objects: Vec<MovableObject>,
    pause_sprite: AnimatedSprite,
    title_screen: SplashScreen,
}

impl GameInterface for ExampleGame {
    fn get_title() -> &'static str {
        "ExampleGame"
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let title_background_filename = "assets/title_background.png";
        let title_background_texture = ctx.get_texture_registry().load(title_background_filename)?;
        let mut title_background = AnimatedSprite::new(128, title_background_texture)?;
        title_background.set_scale(4.0);
        title_background.set_position(ctx.get_screen_bounds().center());

        let title_filename = "assets/title.png";
        let title_texture = ctx.get_texture_registry().load(title_filename)?;
        let mut title_sprite = AnimatedSprite::new(128, title_texture)?;
        title_sprite.set_scale(1.0);
        title_sprite.set_position(ctx.get_screen_bounds().center());

        let title_screen =
            SplashScreen {
                background: title_background,
                foreground: title_sprite,
            };

        let filename = "assets/characters.png";
        let texture = ctx.get_texture_registry().load(filename)?;
        let mut sprite = AnimatedSprite::new(32, texture)?;
        sprite.set_scale(4.0);
        sprite.set_position(Vec2 { x: 100.0, y: 100.0 });

        let pause_filename = "assets/paused.png";
        let pause_texture = ctx.get_texture_registry().load(pause_filename)?;
        let mut pause_sprite = AnimatedSprite::new(128, pause_texture)?;
        pause_sprite.set_scale(1.0);
        pause_sprite.set_position(ctx.get_screen_bounds().center());

        // ctx.play_sound("../src/resources/music/personal_space.wav")?;

        let mainchar = MovableObject::new(sprite, 400.0).unwrap();

        let mut game_objects: Vec<MovableObject> = Vec::new();

        let roombatexture = ctx.get_texture_registry().load(filename)?;
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

    fn update(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        {
            if ctx.is_on_title_screen
            {
                ctx.draw(&self.title_screen);
                return Ok(true);
            }

            // Update player characted if game isn't paused
            if !ctx.paused
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

            ctx.draw(&self.player_object.animated_sprite);
        }

        for object in self.autonomous_moving_objects.iter_mut() {
            // Update other autonomous moving objects if game isn't paused
            if !ctx.paused
            {
                let player_pos = self.player_object.get_position();
                let speed = 300.0;
                let mut new_speed = Vec2::new();
                let direction = self.player_object.get_position() -object.get_position();
                let velocity_scaling= (direction.len()/speed).abs();
                let target_vel = direction*velocity_scaling;
                object.set_target_velocity(target_vel);
                object.update(dt);
            }

            ctx.draw(&object.animated_sprite);
        }

        for object in self.autonomous_moving_objects.iter_mut() {
            let overlap = object.overlaps(self.player_object.bounding_box);
            // println!("Overlap: {:?}", overlap);
        }

        // Draw paused sprite if game is paused
        if ctx.paused
        {
            ctx.draw(&self.pause_sprite);
        }

        Ok(true)
    }

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode) -> Result<bool, Error> {
        if keycode == Keycode::Escape {
            return Ok(false);
        }
        if keycode == Keycode::P {
            ctx.try_to_change_paused();
            return Ok(true);
        }
        if ctx.is_on_title_screen
        {
            ctx.end_title_screen();
            return Ok(true);
        }


        Ok(true)
    }
}

fn main() {
    Engine::execute::<ExampleGame>(640, 480).unwrap();
}
