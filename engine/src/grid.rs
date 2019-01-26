use std::collections::HashMap;
use texture_registry::Texture;
use drawable::{Drawable, DrawContext, Origin};
use transform::Transform;
use vector::Vec2;
use image::{Image, RGBA};
use sdl2::render::BlendMode;
use bounding_box::BoundingBox;
use scene::Scene;
use rect::Rect2D;

pub type TileIndex = RGBA;

pub struct Grid {
    image: Image<RGBA>,
    lightmap: Texture,
    tile_size: u32,
    tile_map: HashMap<TileIndex, Texture>,
    interleaved_scene: Option<Scene>
}

impl Grid {
    pub fn new(image: Image<RGBA>, tile_size: u32, lightmap: Texture) -> Grid {
        Grid {
            image: image,
            lightmap: lightmap,
            tile_size: tile_size,
            tile_map: HashMap::new(),
            interleaved_scene: None
        }
    }

    pub fn set_interleaved_scene(&mut self, scene: Scene) {
        self.interleaved_scene = Some(scene);
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

        let end_x = ((lower_right.x / self.tile_size as f32).ceil() as i32).min(self.image.width());
        let end_y = ((lower_right.y / self.tile_size as f32).ceil() as i32).min(self.image.height());

        let mut best_axis = None;

        let black = RGBA { r: 0, g: 0, b: 0, a: 255 };

        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = (y * self.image.width()) + x;

                let tile_id = self.image.data().iter().nth(index as usize).unwrap();

                if *tile_id == black {
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

    fn draw_with_interleaved_scene(
        &self,
        ctx: &mut DrawContext,
        interleaved_scene: Option<&Scene>
    ) {
        let mut it = self.image.data().iter();

        for y in 0..self.image.height() {
            if let Some(scene) = interleaved_scene {
                let row_rect = self.get_row_rect(y);
                let objects = scene.get_objects_in_rect(row_rect);

                for object in objects.iter() {
                    object.render(ctx);
                }
            }

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

                    // Textures that are taller than the grid size are now drawn with
                    // the overlapping height divided equally on the tile below and
                    // the tile above. Move it up half the extra height to make it only
                    // overlap the tile above
                    let extra_height = texture.extent().height - self.tile_size as i32;
                    transform.translate(Vec2::from_coords(0.0, extra_height as f32 * -0.5));

                    // For now we just scale all tiles so that they match exactly the width
                    let scale = self.tile_size as f32 / texture.extent().width as f32;
                    transform.set_scale(scale);

                    ctx.draw2(&texture, &transform, Origin::TopLeft)
                }
            }
        }
        let mut transform = Transform::new();
        transform.set_translation(Vec2::from_coords(0.0, 0.0));
        ctx.draw2(&self.lightmap, &transform, Origin::TopLeft);
    }

    pub fn interleave_scene<'t>(&'t self, scene: &'t Scene) -> GridWithInterleavedScene<'t> {
        GridWithInterleavedScene {
            scene: scene,
            grid: self
        }
    }
}

impl Drawable for Grid {
    fn draw(&self, ctx: &mut DrawContext) {
        self.draw_with_interleaved_scene(ctx, None);
    }
}

pub struct GridWithInterleavedScene<'t> {
    scene: &'t Scene,
    grid: &'t Grid
}

impl<'t> Drawable for GridWithInterleavedScene<'t> {
    fn draw(&self, ctx: &mut DrawContext) {
        self.grid.draw_with_interleaved_scene(ctx, Some(self.scene));
    }
}
