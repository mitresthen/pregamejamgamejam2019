extern crate engine;
extern crate sdl2;
extern crate rand;

use engine::prelude::*;
use sdl2::render::BlendMode;
use std::collections::HashMap;

mod player;
mod roomba;
mod alex;

#[derive(Hash, PartialEq, Eq)]
enum AudioLibrary {
    Music,
    Toilet, Drain,
    Switch1, Switch2,
    Steps1, Steps2,
    Rustle,
    MoveMetalObject,
    BigMetallicPlong, MetallicPlong, MetallicPling, MetallicHit,
    LockerOpen, LockerClose,
    HooverStart, HooverStop, HooverLoop,
    HeavySwitch, HeavySwitchMetallic,
    HeavySteps1, HeavySteps2,
    FluorescentLight1, FluorescentLight2, FluorescentLight3,
    FloorSqueak, FloorCreakShort, FloorCreakLong,
    Drop1, Drop2, Drop3,
    DrawerOpenSlow, DrawerOpenFast,
    DrawerCloseSlow, DrawerCloseFast,
    DoubleDrop, DoubleDropDull,
    DoorOpen1, DoorOpen2, DoorOpen3,
    DoorClose3, DoorClose2, DoorClose1,
}

struct GoogleHomeopathicMedicine {
    low_level: Grid2,
    mid_level: Grid2,
    lightmap: Texture,
    player_id: SceneObjectId,
    roomba_id: SceneObjectId,
    alex_id: SceneObjectId,
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
        let level = Level::load_from_file(ctx, "assets/levels/tilemap.json");

        let mut low_level = level.ground;
        let mut mid_level = level.objects;

        let mut sounds = HashMap::new();
        sounds.insert(AudioLibrary::Music, "assets/music/home_automation.wav");
        sounds.insert(AudioLibrary::Toilet, "assets/sounds/toilet.wav");
        sounds.insert(AudioLibrary::Switch1, "assets/sounds/switch1.wav");
        sounds.insert(AudioLibrary::Switch2, "assets/sounds/switch2.wav");
        sounds.insert(AudioLibrary::Steps1, "assets/sounds/steps1.wav");
        sounds.insert(AudioLibrary::Steps2, "assets/sounds/steps2.wav");
        sounds.insert(AudioLibrary::Rustle, "assets/sounds/rustle.wav");
        sounds.insert(AudioLibrary::MoveMetalObject, "assets/sounds/move_metal_object_on_floor.wav");
        sounds.insert(AudioLibrary::MetallicPlong, "assets/sounds/metallic_plong.wav");
        sounds.insert(AudioLibrary::MetallicPling, "assets/sounds/metallic_pling.wav");
        sounds.insert(AudioLibrary::MetallicHit, "assets/sounds/metallic_hit.wav");
        sounds.insert(AudioLibrary::LockerOpen, "assets/sounds/locker_open.wav");
        sounds.insert(AudioLibrary::LockerClose, "assets/sounds/locker_close.wav");
        sounds.insert(AudioLibrary::HooverStart, "assets/sounds/hoover_start.wav");
        sounds.insert(AudioLibrary::HooverStop, "assets/sounds/hoover_stop.wav");
        sounds.insert(AudioLibrary::HooverLoop, "assets/sounds/hoover_loop.wav");
        sounds.insert(AudioLibrary::HeavySwitch, "assets/sounds/heavy_switch.wav");
        sounds.insert(AudioLibrary::HeavySwitchMetallic, "assets/sounds/heavy_switch_metallic.wav");
        sounds.insert(AudioLibrary::HeavySteps1, "assets/sounds/heavy_steps1.wav");
        sounds.insert(AudioLibrary::HeavySteps2, "assets/sounds/heavy_steps2.wav");
        sounds.insert(AudioLibrary::FluorescentLight1, "assets/sounds/fluorescent_light_start1.wav");
        sounds.insert(AudioLibrary::FluorescentLight2, "assets/sounds/fluorescent_light_start2.wav");
        sounds.insert(AudioLibrary::FluorescentLight3, "assets/sounds/fluorescent_light_start3.wav");
        sounds.insert(AudioLibrary::FloorSqueak, "assets/sounds/floor_squeak.wav");
        sounds.insert(AudioLibrary::FloorCreakShort, "assets/sounds/floor_creak_short.wav");
        sounds.insert(AudioLibrary::FloorCreakLong, "assets/sounds/floor_creak_long.wav");
        sounds.insert(AudioLibrary::Drop1, "assets/sounds/drop1.wav");
        sounds.insert(AudioLibrary::Drop2, "assets/sounds/drop2.wav");
        sounds.insert(AudioLibrary::Drop3, "assets/sounds/drop3.wav");
        sounds.insert(AudioLibrary::DrawerOpenSlow, "assets/sounds/drawer_open_slow.wav");
        sounds.insert(AudioLibrary::DrawerOpenFast, "assets/sounds/drawer_open_fast.wav");
        sounds.insert(AudioLibrary::DrawerCloseSlow, "assets/sounds/drawer_close_slow.wav");
        sounds.insert(AudioLibrary::DrawerCloseFast, "assets/sounds/drawer_close_fast.wav");
        sounds.insert(AudioLibrary::Drain, "assets/sounds/drain.wav");
        sounds.insert(AudioLibrary::DoubleDrop, "assets/sounds/double_drop.wav");
        sounds.insert(AudioLibrary::DoubleDropDull, "assets/sounds/double_drop_dull.wav");
        sounds.insert(AudioLibrary::DoorOpen1, "assets/sounds/door_open1.wav");
        sounds.insert(AudioLibrary::DoorOpen2, "assets/sounds/door_open2.wav");
        sounds.insert(AudioLibrary::DoorOpen3, "assets/sounds/door_open3.wav");
        sounds.insert(AudioLibrary::DoorClose3, "assets/sounds/door_close3.wav");
        sounds.insert(AudioLibrary::DoorClose2, "assets/sounds/door_close2.wav");
        sounds.insert(AudioLibrary::DoorClose1, "assets/sounds/door_close1.wav");
        sounds.insert(AudioLibrary::BigMetallicPlong, "assets/sounds/big_metallic_plong.wav");

