use animated_sprite::{AnimatedSprite, Animatable, SpriteTrait};
use bounding_box::BoundingBox;
use vector::Vec2;

use super::Error;

pub struct MovableObject {
    pub animated_sprite: AnimatedSprite, 
    player_position: Vec2,
    player_velocity: Vec2,
    acceleration: Vec2,
    pub bounding_box: BoundingBox
}

impl MovableObject {
    pub fn new(sprite: AnimatedSprite) -> Result<MovableObject, Error> {
        let size = sprite.calculate_size();

        let movable_object = 
            MovableObject {
                animated_sprite: sprite,
                player_position: Vec2::new(),
                player_velocity: Vec2::new(),
                acceleration: Vec2::new(),
                bounding_box: BoundingBox::new(size.x, size.y, Vec2::new())
            };

        Ok(movable_object)
    }

    pub fn get_position(&self) -> Vec2 {
        self.player_position
    }

    pub fn update(&mut self, dt: f32) -> Result<bool, Error> {
        self.animated_sprite.set_position(self.player_position);
        self.animated_sprite.step_time(dt * 0.05 * self.player_velocity.len());
        self.player_velocity = self.player_velocity + (self.acceleration * dt * 5.0);
        self.player_position = self.player_position + (self.player_velocity * dt);
        self.acceleration = Vec2::new();
        self.bounding_box.centre = self.player_position;
        Ok(true)
    }

    pub fn set_target_velocity(&mut self, target_velocity: Vec2) {
        let acceleration = target_velocity - self.player_velocity;
        self.acceleration = acceleration;
    }

    pub fn overlaps(&self, other_object: BoundingBox) -> bool {
        let overlap =  self.bounding_box.sat_overlap(other_object);
        match overlap {
            Some(_) => {
                return true
            }
            None => return false
        }
    }
}
