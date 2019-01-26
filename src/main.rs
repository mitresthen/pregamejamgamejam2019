extern crate engine;

use engine::prelude::*;

struct GoogleHomeopathicMedicine {
    level: Grid,
    camera_controller: AxisController,
    zoom_controller: SliderController,
    camera_velocity: Vec2
}

impl GameInterface for GoogleHomeopathicMedicine {
    fn get_title() -> &'static str {
        "Google Homopathic Medicine"
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let level : Image<RGBA> = Image::load("src/resources/image/temp_level.png")?;

        let mut grid = Grid::new(level, 120);

        let tr = ctx.get_texture_registry();

        grid.register_tile_type(
            RGBA { r: 0, g: 0, b: 0, a: 255 },
            tr.load("src/resources/image/tile_Yellow_2.png")?
        );

        grid.register_tile_type(
            RGBA { r: 255, g: 0, b: 0, a: 255 },
            tr.load("src/resources/image/Wall Tile_3boxesHigh.png")?
        );

        let game =
            GoogleHomeopathicMedicine {
                level: grid,
                camera_controller: AxisController::new(
                    Keycode::Up,
                    Keycode::Down,
                    Keycode::Left,
                    Keycode::Right
                ),
                zoom_controller: SliderController::new(
                    Keycode::Plus,
                    Keycode::Minus,
                    (1.0, 2.0)
                ),
                camera_velocity: Vec2::new(),
            };

        Ok(game)
    }

    fn update(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        let target_velocity = self.camera_controller.poll(ctx) * 250.0;

        self.camera_velocity.approach(target_velocity, dt * 250.0);

        ctx.move_camera(self.camera_velocity * dt);
        let zoom = self.zoom_controller.poll(&ctx, dt);
        ctx.set_camera_zoom(zoom);

        ctx.draw(&self.level);

        let fps = (1.0 / dt) as i32;

        Ok(true)
    }
}

fn main() {
    Engine::execute::<GoogleHomeopathicMedicine>(1920, 1080).unwrap();
}
