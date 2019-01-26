extern crate engine;
extern crate sdl2;

use engine::prelude::*;
use sdl2::render::BlendMode;

mod player;
mod roomba;

struct GoogleHomeopathicMedicine {
    level: Grid,
    player_id: SceneObjectId,
    roomba_id: SceneObjectId, 
    scene: Scene,
    zoom_controller: SliderController,
    camera_velocity: Vec2,
    title_screen: SplashScreen,
    main_menu_screen: MenuScreen,
    pause_sprite: StaticSprite,
}

impl GameInterface for GoogleHomeopathicMedicine {
    fn get_title() -> &'static str {
        "Google Homopathic Medicine"
    }

    fn get_title_screen(&self) -> Option<SplashScreen> {
        Some(self.title_screen.clone())
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let level : Image<RGBA> = Image::load("assets/image/temp_level.png")?;

        let lightmap = ctx.get_texture_registry().load2("assets/image/grid_test_lightmap.png", BlendMode::Mod)?;
        let mut grid = Grid::new(level, 120, lightmap);
        ctx.loop_sound("assets/music/home_automation.wav", -1)?;

        {
            let tr = ctx.get_texture_registry();

            grid.register_tile_type(
                RGBA { r: 0, g: 0, b: 0, a: 255 },
                tr.load("assets/image/tile_Yellow_2.png")?
            );

            grid.register_tile_type(
                RGBA { r: 255, g: 0, b: 0, a: 255 },
                tr.load("assets/image/wall_with_dark_top.png")?
            );
            grid.register_tile_type(
                RGBA { r: 254, g: 0, b: 0, a: 255 },
                tr.load("assets/image/wall_dark_only.png")?
            );
            grid.register_tile_type(
                RGBA { r: 253, g: 0, b: 0, a: 255 },
                tr.load("assets/image/single_dark_tile.png")?
            );
        }

        let mut player = player::Player::new(ctx)?;
        player.get_transform_mut().set_translation(Vec2::from_coords(300.0, 300.0));

        let mut roomba = roomba::Roomba::new(ctx)?;

        let mut scene = Scene::new();
        let player_id = scene.add_object(player);
        let roomba_id = scene.add_object(roomba);

        // Loading StaticSprites
        let tr = ctx.get_texture_registry();

        let pause_sprite = StaticSprite::new(128, 64, tr.load("assets/image/paused.png")?)?;

        let title_background = StaticSprite::new(640, 480, tr.load("assets/image/title_background.png")?)?;

        let title_sprite = StaticSprite::new(128, 128, tr.load("assets/image/title.png")?)?;

        let title_screen =
            SplashScreen {
                background: title_background,
                foreground: title_sprite,
            };

        let mut main_menu_background = StaticSprite::new(1920, 1080, tr.load("assets/image/main_menu_background.png")?)?;
        let mut start_game_sprite = StaticSprite::new(128, 64, tr.load("assets/image/start_button.png")?)?;
        let mut exit_sprite = StaticSprite::new(128, 64, tr.load("assets/image/exit_button.png")?)?;

        let main_menu_choices =
            [
                MenuChoice
                {
                    name: "Start Adventure".to_string(),
                    target_game_state: GAMEPLAY_STATE,
                    sprite: start_game_sprite,
                    // sprite: pause_sprite.clone(),
                },
                MenuChoice
                {
                    name: "Quit Game".to_string(),
                    target_game_state: EXIT_STATE,
                    sprite: exit_sprite,
                    // sprite: pause_sprite.clone(),
                },
            ].to_vec();

        let main_menu_screen =
            MenuScreen
            {
                name: "Main Menu".to_string(),
                background: main_menu_background,
                options: main_menu_choices,
            };

        let game =
            GoogleHomeopathicMedicine {
                level: grid,
                scene: scene,
                player_id: player_id,
                roomba_id: roomba_id,
                zoom_controller: SliderController::new(
                    Keycode::Plus,
                    Keycode::Minus,
                    (0.5, 2.0)
                ),
                camera_velocity: Vec2::new(),
                title_screen: title_screen,
                main_menu_screen: main_menu_screen,
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
        &self.pause_sprite.set_position(player_position.shifted(0.0, -75.0));

        let player_bounding_box = self.scene.get(self.player_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_bounding_box()
            .unwrap();

        let roomba_position = self.scene.get(self.roomba_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_transform()
            .get_translation();

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
        let zoom = self.zoom_controller.poll(&ctx, dt);
        ctx.set_camera_zoom(zoom);
        &self.pause_sprite.set_scale(1.0/zoom);

        ctx.draw(&self.level);

        self.scene.render(ctx);

        // let fps = (1.0 / dt) as i32;

        Ok(true)
    }

    fn draw_main_menu(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        ctx.draw(&self.main_menu_screen);

        Ok(true)
    }

    fn draw_pause_menu(&mut self, ctx: &mut Engine, _dt: f32) -> Result<bool, Error> {
        ctx.draw(&self.pause_sprite);

        Ok(true)
    }

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode, is_repeated: bool) -> Result<bool, Error> {
        if ctx.state.gameplay_displayed
        {
            if keycode == Keycode::P && !is_repeated {
                ctx.invert_paused_state();
                return Ok(true);
            }
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

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, click_x: i32, click_y: i32) -> Result<bool, Error> {
        if ctx.state.is_on(TITLE_STATE)
        {
            ctx.end_title_screen();
            return Ok(true);
        }
        if ctx.state.is_on(MAIN_MENU_STATE)
        {
            // Click as "visible" in regards to camera.
            let mut cbc = Vec2 {
                x: click_x as f32,
                y: click_y as f32
            };
            let mut screen_transform = Transform::new();
            screen_transform.translate(ctx.get_screen_bounds().max * 0.5);
            cbc = screen_transform.transform_point_inv(cbc);
            cbc = ctx.get_camera().transform_point(cbc);

            match self.main_menu_screen.get_target_from_pos(cbc)
            {
                Some(game_state) => {
                    let gs_clone = game_state.clone();
                    ctx.state.go_to(game_state, ctx.last_game_state_change.get_time());
                    ctx.last_game_state_change.reset();
                    if (gs_clone.is_on(EXIT_STATE)) {
                        return Ok(false);
                    }
                    return Ok(true)
                },
                None => return Ok(true),
            }
        }
        Ok(true)
    }
}

fn main() {
    Engine::execute::<GoogleHomeopathicMedicine>(1280, 720).unwrap();
}
