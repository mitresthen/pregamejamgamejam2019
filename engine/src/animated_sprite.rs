use drawable::{DrawContext, Drawable};
use texture_registry::Texture;

use super::Error;
use vector::Vec2;
use transform::Transform;
use extent::Extent;
use offset::Offset;

pub struct AnimatedSprite {
    texture: Texture,
    tile_size: i32,
    current_mode: i32,
    current_frame: f32,
    mode_count: i32,
    frame_count: i32,
    position: Vec2,
    scale: f32
}

impl AnimatedSprite {
    pub fn new(tile_size: i32, texture: Texture) -> Result<AnimatedSprite, Error> {
        let extent = texture.extent();

        if extent.width % tile_size != 0 || extent.height % tile_size != 0 {
            return Err(Error::InvalidTileSize);
        }

        let mode_count = extent.height / tile_size;
        let frame_count = extent.width / tile_size;

        let animated_sprite =
            AnimatedSprite {
                texture: texture,
                tile_size: tile_size,
                current_mode: 0,
                current_frame: 0.0,
                mode_count: mode_count,
                frame_count: frame_count,
                position: Vec2::new(),
                scale: 1.0
            };

        Ok(animated_sprite)
    }

    pub fn step_time(&mut self, dt: f32) {
        self.current_frame += dt;
        while self.current_frame as i32 > self.frame_count {
            self.current_frame -= self.frame_count as f32;
        }
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn set_mode(&mut self, mode: i32) {
        if mode < 0 || mode >= self.mode_count {
            panic!("Mode out of range");
        }
        self.current_mode = mode;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn calculate_size(&mut self) -> Vec2 {
        Vec2 {
            x: self.tile_size as f32 * self.scale,
            y: self.tile_size as f32 * self.scale,
        }
    }
}

impl Drawable for AnimatedSprite {
    fn draw(&self, ctx: &mut DrawContext) {
        let mut offset = Offset::new();

        let frame = (self.current_frame as i32) % self.frame_count;

        offset.x = frame * self.tile_size;
        offset.y = self.current_mode * self.tile_size;

        let extent = Extent::new(self.tile_size, self.tile_size);
        let sub_texture = self.texture.sub_texture(offset, extent).unwrap();

        let mut transform = Transform::new();
        transform.set_translation(self.position);
        transform.set_scale(self.scale);

        ctx.draw(&sub_texture, &transform);
    }
}

