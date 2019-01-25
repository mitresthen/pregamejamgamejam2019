use drawable::{DrawContext, Drawable};
use animated_sprite::AnimatedSprite;
use vector::Vec2;

use super::Error;

pub struct MoveableObject {
    animated_sprite: AnimatedSprite,
    position: Vec2,
}

impl MoveableObject {
    pub fn new(sprite: AnimatedSprite, initial_pos: Vec2) -> Result<MoveableObject, Error> {
        let moveable_object = 
            MoveableObject {
                animated_sprite = sprite,
                position = initial_pos
            }
    }
}