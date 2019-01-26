extern crate engine;

use engine::prelude::*;

mod player;

struct GoogleHomeopathicMedicine {
    level: Grid,
    player_id: SceneObjectId,
    scene: Scene,
    zoom_controller: SliderController,
    camera_velocity: Vec2,
    title_screen: SplashScreen,
    pause_sprite: AnimatedSprite,
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

        let mut player = player::Player::new(ctx)?;
        player.get_transform_mut().set_translation(Vec2::from_coords(300.0, 300.0));


        let mut scene = Scene::new();
        let player_id = scene.add_object(player);

        let title_background_filename = "src/resources/image/title_background.png";
        let title_background_texture = ctx.get_texture_registry().load(title_background_filename)?;
        let mut title_background = AnimatedSprite::new(128, title_background_texture)?;
        title_background.set_scale(4.0);
        title_background.set_position(ctx.get_screen_bounds().center());

        let title_filename = "src/resources/image/title.png";
        let title_texture = ctx.get_texture_registry().load(title_filename)?;
        let mut title_sprite = AnimatedSprite::new(128, title_texture)?;

        let pause_filename = "src/resources/image/paused.png";
        let pause_texture = ctx.get_texture_registry().load(pause_filename)?;
        let mut pause_sprite = AnimatedSprite::new(128, pause_texture)?;

        let title_screen =
            SplashScreen {
                background: title_background,
                foreground: title_sprite,
            };

        let game =
            GoogleHomeopathicMedicine {
                level: grid,
                scene: scene,
                player_id: player_id,
                zoom_controller: SliderController::new(
                    Keycode::Plus,
                    Keycode::Minus,
                    (0.5, 2.0)
                ),
                camera_velocity: Vec2::new(),
                title_screen: title_screen,
                pause_sprite: pause_sprite,
            };

        Ok(game)
    }


    fn update_gameplay(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        let player_position = self.scene.get(self.player_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_transform()
            .get_translation();

        ctx.set_camera_position(player_position);

        let zoom = self.zoom_controller.poll(&ctx, dt);
        ctx.set_camera_zoom(zoom);

        let player_bounding_box = self.scene.get(self.player_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_bounding_box()
            .unwrap();

        if let Some(axis) = self.level.get_collision_vector(player_bounding_box) {
            let player_object = self.scene.get_mut(self.player_id).unwrap();
            let physical_object = player_object.get_physical_object_mut().unwrap();
            let player_velocity = physical_object.get_velocity_mut();
            *player_velocity = axis * 100.0;
        }

        self.scene.update(ctx, dt);

        Ok(true)
    }

    fn draw_gameplay(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        ctx.draw(&self.level);

        self.scene.render(ctx);

        let fps = (1.0 / dt) as i32;

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
        if keycode == Keycode::P && !is_repeated {
            ctx.invert_paused_state();
            return Ok(true);
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
    Engine::execute::<GoogleHomeopathicMedicine>(1280, 720).unwrap();
}
