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
struct LayerInfo {
    file: String,
    tiles: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct LevelInfo {
    width: i32,
    height: i32,
    ground: LayerInfo,
    objects: LayerInfo
}



pub struct LevelEditor {
    ground: Grid2,
    ground_filename: String,
    objects: Grid2,
    objects_filename: String,
    controller: AxisController,
    zoom: SliderController,
    camera_velocity: Vec2,
    edit_layer: u32,
    painting_tile: Option<u32>
}

impl LevelEditor {
    pub fn get_edit_layer(&mut self) -> &mut Grid2 {
        if self.edit_layer == 0 {
            &mut self.ground
        } else {
            &mut self.objects
        }
    }
}

fn load_layer(
    ctx: &mut Engine,
    levels_folder: &PathBuf,
    image_folder: &PathBuf,
    layer_info: &LayerInfo
) -> (Grid2, String) {
    let mut filename = levels_folder.clone();
    filename.push(&layer_info.file);


    let mut grid =
        if let Ok(loaded_ground) = Grid2::load(filename.to_str().unwrap()) {
            loaded_ground
        } else {
            println!("Unable to open {}, creating new level", filename.to_str().unwrap());
            Grid2::new(32, 18, 120)
        };

    for tile in layer_info.tiles.iter() {
        let mut tile_filename = image_folder.clone();
        tile_filename.push(tile);
        println!("Loading tile texture: {:?}", tile_filename);

        let texture = ctx.get_texture_registry().load(&tile_filename.to_str().unwrap()).unwrap();

        grid.add_tile_type(texture);
    }

    (grid, filename.to_str().unwrap().to_string())
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

        let (ground, ground_filename) =  load_layer(ctx, &level_folder, &image_folder, &level_info.ground);
        let (objects, objects_filename) =  load_layer(ctx, &level_folder, &image_folder, &level_info.objects);

        let level_editor =
            LevelEditor {
                ground: ground,
                ground_filename: ground_filename,
                objects: objects,
                objects_filename: objects_filename,
                controller: AxisController::new(
                    Keycode::Up,
                    Keycode::Down,
                    Keycode::Left,
                    Keycode::Right
                ),
                zoom: SliderController::new(
                    Keycode::Minus,
                    Keycode::Plus,
                    (1.0, 4.0)
                ),
                camera_velocity: Vec2::new(),
                edit_layer: 1,
                painting_tile: None
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
                let maybe_painting_tile = self.painting_tile;

                let edit_layer : &mut Grid2 = self.get_edit_layer();

                if let Some(painting_tile) = maybe_painting_tile {
                    edit_layer.set_tile_at(drag_state.current, painting_tile);
                }
            }
        }

        ctx.draw(&self.ground);

        if self.edit_layer == 1 {
            ctx.draw(&self.objects);
        }

        Ok(true)
    }

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode, is_repeated: bool)
       -> Result<bool, Error>
    {
        if keycode == Keycode::L {
            self.edit_layer = (self.edit_layer + 1) % 2;

            println!("Current edit layer: {}", if self.edit_layer == 0 { "ground" } else { "objects" });
        }

        Ok(true)
    }

    fn on_mouse_button_down(&mut self, ctx: &mut Engine, x: i32, y: i32, button: MouseButton)
        -> Result<bool, Error>
    {
        let tile_index =
            {
                let edit_layer : &mut Grid2 = self.get_edit_layer();
                let p = ctx.screen_to_world(x, y);
                edit_layer.get_tile_at(p)
            };


        self.painting_tile = tile_index;

        Ok(true)
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, x: i32, y: i32, button: MouseButton)
        -> Result<bool, Error>
    {
        self.painting_tile = None;

        let edit_layer : &mut Grid2 = self.get_edit_layer();

        let step = if button == MouseButton::Left { 1 } else { edit_layer.get_tile_type_count() - 1 };

        if let Some(drag_state) = ctx.get_mouse_drag_state() {
            if (drag_state.start - drag_state.current).len() > 10.0 {
            } else {
                let maybe_tile = edit_layer.get_tile_at(drag_state.current);

                if let Some(mut tile_id) = maybe_tile {
                    tile_id = (tile_id + step) % edit_layer.get_tile_type_count();
                    edit_layer.set_tile_at(drag_state.current, tile_id);
                }
            }
        }
        Ok(true)
    }

    fn on_exit(&mut self) {
        println!("Saving ground to: {}", self.ground_filename);
        self.ground.save_to_file(&self.ground_filename).unwrap();
        println!("Saving objects to: {}", self.objects_filename);
        self.objects.save_to_file(&self.objects_filename).unwrap();
    }
}

fn main() {


    Engine::execute::<LevelEditor>(1280, 720).unwrap();
}

