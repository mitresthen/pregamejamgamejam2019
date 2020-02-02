extern crate rand;

use std::rc::Rc;
use engine::prelude::*;
use self::rand::Rng;

pub struct Plank {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    inv_mass: f32,
    shape: Rc<dyn CollisionShape>,
    plank_state: PlankState
}

#[derive(Debug)]
enum PlankState {
    Ok,
    Broken,
    Repairing,
    Repaired
}

impl Plank {
    pub fn new(ctx: &mut Engine) -> Result<Plank, Error> {
        let sprite;
        {
            let tr = ctx.get_texture_registry();
            let texture_on = tr.load("assets/images/Plank.png")?;
            sprite = AnimatedSprite::new(Extent::new(240, 240), texture_on)?;
        }

        let mut size = sprite.calculate_size();
        size.x = size.x/4.0;
        let shape = SquareShape::from_aabb(Rect2D::centered_rectangle(size));

        let mut plank =
            Plank {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                inv_mass: 0.0,
                shape: Rc::new(shape),
                plank_state: PlankState::Ok
            };
            plank.transform.set_scale(1.0);

        Ok(plank)
    }

    pub fn toggle_texture(&mut self, ctx: &mut Engine) {
        let tr = ctx.get_texture_registry();

        println!("Toggeling texture to {:#?}", self.plank_state);

        match self.plank_state {
            PlankState::Broken => {
                let texture_on = tr.load("assets/images/BrokenPlank.png");
                let sprite = AnimatedSprite::new(Extent::new(240, 240), texture_on.unwrap());
                self.sprite = sprite.unwrap();
                self.sprite.set_transform(&self.transform);
            },
            _ => {
                let texture_off = tr.load("assets/images/Plank.png");
                let sprite = AnimatedSprite::new(Extent::new(240, 240), texture_off.unwrap());
                self.sprite = sprite.unwrap();
                self.sprite.set_transform(&self.transform);    
            }
        }

    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn set_transform(&mut self, input_transform: Transform) {
        self.transform = input_transform;
        self.sprite.set_transform(&self.transform);
    }
}

impl GameObject for Plank {
    fn update(&mut self, ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        match self.plank_state {
            PlankState::Ok => {
                let mut rng = rand::thread_rng();
                let x: f32 = rng.gen();
                if(x > 0.99999){
                    println!("Plank broke {}", x);
                    self.plank_state = PlankState::Broken;
                    self.toggle_texture(ctx);
                }
            },
            PlankState::Repairing => {
                self.toggle_texture(ctx);
                self.plank_state = PlankState::Repaired
            },
            _ => { }
        }

        true
    }

    fn render(&self, ctx: &mut DrawContext) {
        self.sprite.draw(ctx)
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> {
        Some(self)
    }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> {
        Some(self)
    }

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Interact => {
                println!("Someone tried to fix the plank");
                match self.plank_state {
                    PlankState::Broken  => {
                        self.plank_state = PlankState::Repairing;
                    },
                    _ => {}
                }
                true
            },
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for Plank {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn get_velocity(&self) -> &Vec2 {
        &self.velocity
    }

    fn get_velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }

    fn get_inv_mass(&self) -> f32 { self.inv_mass }

    fn get_collision_shape(&self) -> Option<Rc<dyn CollisionShape>> {
        Some(self.shape.clone())
    }
}
