use bounding_box::BoundingBox;
use drawable::DrawContext;
use transform::Transform;
use vector::Vec2;
use Engine;

pub enum EventType {
    Interact,
}

pub struct GameEvent {
    event_type: EventType,
}

pub trait PhysicalObject {
    fn get_transform(&self) -> &Transform;

    fn get_transform_mut(&mut self) -> &mut Transform;

    fn get_velocity(&self) -> &Vec2;

    fn get_velocity_mut(&mut self) -> &mut Vec2;

    fn get_bounding_box(&self) -> Option<BoundingBox>;
}

pub trait GameObject: 'static {
    fn update(&mut self, ctx: &Engine, dt: f32) -> bool;

    fn render(&self, ctx: &mut DrawContext);

    fn get_physical_object(&self) -> Option<&PhysicalObject> { None }

    fn get_physical_object_mut(&mut self) -> Option<&mut PhysicalObject> { None }
}
