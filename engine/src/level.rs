use serde_json;

use std::collections::HashMap;

use Engine;
use grid2::Grid2;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
//use serde_json::Map;


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
    objects: LayerInfo,
    special_blocks: HashMap<String, u32>
}


pub struct Level {
    pub ground: Grid2,
    ground_filename: String,
    pub objects: Grid2,
    objects_filename: String,
    pub special_blocks: HashMap<String, u32>
}

fn load_layer(
    ctx: &mut Engine,
    levels_folder: &Path,
    image_folder: &Path,
    layer_info: &LayerInfo,
    grid_size: u32,
) -> (Grid2, String) {
    let mut filename = levels_folder.to_path_buf();
    filename.push(&layer_info.file);

    let mut grid =
        if let Ok(loaded_ground) = Grid2::load(filename.to_str().unwrap()) {
            loaded_ground
        } else {
            println!("Unable to open {}, creating new level", filename.to_str().unwrap());
            Grid2::new(32, 18, grid_size)
        };

    for tile in layer_info.tiles.iter() {
        let mut tile_filename = image_folder.to_path_buf();
        tile_filename.push(tile);
        println!("Loading tile texture: {:?}", tile_filename);

        let texture = ctx.get_texture_registry().load(tile_filename.to_str().unwrap()).unwrap();

        grid.add_tile_type(texture);
    }

    (grid, filename.to_str().unwrap().to_string())
}

impl Level {
    pub fn load_from_file(
        ctx: &mut Engine,
        filename: &str,
        grid_size: u32,
    ) -> Level {
        let mut file = File::open(filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let level_info = serde_json::from_str::<LevelInfo>(&data).unwrap();

        let mut level_folder : PathBuf = filename.into();
        level_folder.pop();

        let mut image_folder = level_folder.clone();
        image_folder.pop();
        image_folder.push("images");


        let (ground, ground_filename) =
            load_layer(ctx, &level_folder, &image_folder, &level_info.ground, grid_size);
        let (objects, objects_filename) =
            load_layer(ctx, &level_folder, &image_folder, &level_info.objects, grid_size);


        Level {
            ground,
            ground_filename,
            objects,
            objects_filename,
            special_blocks: level_info.special_blocks
        }
    }

    pub fn save(&mut self) {
        println!("Saving ground to: {}", self.ground_filename);
        self.ground.save_to_file(&self.ground_filename).unwrap();
        println!("Saving objects to: {}", self.objects_filename);
        self.objects.save_to_file(&self.objects_filename).unwrap();
    }
}

