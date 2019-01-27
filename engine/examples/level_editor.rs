extern crate engine;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use engine::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LevelInfo {
    width: i32,
    height: i32,
    ground_file: String,
    objects_file: String,
    tiles: Vec<String>
}


pub struct LevelEditor {
    ground: Grid2,
    ground_filename: String,
    controller: AxisController,
    zoom: SliderController,
    camera_velocity: Vec2
}

impl GameInterface for LevelEditor {
    fn get_title() -> &'static str { "Level editor" }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        use std::env;
        let args: Vec<String> = env::args().collect();

        let tilemap_filename = args.iter().nth(1).unwrap();

        let mut level_folder : PathBuf = tilemap_filename.into();
        level_folder.pop();

        let mut image_folder = level_folder.clone();
        image_folder.pop();
        image_folder.push("image");

        let mut file = File::open(&tilemap_filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let level_info : LevelInfo = serde_json::from_str(&data).unwrap();

        let mut ground_filename = level_folder.clone();
        ground_filename.push(&level_info.ground_file);

        let mut ground =
            if let Ok(loaded_ground) = Grid2::load(ground_filename.to_str().unwrap()) {
                loaded_ground
            } else {
                println!("Unable to open {}, creating new level", ground_filename.to_str().unwrap());
                Grid2::new(32, 18, 120)
            };


        for tile in level_info.tiles {
            let mut tile_filename = image_folder.clone();
            tile_filename.push(tile);
            println!("Loading tile texture: {:?}", tile_filename);

            let texture = ctx.get_texture_registry().load(&tile_filename.to_str().unwrap()).unwrap();

            ground.add_tile_type(texture);
        }


        let level_editor =
            LevelEditor {
                ground: ground,
                ground_filename: ground_filename.to_str().unwrap().to_string(),
                controller: AxisController::new(
                    Keycode::Up,
                    Keycode::Down,
                    Keycode::Left,
                    Keycode::Right
                ),
                zoom: SliderController::new(
                    Keycode::Plus,
                    Keycode::Minus,
                    (0.5, 2.0)
                ),
                camera_velocity: Vec2::new(),
            };

        Ok(level_editor)
    }

    fn update_gameplay(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        self.camera_velocity = self.controller.poll(ctx) * 400.0;
        ctx.move_camera(self.camera_velocity * dt);
        let zoom = self.zoom.poll(ctx, dt);
        ctx.set_camera_zoom(zoom);

        Ok(true)
    }

    fn draw_gameplay(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        if let Some(drag_state) = ctx.get_mouse_drag_state() {
            if (drag_state.start - drag_state.current).len() > 10.0 {

            }
        }

        ctx.draw(&self.ground);

        Ok(true)
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, x: i32, y: i32)
        -> Result<bool, Error>
    {
        if let Some(drag_state) = ctx.get_mouse_drag_state() {
            if (drag_state.start - drag_state.current).len() > 10.0 {
            } else {
                let maybe_tile = self.ground.get_tile_at(drag_state.current);

                if let Some(mut tile_id) = maybe_tile {
                    tile_id = (tile_id + 1) % self.ground.get_tile_type_count();
                    self.ground.set_tile_at(drag_state.current, tile_id);
                }
            }

        }
        Ok(true)
    }

    fn on_exit(&mut self) {
        println!("Saving ground to: {}", self.ground_filename);
        self.ground.save_to_file(&self.ground_filename).unwrap()
    }
}

fn main() {


    Engine::execute::<LevelEditor>(1280, 720).unwrap();
}

