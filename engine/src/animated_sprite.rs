use drawable::{DrawContext, Drawable};
use texture_registry::Texture;

use super::Error;

pub struct AnimatedSprite {
    texture: Texture,
    tile_size: i32,
    current_mode: i32,
    current_frame: f32,
    mode_count: i32,
    frame_count: i32,
    position_x: i32,
    position_y: i32,
    scale: i32
}

impl AnimatedSprite {
    pub fn new(tile_size: i32, texture: Texture) -> Result<AnimatedSprite, Error> {
        let width = texture.width();
        let height = texture.height();

        if width % tile_size != 0 || height % tile_size != 0 {
            return Err(Error::InvalidTileSize);
        }

        let mode_count = height / tile_size;
        let frame_count = width / tile_size;

        let animated_sprite =
            AnimatedSprite {
                texture: texture,
                tile_size: tile_size,
                current_mode: 0,
                current_frame: 0.0,
                mode_count: mode_count,
                frame_count: frame_count,
                position_x: 0,
                position_y: 0,
                scale: 1
            };

        Ok(animated_sprite)
    }

    pub fn step_time(&mut self, dt: f32) {
        self.current_frame += dt;
        while self.current_frame as i32 > self.frame_count {
            self.current_frame -= self.frame_count as f32;
        }
    }

    pub fn set_position(&mut self, x: i32, y: i32) {
        self.position_x = x;
        self.position_y = y;
    }

    pub fn set_mode(&mut self, mode: i32) {
        if mode < 0 || mode >= self.mode_count {
            panic!("Mode out of range");
        }
        self.current_mode = mode;
    }

    pub fn set_scale(&mut self, scale: i32) {
        self.scale = scale;
    }
}

impl Drawable for AnimatedSprite {
    fn draw(&self, ctx: &mut DrawContext) {
        let f = (self.current_frame as i32) % self.frame_count;
        let m = self.current_mode;
        let ts = self.tile_size;
        let s = self.scale;

        let x = self.position_x;
        let y = self.position_y;

        use sdl2::rect::Rect;
        let src = Rect::new(f * ts, m * ts, ts as u32, ts as u32);
        let dst = Rect::new(x, y, (ts * s) as u32, (ts * s) as u32);

        ctx.copy_ex(&self.texture, src, dst);
    }
}

