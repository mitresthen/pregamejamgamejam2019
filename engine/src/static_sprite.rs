use drawable::{DrawContext, Drawable};
use texture_registry::Texture;

use super::Error;
use rect::Rect2D;
use transform::Transform;
use vector::Vec2;

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
                texture,
                x_size,
                y_size,
                position: Vec2::new(),
                scale: 1.0
            };

        Ok(static_sprite)
    }

    pub fn get_position(&self) -> Vec2{
        self.position
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

    fn get_rect(&self) -> Rect2D {
        Rect2D {
            min: Vec2 {
                x: self.position.x as f32 - (self.x_size as f32 / 2.0),
                y: self.position.y as f32 - (self.y_size as f32 / 2.0),
            },
            max: Vec2 {
                x: self.position.x as f32 + (self.x_size as f32 / 2.0),
                y: self.position.y as f32 + (self.y_size as f32 / 2.0),
            }
        }
    }

    pub fn is_clicked(&self, click: Vec2) -> bool {
        let sprite_rect = self.get_rect();
        sprite_rect.is_clicked(click)
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
