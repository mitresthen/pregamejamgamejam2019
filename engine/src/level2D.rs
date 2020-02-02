use std::collections::HashMap;
use texture_registry::Texture;
use serde_json;
use drawable::{Drawable, DrawContext};

use Engine;
use Color;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use vector::Vec2;
use rect::Rect2D;
use Error;

use transform::Transform;

#[derive(Serialize, Deserialize)]
pub struct LevelInstance {
    pub object_instances: Vec<ObjectInstance>,
    pub object_types: Vec<ObjectType> 
}

fn default_scale() -> f32 { 1.0 }

#[derive(Serialize, Deserialize)]
pub struct ObjectInstance {
    pub object_id: u32,
    pub position: Vec2,
    pub rotation: f32,
    #[serde(default = "default_scale")]
    pub scale: f32
}

#[derive(Serialize, Deserialize)]
pub struct ObjectType {
    pub file: String,
    pub density: u32,
    pub fixed: bool,
    pub layers: Vec<u32>,
}

pub struct Level2D {
    pub level_instance: LevelInstance,
    pub save_filename: String,
    pub object_textures: HashMap<String, Texture>,
    pub layer_max: u32,
    pub layers_to_draw: Vec<u32>
}

impl Drawable for Level2D {
    fn draw(&self, ctx: &mut DrawContext) {
        for i in 0..(self.layer_max+1) {
            for object in self.level_instance.object_instances.iter() {
                let object_type = &self.level_instance.object_types[object.object_id as usize];
                if object_type.layers.contains(&i) && self.layers_to_draw.contains(&i) {
                    let mut transf: Transform = Transform::new();
                    transf.set_angle(object.rotation);
                    transf.set_translation(object.position);
                    transf.set_scale(object.scale);
                    ctx.draw(&self.object_textures.get(&object_type.file).unwrap(), &transf);
                    if object_type.fixed {
                        ctx.draw_point(transf.get_translation(), Color::RGB(255, 0, 0));   
                    }
                }
            }
        }
    }
}

impl Level2D {
    pub fn load_from_file(ctx: &mut Engine, filename: &str) -> Level2D {
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
    
            let texture = ctx.get_texture_registry().load(&object_filename.to_str().unwrap()).unwrap();
    
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

    pub fn take_instance_at(&mut self, v: Vec2) -> Option<ObjectInstance> {
        let mut selected_index = None;

        for (index, instance) in self.level_instance.object_instances.iter().enumerate() {
            let object_type = self.level_instance.object_types.get(instance.object_id as usize).unwrap();
            let texture = self.object_textures.get(&object_type.file).unwrap();

            let half_size = Vec2::from_coords(texture.extent().width as f32, texture.extent().height as f32) * 0.5 * instance.scale;

            let rect = Rect2D::new(instance.position - half_size, instance.position + half_size);

            if rect.contains(v) {
                selected_index = Some(index);
            }
        }

        if let Some(index) = selected_index {
            Some(self.level_instance.object_instances.remove(index))
        } else {
            None
        }
    }

    pub fn set_layers_to_draw(&mut self, layers_to_draw: Vec<u32>) {
        self.layers_to_draw = layers_to_draw;
    }

    pub fn save_to_file(&self) -> Result<(), Error> {
        if let Ok(mut f) = File::create(self.save_filename.clone()) {
        
            let instance = serde_json::to_string_pretty(&self.level_instance).unwrap();
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

