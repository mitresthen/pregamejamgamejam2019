extern crate engine;

use engine::prelude::*;
use std::vec::Vec;

pub struct Player {
    controller: AxisController,
    sprite: AnimatedSprite,
    position: Vec2,
    velocity: Vec2
}

impl Player {
    pub fn new(ctx: &mut Engine) -> Result<Player, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/characters.png")?;

        let mut sprite = AnimatedSprite::new(32, texture)?;
        sprite.set_scale(4.0);

        let player =
            Player {
                controller: AxisController::new(
                    Keycode::Up,
                    Keycode::Down,
                    Keycode::Left,
                    Keycode::Right,
                ),
                sprite: sprite,
                position: Vec2::new(),
                velocity: Vec2::new()
            };

        Ok(player)
    }
}

impl GameObject for Player {

    fn update(&mut self, ctx: &Engine, dt: f32) -> bool {
        let target_velocity =
            self.controller.poll(ctx) * 120.0;

        self.velocity.approach(target_velocity, 120.0 * dt);
        self.position = self.position + (self.velocity * dt);
        self.sprite.set_position(self.position);
        self.sprite.step_time(dt * self.velocity.len() * 0.1);

        true
    }

    fn render(&self, ctx: &mut DrawContext) {
        self.sprite.draw(ctx)
    }
}



pub struct GameObjectTest{
    scene: Scene
}

impl GameInterface for GameObjectTest {
    fn get_title() -> &'static str {
        "GameObjectTest"
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let mut scene = Scene::new();
        let player = Player::new(ctx)?;

        scene.add_object(player);

        let game =
            GameObjectTest
            {
                scene: scene
            };

        Ok(game)
    }

    fn update(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        self.scene.update(ctx, dt);
        self.scene.render(ctx);

        Ok(true)
    }
}

fn main() {
    Engine::execute::<GameObjectTest>(640, 480).unwrap();
}

