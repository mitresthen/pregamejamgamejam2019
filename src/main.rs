extern crate engine;
extern crate sdl2;
extern crate rand;

use engine::prelude::*;
use sdl2::render::BlendMode;
use std::collections::HashMap;
use std::collections::HashSet;

mod player;
mod roomba;
mod alex;
mod audio_library;
mod key;
mod dust;
mod door;
mod fuse_box;

use audio_library::AudioLibrary;

struct GoogleHomeopathicMedicine {
    low_level: Grid2,
    mid_level: Grid2,
    player_id: SceneObjectId,
    scene: Scene,
    zoom_controller: SliderController,
    camera_velocity: Vec2,
    title_screen: SplashScreen,
    main_menu_screen: MenuScreen,
    pause_screen: MenuScreen,
    dimmer: Dimmer,
}

impl GameInterface for GoogleHomeopathicMedicine {
    fn get_title() -> &'static str {
        "Google Homopathic Medicine"
    }

    fn get_title_screen(&self) -> Option<SplashScreen> {
        Some(self.title_screen.clone())
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let dimmer = { Dimmer::new(ctx).with_initial_value(0.0).with_target_value(1.0) };
        let level = Level::load_from_file(ctx, "assets/levels/GroundFloor.json");

        let mut low_level = level.ground;
        let mut mid_level = level.objects;

        let mut sounds = HashMap::new();
        sounds.insert(AudioLibrary::Music, "assets/music/home_automation.wav");
        sounds.insert(AudioLibrary::AccidentSong, "assets/music/would_you_like_to_hear_a_song.wav");
        sounds.insert(AudioLibrary::Intro, "assets/sounds/intro.wav");
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
        sounds.insert(AudioLibrary::Victory, "assets/sounds/all_your_home.wav");
        sounds.insert(AudioLibrary::Defeat, "assets/sounds/defeat.wav");
        sounds.insert(AudioLibrary::Nope, "assets/sounds/nope.wav");

        ctx.load_sounds(sounds);

        ctx.reset_sound();

        ctx.loop_sound(AudioLibrary::Music, -1)?;

        let mut player = player::Player::new(ctx)?;
        player.get_transform_mut().set_translation(Vec2::from_coords(300.0, 300.0));

        let mut alex = alex::Alex::new(ctx)?;
        alex.get_transform_mut().set_translation(Vec2::from_coords(8.5 * 120.0, 1.5 * 120.0));


        let mut scene = Scene::new();
        let player_id = scene.add_object(player);
        scene.add_object(alex);

        let roombas_in_level = mid_level.take_tile_with_id(19);

        for (_, position) in roombas_in_level.iter() {
            let mut roomba = roomba::Roomba::new(ctx)?;
            roomba.get_transform_mut().set_translation(*position);
            scene.add_object(roomba);
        }

        let dust = mid_level.take_tile_with_id(22);

        for (_, position) in dust.iter() {


            let mut key = key::Key::new(ctx)?;
            key.get_transform_mut().set_translation(*position);
            scene.add_object(key);

            let mut dust = dust::Dust::new(ctx)?;
            dust.get_transform_mut().set_translation(*position);
            scene.add_object(dust);
        }

        for (texture, position) in mid_level.take_tile_with_id(17) {
            let mut door = door::Door::new(ctx, texture);
            door.get_transform_mut().set_translation(position);
            scene.add_object(door);
        }

        let fuse_box = mid_level.take_tile_with_id(16);
        for (_, position) in fuse_box.iter() {
            let mut fuse_box = fuse_box::FuseBox::new(ctx)?;
            fuse_box.get_transform_mut().set_translation(*position);
            scene.add_object(fuse_box);
        }




        // Loading StaticSprites
        let tr = ctx.get_texture_registry();

        let title_background = StaticSprite::new(1280, 720, tr.load("assets/image/title_background.png")?)?;
        let title_sprite = StaticSprite::new(128, 128, tr.load("assets/image/title.png")?)?;

        let title_screen =
            SplashScreen {
                background: title_background,
                foreground: title_sprite,
            };

        let mut main_menu_background = StaticSprite::new(1280, 720, tr.load("assets/image/main_menu_background.png")?)?;
        let mut start_game_sprite = StaticSprite::new(128, 64, tr.load("assets/image/start_button.png")?)?;
        let mut exit_sprite = StaticSprite::new(128, 64, tr.load("assets/image/exit_button.png")?)?;

        let main_menu_choices =
            [
                MenuChoice
                {
                    name: "Start Adventure".to_string(),
                    target_game_state: GAMEPLAY_STATE,
                    sprite: start_game_sprite,
                },
                MenuChoice
                {
                    name: "Quit Game".to_string(),
                    target_game_state: EXIT_STATE,
                    sprite: exit_sprite,
                },
            ].to_vec();

        let main_menu_screen =
            MenuScreen
            {
                name: "Main Menu".to_string(),
                background: main_menu_background,
                options: main_menu_choices,
                current_zoom: 1.0,
                camera_pos: Vec2::new(),
            };

        let mut pause_menu_background = StaticSprite::new(1280, 720, tr.load("assets/image/pause_menu_background.png")?)?;
        let mut continue_sprite = StaticSprite::new(128, 64, tr.load("assets/image/continue_button.png")?)?;
        let mut return_to_menu_sprite = StaticSprite::new(128, 64, tr.load("assets/image/return_to_menu_button.png")?)?;

        let pause_menu_choices =
            [
                MenuChoice
                {
                    name: "Continue Adventure".to_string(),
                    target_game_state: GAMEPLAY_STATE,
                    sprite: continue_sprite,
                },
                MenuChoice
                {
                    name: "Return to Main Menu".to_string(),
                    target_game_state: RESET_GAME,
                    sprite: return_to_menu_sprite,
                },
            ].to_vec();

        let pause_screen =
            MenuScreen
            {
                name: "Pause Menu".to_string(),
                background: pause_menu_background,
                options: pause_menu_choices,
                current_zoom: 1.0,
                camera_pos: Vec2::new(),
            };

        let game =
            GoogleHomeopathicMedicine {
                low_level: low_level,
                mid_level: mid_level,
                scene: scene,
                player_id: player_id,
                zoom_controller: SliderController::new(
                    Keycode::Minus,
                    Keycode::Plus,
                    (0.5, 2.0)
                ),
                camera_velocity: Vec2::new(),
                title_screen: title_screen,
                main_menu_screen: main_menu_screen,
                pause_screen: pause_screen,
                dimmer: dimmer,
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
        &self.main_menu_screen.set_camera_pos(player_position);
        &self.pause_screen.set_camera_pos(player_position);

        self.scene.update(ctx, Some(&self.mid_level), dt);
        self.dimmer.update(dt);

        if ctx.is_done(0) {
            ctx.replace_sound(AudioLibrary::Music, 0, -1);
        }

        Ok(true)
    }

    fn draw_gameplay(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        let zoom = self.zoom_controller.poll(&ctx, dt);
        ctx.set_camera_zoom(zoom);
        &self.main_menu_screen.set_scale(zoom);
        &self.pause_screen.set_scale(zoom);

        ctx.draw(&self.low_level);
        ctx.draw(&self.mid_level.interleave_scene(&self.scene));

        let mut transform = Transform::new();
        transform.set_translation(Vec2::from_coords(0.0, 0.0));

        // Scene is now rendered as a part of the interleaved grid
        //self.scene.render(ctx);

        // let fps = (1.0 / dt) as i32;
        self.dimmer.draw(ctx);

        Ok(true)
    }

    fn draw_main_menu(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        ctx.draw(&self.main_menu_screen);

        Ok(true)
    }

    fn draw_pause_menu(&mut self, ctx: &mut Engine, _dt: f32) -> Result<bool, Error> {
        ctx.draw(&self.pause_screen);

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
            if keycode == Keycode::M && !is_repeated {
                ctx.toggle_mute();
                return Ok(true);
            }
            if keycode == Keycode::I && !is_repeated {
                ctx.increase_volume();
                return Ok(true);
            }
            if keycode == Keycode::D && !is_repeated {
                ctx.decrease_volume();
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
        if ctx.state.presents_menu
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
            let current_screen = if ctx.state.gameplay_displayed
            {
                self.pause_screen.clone()
            } else {
                self.main_menu_screen.clone()
            };

            match current_screen.get_target_from_pos(cbc)
            {
                Some(game_state) => {
                    let dt = ctx.last_game_state_change.get_time();
                    if dt >= 0.5
                    {
                        let gs_clone = game_state.clone();
                        ctx.state.go_to(game_state, dt);
                        ctx.last_game_state_change.reset();
                        if gs_clone.is_on(EXIT_STATE) {
                            return Ok(false);
                        }
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
