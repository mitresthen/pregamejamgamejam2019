extern crate engine;

use engine::prelude::*;

struct ExampleGame {
    sprite: AnimatedSprite,
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
        sprite.set_scale(4.0);
        sprite.set_position(Vec2 { x: 100.0, y: 100.0 });

        ctx.play_sound("../src/resources/music/personal_space.wav")?;

        let game =
            ExampleGame {
                sprite: sprite,
                player_position: Vec2::new(),
                player_velocity: Vec2::new(),
            };

        Ok(game)
    }

    fn update(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
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

        let mut screen_bounds = ctx.get_screen_bounds();
        let sprite_size = self.sprite.calculate_size();
        screen_bounds.min = screen_bounds.min;
        screen_bounds.max = screen_bounds.max;

        self.player_position = screen_bounds.wrap(self.player_position);

        self.sprite.set_position(self.player_position);
        self.sprite.step_time(dt * 0.1 * self.player_velocity.len());
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
