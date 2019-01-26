extern crate engine;

use engine::prelude::*;

struct GridTest {
    grid: Grid,
    controller: AxisController,
    zoom_controller: SliderController,
    camera_velocity: Vec2,
}

impl GameInterface for GridTest {
    fn get_title() -> &'static str {
        "GridTest"
    }

    fn get_title_screen(&self) -> Option<SplashScreen> {
        None
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let level : Image<RGBA> = Image::load("assets/grid_test.png")?;

        let mut grid = Grid::new(level, 120);

        let tr = ctx.get_texture_registry();

        grid.register_tile_type(RGBA { r: 255, g: 0, b: 0, a: 255 }, tr.load("../src/resources/image/tile_Yellow_2.png")?);

        let game =
            GridTest {
                grid: grid,
                controller: AxisController::new(Keycode::Up, Keycode::Down, Keycode::Left, Keycode::Right),
                zoom_controller: SliderController::new(Keycode::Plus, Keycode::Minus, (1.0, 2.0)),
                camera_velocity: Vec2::new(),
            };

        Ok(game)
    }

    fn update(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        let target_velocity = self.controller.poll(ctx) * 2500.0;

        self.camera_velocity.approach(target_velocity, dt * 2500.0);

        ctx.move_camera(self.camera_velocity * dt);
        let zoom = self.zoom_controller.poll(&ctx, dt);
        ctx.set_camera_zoom(zoom);

        ctx.draw(&self.grid);

        let fps = (1.0 / dt) as i32;

        Ok(true)
    }
}

fn main() {
    Engine::execute::<GridTest>(1920, 1080).unwrap();
}
