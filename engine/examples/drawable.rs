extern crate engine;

use engine::prelude::*;
use std::vec::Vec;

pub struct ExampleGame{
    moving_objects: Vec<MovableObject>,
}

impl GameInterface for ExampleGame {
    fn get_title() -> &'static str {
        "ExampleGame"
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let filename = "assets/characters.png";
        let texture = ctx.get_texture_registry().load(filename)?;
        let mut sprite = AnimatedSprite::new(32, texture)?;
        sprite.set_scale(4);
        sprite.set_position(100, 100);

        ctx.play_sound("../src/resources/music/personal_space.wav")?;

        let mainchar = MovableObject::new(sprite, 400.0).unwrap();

        let mut game_objects: Vec<MovableObject> = Vec::new();
        game_objects.push(mainchar);

        let game =
            ExampleGame 
            {
                moving_objects: game_objects,
            };

        Ok(game)
    }

    fn update(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {

        for object in self.moving_objects.iter_mut() {
            
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

            object.set_target_velocity(new_speed);
            object.update(dt);

            ctx.draw(&object.animated_sprite);
        }

        Ok(true)
    }

    fn on_key_down(&mut self, keycode: Keycode) -> Result<bool, Error> {
        if keycode == Keycode::Escape {
            return Ok(false);
        }


        Ok(true)
    }
}

fn main() {
    Engine::execute::<ExampleGame>(640, 480).unwrap();
}
