use std::any::Any;
use drawable::DrawContext;
use transform::Transform;
use vector::Vec2;
use scene::SceneObjectId;
use Engine;
use rect::Rect2D;
use std::rc::Rc;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Items {
    Key,
    Gum,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Item {
    pub item: Items
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
    Loot { item: Item },
    RequestItem { item: Item },
    SendItem { item: Item },
    Suck,
    DeleteMe,
    FreeFromDust,
    PlankBroke,
    PlankRepaired,
    OceanRiseRate { rate: f32 },
    Custom { data: Rc<dyn Any> }
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
    Scene,
    // Only the nearby objects to the origin receives the event
    Nearby { origin: Vec2, max_distance: Option<f32> },
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

#[derive(Copy, Clone)]
pub struct SATResult {
    pub axis: Vec2,
    pub depth: f32
}

#[derive(Debug)]
pub struct Range {
    start: f32,
    end: f32
}

impl Range {
    pub fn inf_negative() -> Range {
        Range {
            start: std::f32::MAX,
            end: -std::f32::MAX
        }
    }

    pub fn expand(&mut self, p: f32) {
        self.start = self.start.min(p);
        self.end = self.end.max(p);
    }

    pub fn start(&self) -> f32 { self.start }
    pub fn end(&self) -> f32 { self.end }
    pub fn center(&self) -> f32 { (self.start + self.end) / 2.0 }
    pub fn size(&self) -> f32 { self.end - self.start }

    pub fn overlap(&self, other: &Range) -> Range {
        Range {
            start: self.start.max(other.start),
            end: self.end.min(other.end),
        }
    }
}

#[derive(Debug)]
pub struct Manifold {
    pub point_count: usize,
    pub points: [Vec2; 2]
}

impl Manifold {
    pub fn from_points(v: Vec<Vec2>) -> Manifold {

        if v.len() == 0 {
            panic!("Cannot create manifold from zero points!");
        }

        let skip = if v.len() > 2 { v.len() - 2 } else { 0 };

        let mut points = [ Vec2::new(), Vec2::new() ];

        let mut point_count = 0;
        for (i, p) in v.into_iter().skip(skip).enumerate() {
            points[i] = p;
            point_count += 1;
        }

        Manifold { points, point_count }
    }

    pub fn clip(self, other: Manifold, axis: Vec2) -> Manifold {
        let perp = axis.perpendicular();


        let mut range_a = Range::inf_negative();
        let mut range_b = Range::inf_negative();

        let mut depth_range = Range::inf_negative();

        for i in 0..self.point_count {
            range_a.expand(self.points[i].dot_product(perp));
            depth_range.expand(self.points[i].dot_product(axis));
        }

        for i in 0..other.point_count {
            range_b.expand(other.points[i].dot_product(perp));
            depth_range.expand(other.points[i].dot_product(axis));
        }

        let depth = depth_range.center();
        let width = range_a.overlap(&range_b);

        let manifold = 
            if width.size() <= 0.0 {
                Manifold {
                    point_count: 1,
                    points: [(perp * width.center()) + (axis * depth), Vec2::new()]
                }
            } else {
                Manifold {
                    point_count: 2,
                    points: [
                        (perp * width.start()) + (axis * depth),
                        (perp * width.end()) + (axis * depth),
                    ]
                }
            };

        manifold
    }
}

pub trait CollisionShape {
    fn get_points(&self) -> &[Vec2];

    fn get_axes(&self) -> &[Vec2];

    fn get_aabb(&self) -> Rect2D {
        let mut aabb = Rect2D {
            min: Vec2::from_coords(std::f32::MAX, std::f32::MAX),
            max: Vec2::from_coords(-std::f32::MAX, -std::f32::MAX),
        };

        for p in self.get_points() {
            aabb.min.x = aabb.min.x.min(p.x);
            aabb.min.y = aabb.min.y.min(p.y);
            aabb.max.x = aabb.max.x.max(p.x);
            aabb.max.y = aabb.max.y.max(p.y);
        }

        aabb
    }

    fn build_manifold(&self, axis: Vec2, transform: &Transform) -> Manifold {
        let mut points = Vec::new();

        let mut maximum = -std::f32::MAX;
        for p0 in self.get_points() {
            let p = transform.transform_point(*p0);
            let d = axis.dot_product(p);
            maximum = maximum.max(d);
            points.push((d, p));
        }

        points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let epsilon = 0.1;

        let points : Vec<Vec2> = points.into_iter()
            .filter(|(d, _p)| *d >= maximum - epsilon)
            .map(|(_, p)| p)
            .collect();

        Manifold::from_points(points)
    }

    fn sat_collide(
        &self,
        transform: &Transform,
        other: &dyn CollisionShape,
        other_transform: &Transform
    ) -> Option<SATResult> {
        let mut result =
            SATResult {
                axis: Vec2::new(),
                depth: std::f32::MAX,
            };

        let transformed_axes : Vec<Vec2> = self.get_axes()
            .iter()
            .map(|a| transform.transform_vector(*a).normalize())
            .chain(
                other.get_axes()
                .iter()
                .map(|b| other_transform.transform_vector(*b).normalize())
            )
            .collect();

        for axis in transformed_axes.into_iter() {
            let mut r1 = Range::inf_negative();
            let mut r2 = Range::inf_negative();

            for p in self.get_points() {
                r1.expand(transform.transform_point(*p).dot_product(axis));
            }

            for p in other.get_points() {
                r2.expand(other_transform.transform_point(*p).dot_product(axis));
            }

            let overlap = r1.overlap(&r2).size();

            let factor = if r1.center() < r2.center() { -1.0 } else { 1.0 };
            if overlap < result.depth {
                result.depth = overlap;
                result.axis = axis * factor;
            }
        }

        if result.depth > 0.0 {
            Some(result)
        } else {
            None
        }
    }
}

pub trait PhysicalObject {
    fn get_transform(&self) -> &Transform;

    fn get_transform_mut(&mut self) -> &mut Transform;

    fn get_velocity(&self) -> &Vec2;

    fn get_velocity_mut(&mut self) -> &mut Vec2;

    fn get_collision_shape(&self) -> Option<Rc<dyn CollisionShape>> { None }

    fn should_block(&self) -> bool { true }

    fn get_inv_mass(&self) -> f32 { 1.0 }

    fn get_rotatable(&self) -> Option<&dyn Rotatable> { None }

    fn get_rotatable_mut(&mut self) -> Option<&mut dyn Rotatable> { None }

    fn get_friction(&self) -> f32 { 0.3 }

    fn get_src_mask(&self) -> u32 { 0 }

    fn get_dst_mask(&self) -> u32 { 0 }
}

pub trait Rotatable {
    fn get_spin(&self) -> f32;

    fn get_spin_mut(&mut self) -> &mut f32;

    fn get_inv_inertia(&self) -> f32;
}

pub trait GameObject: 'static {
    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool;

    fn render(&self, ctx: &mut DrawContext);

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { None }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { None }

    fn on_event(&mut self, _event: EventType, _sender: Option<SceneObjectId>) -> bool { false }
    fn get_z_index(&self) -> i32 { 0 }
}
