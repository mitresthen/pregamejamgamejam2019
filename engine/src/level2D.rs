use serde_json;

use Engine;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use vector::Vec2;


#[derive(Serialize, Deserialize)]
pub struct LevelInstance {
    object_instances: Vec<ObjectInstance>,
    object_types: Vec<ObjectType> 
}

#[derive(Serialize, Deserialize)]
pub struct ObjectInstance {
    object_id: u32,
    position: Vec2,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectType {
    file: String,
    density: u32,
    fixed: bool,
    layers: Vec<u32>,
}

pub struct Level2D {
    pub level_instance: LevelInstance
}

impl Level2D {
    pub fn load_from_file(_ctx: &mut Engine, filename: &str) -> Level2D {
        let mut file = File::open(&filename).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let level_instance = serde_json::from_str::<LevelInstance>(&data).unwrap();

        let mut level_folder : PathBuf = filename.into();
        level_folder.pop();

        let mut image_folder = level_folder.clone();
        image_folder.pop();
        image_folder.push("image");

        Level2D {
            level_instance
        }
    }

    pub fn save(&mut self) {
        println!("Saving to file        NOT");
    }
}
