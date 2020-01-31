use serde_json;

use std::collections::HashMap;

use Engine;
use grid2::Grid2;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

##[derive(Serialize, Deserialize)]
pub struct LevelInstance {
    pub objects: HashMap<ObjectInstance, ObjectType>
}

#[derive(Serialize, Deserialize)]
pub struct ObjectInstance {
    object_id: u32,
    position: Vec2,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectType {
    file: String,
    weight: u32,
    fixed: bool,
    Layers: Vec2<u32>
}

pub struct Level2D {
    pub level_instance: LevelInstance
}

impl Level2D {
    pub fn load_from_file(ctx: &mut Engine, filename: &str) -> Level {
        let mut file = File::open(&filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let level_instance = serde_json::from_str::<LevelInstance>(&data).unwrap();

        let mut level_folder : PathBuf = filename.into();
        level_folder.pop();

        let mut image_folder = level_folder.clone();
        image_folder.pop();
        image_folder.push("image");

        Level {
            level_instance
        }
    }
}

