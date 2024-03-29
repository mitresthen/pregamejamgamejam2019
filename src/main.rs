extern crate engine;
extern crate sdl2;
extern crate rand;

use engine::prelude::*;
use std::collections::HashMap;

mod player;
mod roomba;
mod alex;
mod audio_library;
mod key;
mod dust;
mod door;
mod fuse_box;
mod menu_screen;

mod main_menu;
mod title_screen;
mod pause_screen;

use audio_library::AudioLibrary;

enum TransitionLogicLevel {
    Active,
    FadingOut { target_level: i32 },
    FadingIn { target_level: i32 }
}

pub struct RunningGameState {
    low_level: Grid2,
    mid_level: Grid2,
    player_id: SceneObjectId,
    scene: Scene,
    zoom_controller: SliderController,
    dimmer: Dimmer,
    transition_logic: TransitionLogicLevel,
    intro_played: bool,
    go_to_pause: bool
}

impl RunningGameState {
    fn new(ctx: &mut Engine) -> Result<Self, Error> {
        let dimmer = { Dimmer::new(ctx).with_initial_value(0.0).with_target_value(1.0) };
        let (low_level, mid_level, scene, player_id) = { Self::load_level(ctx, 0)? };

        let game =
            RunningGameState {
                low_level,
                mid_level,
                scene,
                player_id,
                zoom_controller: SliderController::new(
                    Keycode::Minus,
                    Keycode::Plus,
                    (0.5, 2.0)
                ),
                dimmer,
                transition_logic: TransitionLogicLevel::Active,
                intro_played: false,
                go_to_pause: false,
            };

        Ok(game)
    }

    fn load_level(ctx: &mut Engine, level_index: i32) -> Result<(Grid2, Grid2, Scene, SceneObjectId), Error> {

        let levels = ["assets/levels/GroundFloor.json", "assets/levels/Basement.json"];

        let level = Level::load_from_file(ctx, levels[level_index as usize], 120);

        let low_level = level.ground;
        let mut mid_level = level.objects;

        let mut player = player::Player::new(ctx)?;
        player.get_transform_mut().set_translation(Vec2::from_coords(300.0, 300.0));

        let mut scene = Scene::new();

        let alex_in_level = mid_level.take_tile_with_id(*level.special_blocks.get("alex").unwrap());
        for (_, position) in alex_in_level.iter() {
            let mut alex = alex::Alex::new(ctx)?;
            println!("creating alexa at position {:#?}", position);
            alex.get_transform_mut().set_translation(*position);
            scene.add_object(alex);
        }

        let player_id = scene.add_object(player);

        let roombas_in_level = mid_level.take_tile_with_id(*level.special_blocks.get("roomba").unwrap());

        for (_, position) in roombas_in_level.iter() {
            let mut roomba = roomba::Roomba::new(ctx)?;
            roomba.get_transform_mut().set_translation(*position);
            scene.add_object(roomba);
        }

        let dust = mid_level.take_tile_with_id(*level.special_blocks.get("dust").unwrap());

        for (_, position) in dust.iter() {


            let mut key = key::Key::new(ctx)?;
            key.get_transform_mut().set_translation(*position);
            scene.add_object(key);

            let mut dust = dust::Dust::new(ctx)?;
            dust.get_transform_mut().set_translation(*position);
            scene.add_object(dust);
        }

        for (index, (texture, position)) in mid_level.take_tile_with_id(*level.special_blocks.get("door").unwrap()).into_iter().enumerate() {
            let mut door = door::Door::new(ctx, texture);

            if index == 2 {
                door = door.with_key_requirement();
            }
            door.get_transform_mut().set_translation(position);
            scene.add_object(door);
        }

        let fuse_box = mid_level.take_tile_with_id(*level.special_blocks.get("fusebox").unwrap());
        for (_, position) in fuse_box.iter() {
            let mut fuse_box = fuse_box::FuseBox::new(ctx)?;
            fuse_box.get_transform_mut().set_translation(*position);
            scene.add_object(fuse_box);
        }




        Ok((low_level, mid_level, scene, player_id))
    }

