use std::collections::HashMap;
use texture_registry::Texture;
use serde_json;

use Engine;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use vector::Vec2;
use Error;

#[derive(Serialize, Deserialize)]
pub struct LevelInstance {
    pub object_instances: Vec<ObjectInstance>,
    pub object_types: Vec<ObjectType> 
}

#[derive(Serialize, Deserialize)]
pub struct ObjectInstance {
    pub object_id: u32,
    pub position: Vec2,
    pub rotation: f32
}

#[derive(Serialize, Deserialize)]
pub struct ObjectType {
    pub file: String,
    density: u32,
    fixed: bool,
    layers: Vec<u32>,
}

pub struct Level2D {
    pub level_instance: LevelInstance,
    pub save_filename: String,
    pub object_textures: HashMap<String, Texture>,
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
        image_folder.push("images");

        let save_filename = filename.to_string();

        let mut object_textures: HashMap<String, Texture> = HashMap::new();

        for object in level_instance.object_types.iter() {
            let mut object_filename = image_folder.clone();
            object_filename.push(object.file.clone());
            println!("Loading object texture: {:?}", object_filename);
    
            let texture = _ctx.get_texture_registry().load(&object_filename.to_str().unwrap()).unwrap();
    
            object_textures.insert(object.file.clone(), texture);
        }
    
        Level2D {
            level_instance,
            save_filename,
            object_textures
        }
    }

    pub fn save_to_file(&self) -> Result<(), Error> {
        if let Ok(mut f) = File::create(self.save_filename.clone()) {
        
            let instance = serde_json::to_string(&self.level_instance).unwrap();
            f.write_all(&instance.as_bytes()).unwrap();

            Ok(())
        } else {
            Err(Error::IO { path: Some(self.save_filename.clone()) })
        }
    }

    pub fn save(&mut self) {
        println!("Saving to file");
        self.save_to_file().unwrap();
    }
}

