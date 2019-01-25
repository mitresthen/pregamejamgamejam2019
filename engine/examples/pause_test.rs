extern crate engine;

use engine::prelude::*;

struct ExampleGame {
    sprite: AnimatedSprite,
    sprite2: AnimatedSprite,
    player_position: Vec2,
    player_velocity: Vec2,
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

        let filename2 = "assets/paused.png";
        let texture2 = ctx.get_texture_registry().load(filename2)?;
        let mut sprite2 = AnimatedSprite::new(128, texture2)?;
        sprite2.set_scale(2);
        sprite2.set_position(200, 200);

        // ctx.play_sound("../src/resources/music/personal_space.wav")?;

        let game =
            ExampleGame {
                sprite: sprite,
                sprite2: sprite2,
                player_position: Vec2::new(),
                player_velocity: Vec2::new(),
            };

        Ok(game)
    }

    fn update(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        if ctx.paused
        {
            ctx.draw(&self.sprite2);
            return Ok(true);
        }
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

        let acceleration = new_speed - self.player_velocity;
        self.player_velocity = self.player_velocity + (acceleration * dt * 5.0);
        self.player_position = self.player_position + (self.player_velocity * dt);

        self.sprite.set_position(self.player_position.x as i32, self.player_position.y as i32);
        self.sprite.step_time(dt * 0.1 * self.player_velocity.len());
        ctx.draw(&self.sprite);
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

        Ok(true)
    }
}

fn main() {
    Engine::execute::<ExampleGame>(640, 480).unwrap();
}
