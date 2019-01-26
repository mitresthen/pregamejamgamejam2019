use bounding_box::BoundingBox;
use drawable::DrawContext;
use transform::Transform;
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

    fn get_bounding_box(&self) -> Option<BoundingBox>;
}

pub trait GameObject: 'static {
    fn update(&mut self, ctx: &Engine, dt: f32) -> bool;

    fn render(&self, ctx: &mut DrawContext);

    fn get_physical_object(&self) -> Option<&PhysicalObject> { None }
}
