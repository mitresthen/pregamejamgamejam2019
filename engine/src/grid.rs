use std::collections::HashMap;
use texture_registry::Texture;
use drawable::{Drawable, DrawContext};
use transform::Transform;
use vector::Vec2;
use image::{Image, RGBA};

pub type TileIndex = RGBA;

pub struct Grid {
    image: Image<RGBA>,
    tile_size: u32,
    tile_map: HashMap<TileIndex, Texture>
}

impl Grid {
    pub fn new(image: Image<RGBA>, tile_size: u32) -> Grid {
        Grid {
            image: image,
            tile_size: tile_size,
            tile_map: HashMap::new()
        }
    }

    pub fn register_tile_type(&mut self, id: TileIndex, texture: Texture) {
        self.tile_map.insert(id, texture);
    }
}

impl Drawable for Grid {
    fn draw(&self, ctx: &mut DrawContext) {
        let mut it = self.image.data().iter();

        for y in 0..self.image.height() {
            for x in 0..self.image.width() {
                let id = it.next().unwrap();

                if let Some(texture) = self.tile_map.get(id) {
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
                    transform.translate(Vec2::from_coords(0.0, extra_height as f32 * -0.5));

                    // For now we just scale all tiles so that they match exactly the width
                    let scale = self.tile_size as f32 / texture.extent().width as f32;
                    transform.set_scale(scale);

                    ctx.draw(&texture, &transform)
                }
            }
        }
    }
}
