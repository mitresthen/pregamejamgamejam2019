use bounding_box::BoundingBox;
use drawable::DrawContext;
use transform::Transform;
use vector::Vec2;
use scene::SceneObjectId;
use Engine;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Items {
    Key,
    Gum,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Item {
    item: Items
}

#[derive(Debug, Clone)]
pub enum EventType {
    Interact,
    Collide { force: Vec2 },
    Probe { hint: String },
    ProbeReply { p: Vec2 },
    RayCast { origin: Vec2, target: Vec2 },
    RayCastReply { success: bool, target: Vec2 },
    Attack { damage: f32 },
    Loot { item: Item }
}

#[derive(Debug)]
pub enum EventReceiver {
    // Only the nearest object to the origin receives the event
    Nearest { origin: Vec2, max_distance: Option<f32> },
    // Every object receive the event
    Broadcast,
    // Send to specific object
    Addressed { object_id: SceneObjectId },
    // Send to the scene for internal handling
    Scene
}

#[derive(Debug)]
pub struct GameEvent {
    pub event_type: EventType,
    pub receiver: EventReceiver,
    pub sender: Option<SceneObjectId>,

}

pub trait EventMailbox  {
    fn submit_event(&mut self, event: EventType, receiver: EventReceiver);
}

pub struct EventQueue {
    queue: Vec<GameEvent>
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queue: Vec::new()
        }
    }

    pub fn poll(&mut self) -> Option<GameEvent> {
        self.queue.pop()
    }

    pub fn bind_to_sender(&mut self, sender_id: SceneObjectId) -> SenderBoundEventQueue {
        SenderBoundEventQueue {
            event_queue: self,
            sender_id: sender_id
        }
    }
}

impl EventMailbox for EventQueue {
    fn submit_event(&mut self, event: EventType, receiver: EventReceiver) {
        let game_event = GameEvent {
            event_type: event,
            receiver: receiver,
            sender: None
        };
        self.queue.push(game_event);
    }
}

pub struct SenderBoundEventQueue<'t> {
    event_queue: &'t mut EventQueue,
    sender_id: SceneObjectId
}

impl<'t> EventMailbox for SenderBoundEventQueue<'t> {
    fn submit_event(&mut self, event: EventType, receiver: EventReceiver) {
        let game_event = GameEvent {
            event_type: event,
            receiver: receiver,
            sender: Some(self.sender_id)
        };

        self.event_queue.queue.push(game_event);
    }
}


pub trait PhysicalObject {
    fn get_transform(&self) -> &Transform;

    fn get_transform_mut(&mut self) -> &mut Transform;

    fn get_velocity(&self) -> &Vec2;

    fn get_velocity_mut(&mut self) -> &mut Vec2;

    fn get_bounding_box(&self) -> Option<BoundingBox>;
}

pub trait GameObject: 'static {
    fn update(&mut self, ctx: &Engine, event_mailbox: &mut EventMailbox, dt: f32) -> bool;

    fn render(&self, ctx: &mut DrawContext);

    fn get_physical_object(&self) -> Option<&PhysicalObject> { None }

    fn get_physical_object_mut(&mut self) -> Option<&mut PhysicalObject> { None }

    fn on_event(&mut self, _event: EventType, sender: Option<SceneObjectId>) -> bool { false }
}
