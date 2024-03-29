use texture_registry::Texture;
use drawable::{Drawable, DrawContext};
use transform::Transform;
use vector::Vec2;
use scene::{
    Scene,
    LevelCollider,
};
use game_object::{
    SATResult,
    GameObject,
    CollisionShape
};
use rect::Rect2D;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use Error;
use super::bincode;

use crate::square_shape::SquareShape;

pub type TileIndex = u32;

#[derive(Serialize, Deserialize)]
struct GridData {
    width: i32,
    height: i32,
    tile_size: u32,
    buffer: Vec<TileIndex>,
}

pub struct Grid2 {
    buffer: Vec<TileIndex>,
    width: i32,
    height: i32,
    tile_size: u32,
    tile_list: Vec<Texture>,
    interleaved_scene: Option<Scene>
}

impl Grid2 {
    pub fn new(width: i32, height: i32, tile_size: u32) -> Grid2 {
        Grid2 {
            buffer: vec![0u32 as TileIndex; (width * height) as usize],
            width,
            height,
            tile_size,
            tile_list: Vec::new(),
            interleaved_scene: None
        }
    }

    pub fn load(filename: &str) -> Result<Grid2, Error> {
        match  File::open(filename) {
            Ok(mut f) => {
                let mut bytes : Vec<u8> = Vec::new();
                f.read_to_end(&mut bytes).unwrap();
                let grid_data = bincode::deserialize::<GridData>(&bytes).unwrap();

                Ok(
                    Grid2 {
                        buffer: grid_data.buffer,
                        width: grid_data.width,
                        height: grid_data.height,
                        tile_size: grid_data.tile_size,
                        tile_list: Vec::new(),
                        interleaved_scene: None
                    }
                )
            },
            Err(e) => {
                println!("IO error: {:?}, filename={}", e, filename);
                Err(Error::IO { path: Some(filename.to_string()) })
            }
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Error> {
        if let Ok(mut f) = File::create(filename) {
            let grid_data =
                GridData {
                    width: self.width,
                    height: self.height,
                    buffer: self.buffer.clone(),
                    tile_size: self.tile_size
                };
            let bytes = bincode::serialize(&grid_data).unwrap();

            let _ignored = f.write(&bytes).unwrap();

            Ok(())
        } else {
            Err(Error::IO { path: Some(filename.to_string()) })
        }
    }

    pub fn set_interleaved_scene(&mut self, scene: Scene) {
        self.interleaved_scene = Some(scene);
    }

    pub fn add_tile_type(&mut self, texture: Texture) {
        self.tile_list.push(texture);
    }

    fn get_row_rect(&self, y: i32) -> Rect2D {
        use std::f32;
        Rect2D {
            min: Vec2::from_coords(
                f32::MIN,
                (y * self.tile_size as i32) as f32
            ),
            max: Vec2::from_coords(
                f32::MAX,
                ((y + 1) * self.tile_size as i32) as f32
            )
        }
    }

    pub fn take_tile_with_id(&mut self, id: u32) -> Vec<(Texture, Vec2)> {
        let mut out : Vec<(Texture, Vec2)> = Vec::new();

        let mut it = self.buffer.iter_mut();
        for y in 0..self.height {
            for x in 0..self.width {
                let tile_index = it.next().unwrap();

                if *tile_index == id {
                    let p = Vec2::from_coords(x as f32 + 0.5, y as f32 + 0.5) * (self.tile_size as f32);
                    out.push((self.tile_list.get(*tile_index as usize).unwrap().clone(), p));
                    *tile_index = 0;
                }
            }
        }

        out
    }

    fn draw_with_interleaved_scene(
        &self,
        ctx: &mut DrawContext,
        interleaved_scene: Option<&Scene>
    ) {
        let mut it = self.buffer.iter();

        for y in 0..self.height {
            if let Some(scene) = interleaved_scene {
                let row_rect = self.get_row_rect(y);
                let mut objects : Vec<(&dyn GameObject, f32)> = scene.get_objects_in_rect(row_rect).into_iter().map(
                    |o| {
                        let y = o.get_physical_object().unwrap().get_transform().get_translation().y;
                        (o, y)
                    }).collect();

                objects.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                for (object, _y) in objects.iter() {
                    object.render(ctx);
                }
            }

            for x in 0..self.width {
                let id = it.next().unwrap();

                if let Some(texture) = self.tile_list.get(*id as usize) {
                    let mut transform = Transform::new();
                    transform.set_translation(
                        Vec2::from_coords(
                            (x * self.tile_size as i32) as f32,
                            (y * self.tile_size as i32) as f32
                        )
                    );

                    // Currently the tiles would be drawn with the center at the grid
                    // intersection. We move them a half tile size down
                    transform.translate(
                        Vec2::from_coords(
                            self.tile_size as f32 * 0.5,
                            self.tile_size as f32 * 0.5
                        )
                    );


                    // Textures that are taller than the grid size are now drawn with
                    // the overlapping height divided equally on the tile below and
                    // the tile above. Move it up half the extra height to make it only
                    // overlap the tile above
                    let extra_height = texture.extent().height - self.tile_size as i32;
                    let extra_width = texture.extent().width - self.tile_size as i32;
                    transform.translate(Vec2::from_coords(extra_width as f32 * -0.5, extra_height as f32 * -0.5));

                    ctx.draw(texture, &transform)
                }
            }
        }
    }

    pub fn interleave_scene<'t>(&'t self, scene: &'t Scene) -> GridWithInterleavedScene<'t> {
        GridWithInterleavedScene {
            scene,
            grid: self
        }
    }

    pub fn get_tile_at(&self, p: Vec2) -> Option<TileIndex> {
        let ix = (p.x / self.tile_size as f32).floor() as i32;
        let iy = (p.y / self.tile_size as f32).floor() as i32;

        if ix < 0 || iy < 0 || ix >= self.width as i32 || iy >= self.height as i32 {
            return None;
        }

        let index = ((iy * self.width) + ix) as usize;

        Some(*self.buffer.get(index).unwrap())
    }

    pub fn set_tile_at(&mut self, p: Vec2, tile_index: TileIndex) -> Result<(), Error> {
        let ix = (p.x / self.tile_size as f32).floor() as i32;
        let iy = (p.y / self.tile_size as f32).floor() as i32;

        if ix < 0 || iy < 0 || ix >= self.width || iy >= self.height {
            return Err(Error::FatalError("Index out of bounds".to_string()));
        }

        let index = ((iy * self.width) + ix) as usize;

        if let Some(dst) = self.buffer.get_mut(index) {
            *dst = tile_index;
            Ok(())
        } else {
            Err(Error::FatalError("Index out of bounds".to_string()))
        }
    }

    pub fn get_tile_type_count(&self) -> u32 { self.tile_list.len() as u32 }
}

impl LevelCollider for Grid2 {
    fn get_collision_vector(&self, shape: &dyn CollisionShape, transform: &Transform)
        -> Option<Vec2>
    {
        let aabb = (shape.get_aabb() * transform.get_scale()) + transform.get_translation();

        let start_x = ((aabb.min.x / self.tile_size as f32).floor() as i32).max(0);
        let start_y = ((aabb.min.y / self.tile_size as f32).floor() as i32).max(0);

        let end_x = ((aabb.max.x / self.tile_size as f32).ceil() as i32).min(self.width);
        let end_y = ((aabb.max.y / self.tile_size as f32).ceil() as i32).min(self.height);

        let mut best_axis : Option<SATResult> = None;

        let empty : TileIndex = 0;

        let tile_transform = Transform::new();

        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = (y * self.width) + x;

                let tile_id = self.buffer.get(index as usize).unwrap();

                if *tile_id == empty {
                    continue;
                }

                let tile_size = self.tile_size as f32;
                let tile_center = Vec2::from_coords(x as f32 + 0.5, y as f32 + 0.5) * tile_size;
                let tile_rect = Rect2D::centered_square(tile_size) + tile_center;
                let tile_shape = SquareShape::from_aabb(tile_rect);

                if let Some(result) = shape.sat_collide(transform, &tile_shape, &tile_transform) {
                    best_axis = Some(best_axis.map(|x| if x.depth > result.depth { x } else { result }).unwrap_or(result));
                }
            }
        }

        best_axis.map(|x| x.axis)
    }
}

impl Drawable for Grid2 {
    fn draw(&self, ctx: &mut DrawContext) {
        self.draw_with_interleaved_scene(ctx, None);
    }
}

pub struct GridWithInterleavedScene<'t> {
    scene: &'t Scene,
    grid: &'t Grid2
}

impl<'t> Drawable for GridWithInterleavedScene<'t> {
    fn draw(&self, ctx: &mut DrawContext) {
        self.grid.draw_with_interleaved_scene(ctx, Some(self.scene));
    }
}
