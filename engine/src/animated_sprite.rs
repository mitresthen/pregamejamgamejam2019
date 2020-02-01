use drawable::{DrawContext, Drawable};
use texture_registry::Texture;

use super::Error;
use vector::Vec2;
use transform::Transform;
use extent::Extent;
use offset::Offset;

#[derive(Clone)]
pub struct AnimatedSprite {
    texture: Texture,
    tile_extent: Extent,
    current_mode: i32,
    current_frame: f32,
    mode_count: i32,
    frame_count: i32,
    transform: Transform
}

pub trait SpriteTrait : Drawable + 'static {
    fn set_position(&mut self, p: Vec2);

    fn set_transform(&mut self, transform: &Transform);

    fn set_scale(&mut self, scale: f32);

    fn calculate_size(&self) -> Vec2;
}

pub trait Animatable : Drawable + 'static {
    fn set_mode(&mut self, mode: i32);

    fn get_mode_count(&self) -> i32;

    fn step_time(&mut self, dt: f32);
}

impl AnimatedSprite {
    pub fn new(tile_extent: Extent, texture: Texture) -> Result<AnimatedSprite, Error> {
        let extent = texture.extent();

        if extent.width % tile_extent.width != 0 || extent.height % tile_extent.height != 0 {
            return Err(Error::InvalidTileSize);
        }

        let mode_count = extent.height / tile_extent.height;
        let frame_count = extent.width / tile_extent.width;

        let animated_sprite =
            AnimatedSprite {
                texture: texture,
                tile_extent: tile_extent,
                current_mode: 0,
                current_frame: 0.0,
                mode_count: mode_count,
                frame_count: frame_count,
                transform: Transform::new(),
            };

        Ok(animated_sprite)
    }

}

impl Animatable for AnimatedSprite {
    fn set_mode(&mut self, mode: i32) {
        if mode < 0 || mode >= self.mode_count {
            panic!("Mode out of range");
        }
        self.current_mode = mode;
    }

    fn get_mode_count(&self) -> i32 {
        self.mode_count
    }

    fn step_time(&mut self, dt: f32) {
        self.current_frame += dt;
        while self.current_frame as i32 > self.frame_count {
            self.current_frame -= self.frame_count as f32;
        }
    }
}

impl SpriteTrait for AnimatedSprite {
    fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position);
    }

    fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }

    fn set_scale(&mut self, scale: f32) {
        self.transform.set_scale(scale);
    }

    fn calculate_size(&self) -> Vec2 {
        self.tile_extent.to_vec() * self.transform.get_scale()
    }
}

impl Drawable for AnimatedSprite {
    fn draw(&self, ctx: &mut DrawContext) {
        let mut offset = Offset::new();

        let frame = (self.current_frame as i32) % self.frame_count;

        offset.x = frame * self.tile_extent.width;
        offset.y = self.current_mode * self.tile_extent.height;

        let extent = Extent::new(self.tile_extent.width, self.tile_extent.height);
        let sub_texture = self.texture.sub_texture(offset, extent).unwrap();

        ctx.draw(&sub_texture, &self.transform);
    }
}

pub trait Aggregatable : Animatable + SpriteTrait { }

impl Aggregatable for AnimatedSprite { }

pub struct AggregatedAnimatedSprite {
    sprites: Vec<Box<dyn Aggregatable>>,
    sprite_index: i32,
    mode: i32,
}

impl AggregatedAnimatedSprite {
    pub fn new() -> AggregatedAnimatedSprite {
        AggregatedAnimatedSprite {
            sprites: Vec::new(),
            sprite_index: 0i32,
            mode: 0i32
        }
    }

    pub fn add<T: Aggregatable>(&mut self, t: T) {
        self.sprites.push(Box::new(t));
    }
}

impl Drawable for AggregatedAnimatedSprite {
    fn draw(&self, ctx: &mut DrawContext) {
        self.sprites[self.sprite_index as usize].draw(ctx);
    }
}

impl Animatable for AggregatedAnimatedSprite {
    fn set_mode(&mut self, mode: i32) {
        self.mode = mode;
        for (index, sprite) in self.sprites.iter().enumerate() {
            if self.mode >= sprite.get_mode_count() {
                self.mode -= sprite.get_mode_count();
            } else {
                self.sprite_index = index as i32;
                break;
            }
        }

        self.sprites[self.sprite_index as usize].set_mode(self.mode);
    }

    fn get_mode_count(&self) -> i32 {
        let mut total_mode_count = 0i32;
        for sprite in self.sprites.iter() {
            total_mode_count += sprite.get_mode_count();
        }
        total_mode_count
    }

    fn step_time(&mut self, dt: f32) {
        let index = self.sprite_index as usize;
        self.sprites[index].step_time(dt);
    }
}

impl SpriteTrait for AggregatedAnimatedSprite
{
    fn set_position(&mut self, position: Vec2) {
        self.sprites[self.sprite_index as usize].set_position(position);
    }

    fn set_transform(&mut self, transform: &Transform) {
        self.sprites[self.sprite_index as usize].set_transform(transform);
    }

    fn set_scale(&mut self, scale: f32) {
        self.sprites[self.sprite_index as usize].set_scale(scale);
    }

    fn calculate_size(&self) -> Vec2 {
        self.sprites[self.sprite_index as usize].calculate_size()
    }
}