    pub fn change_level(&mut self, ctx: &mut Engine, level_index: i32) {
        let old_position =
            self.scene.get(self.player_id).unwrap().get_physical_object().unwrap().get_transform().get_translation();

        let (low_level, mid_level, scene, player_id) = Self::load_level(ctx, level_index).unwrap();


        self.low_level = low_level;
        self.mid_level = mid_level;
        self.scene = scene;
        self.player_id = player_id;

        self.scene.get_mut(self.player_id).unwrap().get_physical_object_mut().unwrap().get_transform_mut().set_translation(
            old_position);
    }
}

impl GameState for RunningGameState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        if self.go_to_pause {
            self.go_to_pause = false;

            return Ok(Box::new(pause_screen::PauseScreenState::new(ctx, self)?));
        }

        match self.transition_logic {
            TransitionLogicLevel::FadingOut { target_level: next_level } => {
                self.dimmer.set_target(0.0);

                if self.dimmer.get_value() == 0.0 {
                    self.transition_logic = TransitionLogicLevel::FadingIn { target_level: next_level };

                    self.change_level(ctx, next_level);
                }
            },
            TransitionLogicLevel::FadingIn { target_level: _next_level } => {
                self.dimmer.set_target(1.0);

                if self.dimmer.get_value() == 1.0 {
                    self.transition_logic = TransitionLogicLevel::Active;
                }
            },
            TransitionLogicLevel::Active => { }
        };

        let player_position = self.scene.get(self.player_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_transform()
            .get_translation();

        ctx.set_camera_position(player_position);

        self.scene.update(ctx, Some(&self.mid_level), dt);
        self.dimmer.update(dt);

        if !self.intro_played {
            self.intro_played = true;
            ctx.replace_sound(AudioLibrary::Intro, 0, 0)?;
        }
        if ctx.is_done(0) {
            ctx.replace_sound(AudioLibrary::Music, 0, -1)?;
        }

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<(), Error>
    {
        let zoom = self.zoom_controller.poll(ctx, dt);
        ctx.set_camera_zoom(zoom);

        ctx.draw(&self.low_level);
        ctx.draw(&self.mid_level.interleave_scene(&self.scene));

        let mut transform = Transform::new();
        transform.set_translation(Vec2::from_coords(0.0, 0.0));

        // let fps = (1.0 / dt) as i32;
        self.dimmer.draw(ctx);

        Ok(())
    }

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode, is_repeated: bool) -> Result<(), Error> {
        if keycode == Keycode::P && !is_repeated {
            self.go_to_pause = true;
        }
        if keycode == Keycode::S && !is_repeated {
            ctx.play_sound(AudioLibrary::HeavySwitch)?;
        }
        if keycode == Keycode::T && !is_repeated {
            ctx.play_sound(AudioLibrary::Toilet)?;
        }
        if keycode == Keycode::M && !is_repeated {
            ctx.toggle_mute();
        }
        if keycode == Keycode::I && !is_repeated {
            ctx.increase_volume();
        }
        if keycode == Keycode::D && !is_repeated {
            ctx.decrease_volume();
        }

        Ok(())
    }

    fn on_key_up(&mut self, ctx: &mut Engine, keycode: Keycode) -> Result<(), Error> {
        if keycode == Keycode::M {
            self.transition_logic = TransitionLogicLevel::FadingOut { target_level: 1 };
        }

        self.on_key_down(ctx, keycode, true)?;

        Ok(())
    }
}

struct GoogleHomeopathicMedicine { }

impl GameInterface for GoogleHomeopathicMedicine {
    fn get_title() -> &'static str {
        "Google Homopathic Medicine"
    }

    fn create_starting_state(ctx: &mut Engine) -> Result<Box<dyn GameState>, Error> {
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

        ctx.load_sounds(sounds)?;

        ctx.reset_sound()?;

        ctx.loop_sound(AudioLibrary::Music, -1)?;

        Ok(Box::new(title_screen::TitleScreenState::new(ctx)?))
    }
}

fn main() {
    Engine::execute::<GoogleHomeopathicMedicine>(1280, 720).unwrap();
}
