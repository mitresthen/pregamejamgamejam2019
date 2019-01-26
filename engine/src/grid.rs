use std::collections::HashMap;
use texture_registry::Texture;
use drawable::{Drawable, DrawContext, Origin};
use transform::Transform;
use vector::Vec2;
use image::{Image, RGBA};
use bounding_box::BoundingBox;

pub type TileIndex = RGBA;

pub struct Grid {
    ground: Image<RGBA>,
    objects: Image<RGBA>,
    lightmap: Texture,
    tile_size: u32,
    tile_map: HashMap<TileIndex, Texture>
}

impl Grid {
    pub fn new(tile_size: u32, ground: Image<RGBA>, objects: Image<RGBA>, lightmap: Texture) -> Grid {
        Grid {
            ground: ground,
            objects: objects,
            lightmap: lightmap,
            tile_size: tile_size,
            tile_map: HashMap::new()
        }
    }

    pub fn register_tile_type(&mut self, id: TileIndex, texture: Texture) {
        self.tile_map.insert(id, texture);
    }

    pub fn get_collision_vector(&self, bounding_box: BoundingBox)
        -> Option<Vec2>
    {
        let upper_left = bounding_box.get_upper_left();
        let lower_right = bounding_box.get_lower_right();

        let start_x = ((upper_left.x / self.tile_size as f32).floor() as i32).max(0);
        let start_y = ((upper_left.y / self.tile_size as f32).floor() as i32).max(0);

        let end_x = ((lower_right.x / self.tile_size as f32).ceil() as i32).min(self.objects.width());
        let end_y = ((lower_right.y / self.tile_size as f32).ceil() as i32).min(self.objects.height());

        let mut best_axis = None;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = (y * self.objects.width()) + x;

                let tile_id = &self.objects.data()[index as usize];

                if tile_id.a < 128 {
                    continue;
                }

                let tile_bb =
                    BoundingBox::new(
                        self.tile_size as f32,
                        self.tile_size as f32,
                        Vec2::from_coords(
                            x as f32 + 0.5,
                            y as f32 + 0.5
                        ) * self.tile_size as f32
                    );

                if let Some(result) = bounding_box.sat_overlap(tile_bb) {
                    best_axis = Some(best_axis.map(|x: (Vec2, f32)| if x.1 > result.1 { x } else { result }).unwrap_or(result));
                }
            }
        }

        best_axis.map(|x| x.0)
    }

    fn _drawGround(&self, ctx: &mut DrawContext) {
        let data = self.ground.data();

        let h = self.ground.height();
        let w = self.ground.width();
        let mut y = 0;
        let mut x = 0;
        for y in 0..h {
            for x in 0..w {
                let id = &data[(w * y + x) as usize];
                if let Some(texture) = self.tile_map.get(id) {
                    let mut transform = Transform::new();
                    transform.set_translation(
                        Vec2::from_coords(
                            (x * self.tile_size as i32) as f32,
                            (y * self.tile_size as i32) as f32
                        )
                    );
                    ctx.draw2(texture, &transform, Origin::TopLeft);
                }
            }
        }
    }

    fn _drawObjects(&self, ctx: &mut DrawContext) {
        let data = self.objects.data();

        let h = self.objects.height();
        let w = self.objects.width();
        let mut y = 0;
        let mut x = 0;
        for y in 0..h {
            for x in 0..w {
                let id = &data[(w * y + x) as usize];
                if let Some(texture) = self.tile_map.get(id) {
                    let mut transform = Transform::new();
                    transform.set_translation(
                        Vec2::from_coords(
                            (x * self.tile_size as i32) as f32,
                            ((y + 1) * self.tile_size as i32) as f32
                        )
                    );
                    ctx.draw2(texture, &transform, Origin::BottomLeft);
                }
            }
        }
    }

    fn _drawLightmap(&self, ctx: &mut DrawContext) {
        let mut transform = Transform::new();
        transform.set_translation(Vec2::from_coords(0.0, 0.0));
        ctx.draw2(&self.lightmap, &transform, Origin::TopLeft);
    }
}

impl Drawable for Grid {
    fn draw(&self, ctx: &mut DrawContext) {
        self._drawGround(ctx);
        self._drawLightmap(ctx);
        self._drawObjects(ctx);
    }

}
