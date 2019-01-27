use serde::{Deserialize, Serialize};
use serde_json;

use Engine;
use grid2::Grid2;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;


#[derive(Serialize, Deserialize)]
pub struct LayerInfo {
    file: String,
    tiles: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct LevelInfo {
    width: i32,
    height: i32,
    ground: LayerInfo,
    objects: LayerInfo
}


pub struct Level {
    pub ground: Grid2,
    ground_filename: String,
    pub objects: Grid2,
    objects_filename: String,
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

impl Level {
    pub fn load_from_file(ctx: &mut Engine, filename: &str) -> Level {
        let mut file = File::open(&filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let level_info = serde_json::from_str::<LevelInfo>(&data).unwrap();

        let mut level_folder : PathBuf = filename.into();
        level_folder.pop();

        let mut image_folder = level_folder.clone();
        image_folder.pop();
        image_folder.push("image");


        let (ground, ground_filename) =
            load_layer(ctx, &level_folder, &image_folder, &level_info.ground);
        let (objects, objects_filename) =
            load_layer(ctx, &level_folder, &image_folder, &level_info.objects);


        Level {
            ground,
            ground_filename,
            objects,
            objects_filename
        }
    }

    pub fn save(&mut self) {
        println!("Saving ground to: {}", self.ground_filename);
        self.ground.save_to_file(&self.ground_filename).unwrap();
        println!("Saving objects to: {}", self.objects_filename);
        self.objects.save_to_file(&self.objects_filename).unwrap();
    }
}

