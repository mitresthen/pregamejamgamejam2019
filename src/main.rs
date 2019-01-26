extern crate engine;

use engine::prelude::*;

struct GoogleHomeopathicMedicine {
    level: Grid,
    camera_controller: AxisController,
    zoom_controller: SliderController,
    camera_velocity: Vec2,
    title_screen: SplashScreen,
}

impl GameInterface for GoogleHomeopathicMedicine {
    fn get_title() -> &'static str {
        "Google Homopathic Medicine"
    }

    fn get_title_screen(&self) -> Option<SplashScreen> {
        Some(self.title_screen.clone())
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let level : Image<RGBA> = Image::load("src/resources/image/temp_level.png")?;

        let mut grid = Grid::new(level, 120);

        {
            let tr = ctx.get_texture_registry();

            grid.register_tile_type(
                RGBA { r: 0, g: 0, b: 0, a: 255 },
                tr.load("src/resources/image/tile_Yellow_2.png")?
            );

            grid.register_tile_type(
                RGBA { r: 255, g: 0, b: 0, a: 255 },
                tr.load("src/resources/image/wall_with_dark_top.png")?
            );
            grid.register_tile_type(
                RGBA { r: 254, g: 0, b: 0, a: 255 },
                tr.load("src/resources/image/wall_dark_only.png")?
            );
            grid.register_tile_type(
                RGBA { r: 253, g: 0, b: 0, a: 255 },
                tr.load("src/resources/image/single_dark_tile.png")?
            );
        }

        let title_background_filename = "src/resources/image/title_background.png";
        let title_background_texture = ctx.get_texture_registry().load(title_background_filename)?;
        let mut title_background = AnimatedSprite::new(128, title_background_texture)?;
        title_background.set_scale(4.0);
        title_background.set_position(ctx.get_screen_bounds().center());

        let title_filename = "src/resources/image/title.png";
        let title_texture = ctx.get_texture_registry().load(title_filename)?;
        let mut title_sprite = AnimatedSprite::new(128, title_texture)?;
        title_sprite.set_scale(1.0);
        title_sprite.set_position(ctx.get_screen_bounds().center());

        let title_screen =
            SplashScreen {
                background: title_background,
                foreground: title_sprite,
            };

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
                    (0.5, 2.0)
                ),
                camera_velocity: Vec2::new(),
                title_screen: title_screen,
            };

        Ok(game)
    }

    fn update(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        let target_velocity = self.camera_controller.poll(ctx) * 1000.0;

        self.camera_velocity.approach(target_velocity, dt * 250.0);

        ctx.move_camera(self.camera_velocity * dt);
        let zoom = self.zoom_controller.poll(&ctx, dt);
        ctx.set_camera_zoom(zoom);

        ctx.draw(&self.level);

        let fps = (1.0 / dt) as i32;

        Ok(true)
    }

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode, is_repeated: bool) -> Result<bool, Error> {
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
    Engine::execute::<GoogleHomeopathicMedicine>(1920, 1080).unwrap();
}
