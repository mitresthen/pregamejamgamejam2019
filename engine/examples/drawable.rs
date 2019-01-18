extern crate engine;

use engine::prelude::*;


struct ExampleGame {
    sprite: AnimatedSprite
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

        Ok(ExampleGame { sprite: sprite })
    }

    fn update(&mut self, ctx: &mut Engine) -> Result<bool, Error> {
        self.sprite.next_frame();
        ctx.draw(&self.sprite);
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