        ctx.load_sounds(sounds);

        ctx.loop_sound(AudioLibrary::Music, -1)?;

        let lightmap = ctx.get_texture_registry().load2("assets/image/grid_test_lightmap.png", BlendMode::Mod)?;

        let mut player = player::Player::new(ctx)?;
        player.get_transform_mut().set_translation(Vec2::from_coords(300.0, 300.0));

        let mut roomba = roomba::Roomba::new(ctx)?;
        roomba.get_transform_mut().set_translation(Vec2::from_coords(400.0, 400.0));

        let mut alex = alex::Alex::new(ctx)?;
        alex.get_transform_mut().set_translation(Vec2::from_coords(8.5 * 120.0, 1.5 * 120.0));


        let mut scene = Scene::new();
        let player_id = scene.add_object(player);
        let roomba_id = scene.add_object(roomba);
        let alex_id = scene.add_object(alex);

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
                low_level: low_level,
                mid_level: mid_level,
                lightmap: lightmap,
                scene: scene,
                player_id: player_id,
                roomba_id: roomba_id,
                alex_id: alex_id,
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

        self.scene.update(ctx, Some(&self.mid_level), dt);

        Ok(true)
    }

    fn draw_gameplay(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        let zoom = self.zoom_controller.poll(&ctx, dt);
        ctx.set_camera_zoom(zoom);
        &self.pause_sprite.set_scale(1.0/zoom);

        ctx.draw(&self.low_level);
        ctx.draw(&self.mid_level.interleave_scene(&self.scene));

        let mut transform = Transform::new();
        transform.set_translation(Vec2::from_coords(0.0, 0.0));

        if std::env::var("DO_NOT_DRAW_LIGHTMAP").is_err() {
            ctx.get_draw_context().draw2(&self.lightmap, &transform, Origin::TopLeft);
        }

        // Scene is now rendered as a part of the interleaved grid
        //self.scene.render(ctx);

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
            if keycode == Keycode::S && !is_repeated {
                ctx.play_sound(AudioLibrary::HeavySwitch);
                return Ok(true);
            }
            if keycode == Keycode::T && !is_repeated {
                ctx.play_sound(AudioLibrary::Toilet);
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

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, click_x: i32, click_y: i32, button: MouseButton)
        -> Result<bool, Error>
    {
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
