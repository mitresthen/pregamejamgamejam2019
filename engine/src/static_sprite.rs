use drawable::{DrawContext, Drawable};
use texture_registry::Texture;

use super::Error;
use vector::Vec2;
use transform::Transform;

#[derive(Clone)]
pub struct StaticSprite {
    texture: Texture,
    x_size: i32,
    y_size: i32,
    position: Vec2,
    scale: f32
}

impl StaticSprite {
    pub fn new(x_size: i32, y_size: i32, texture: Texture) -> Result<StaticSprite, Error> {
        let static_sprite =
            StaticSprite {
                texture: texture,
                x_size: x_size,
                y_size: y_size,
                position: Vec2::new(),
                scale: 1.0
            };

        Ok(static_sprite)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn calculate_size(&mut self) -> Vec2 {
        Vec2 {
            x: self.x_size as f32 * self.scale,
            y: self.y_size as f32 * self.scale,
        }
    }
}

impl Drawable for StaticSprite {
    fn draw(&self, ctx: &mut DrawContext) {
        let mut transform = Transform::new();
        transform.set_translation(self.position);
        transform.set_scale(self.scale);

        ctx.draw(&self.texture, &transform);
    }
}
