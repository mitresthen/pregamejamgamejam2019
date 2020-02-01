use std::collections::HashMap;
use texture_registry::Texture;
use serde_json;
use drawable::{Drawable, DrawContext};

use Engine;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use vector::Vec2;
use Error;

use transform::Transform;

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
    pub layer_max: u32,
    pub layers_to_draw: Vec<u32>
}

impl Drawable for Level2D {
    fn draw(&self, _ctx: &mut DrawContext) {
        for i in 0..(self.layer_max+1) {
            for object in self.level_instance.object_instances.iter() {
                let object_type = &self.level_instance.object_types[object.object_id as usize];
                if object_type.layers.contains(&i) && self.layers_to_draw.contains(&i) {
                    let mut transf: Transform = Transform::new();
                    transf.set_rotation(object.rotation);
                    transf.set_translation(object.position);
                    _ctx.draw(&self.object_textures.get(&object_type.file).unwrap(), &transf);
                }
            }
        }
    }
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

        let mut layer_max: u32 = 0;

        for object in level_instance.object_types.iter() {
            let curr_max = object.layers.iter().max();
            match curr_max {
                Some(i) => {
                    layer_max = layer_max.max(*i);
                },
                _ => {},
            };
            let mut object_filename = image_folder.clone();
            object_filename.push(object.file.clone());
            println!("Loading object texture: {:?}", object_filename);
    
            let texture = _ctx.get_texture_registry().load(&object_filename.to_str().unwrap()).unwrap();
    
            object_textures.insert(object.file.clone(), texture);
        }
        println!("Layer max is {}", layer_max);
        let mut layers_to_draw: Vec<u32> = Vec::new();
        for i in 0..(layer_max+1) {
            layers_to_draw.push(i);
        }
    
        Level2D {
            level_instance,
            save_filename,
            object_textures,
            layer_max,
            layers_to_draw
        }
    }

    pub fn set_layers_to_draw(&mut self, layers_to_draw: Vec<u32>) {
        self.layers_to_draw = layers_to_draw;
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

