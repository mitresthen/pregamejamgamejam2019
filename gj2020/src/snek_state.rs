use engine::prelude::*;

use crate::snek::Snek;
use crate::audio_library::AudioLibrary;

pub struct SnekState {
    level: Level,
    snek_id: SceneObjectId,
    scene: Scene,
    return_to_state: Option<Box<dyn GameState>>
}

impl SnekState {
    pub fn new(ctx: &mut Engine, return_to_state: Box<dyn GameState>) -> Result<SnekState, Error> {
        let mut level = Level::load_from_file(ctx, "assets/levels/snek.json", 120);

        let mut scene = Scene::new();

        let apple_tree = level.objects.take_tile_with_id(*level.special_blocks.get("apple_tree").unwrap());
        for (_, position) in apple_tree.iter() {
            let mut apple_tree = AppleTree::new(ctx)?;
            println!("Creating apple tree at position {:#?}", position);
            apple_tree.get_transform_mut().set_translation(*position);
            scene.add_object(apple_tree);
        }

        let mut snek = Snek::new(ctx)?;

        let tile_size = 240.0;
        snek.set_position(Vec2::from_coords(1.5, 1.5) * tile_size);

        let snek_id = scene.add_object(snek);

        ctx.replace_sound(AudioLibrary::Snek, 0, -1)?;

        let snek_state =
            SnekState {
                level,
                snek_id,
                scene,
                return_to_state: Some(return_to_state),
            };

        Ok(snek_state)
    }
}

impl GameState for SnekState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        let events = self.scene.update(ctx, Some(&self.level.objects), dt);
        for event in events {
            match event.event_type {
                EventType::Suck => {
                    ctx.play_sound(AudioLibrary::Kill)?;
                    ctx.reset_sound()?;
                    let mut next_state = Some(self.return_to_state.take().unwrap());
                    let transition_state = TransitionState::new(self, move |_, _| Ok(next_state.take().unwrap()));
                    return Ok(Box::new(transition_state));
                },
                _ => ()
            }
        }

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        let snek_position = self.scene.get(self.snek_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_transform()
            .get_translation();

        ctx.set_camera_position(snek_position);
        ctx.set_camera_zoom(2.0);

        ctx.draw(&self.level.ground);
        ctx.draw(&self.level.objects.interleave_scene(&self.scene));

        Ok(())
    }
}

pub struct AppleTree {
    sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    touched: bool,
}

impl AppleTree {
    pub fn new(ctx: &mut Engine) -> Result<AppleTree, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/images/AppleTree/apple_tree.png")?;

        let sprite = AnimatedSprite::new(Extent::new(120, 240), texture)?;

        let mut apple_tree =
        AppleTree {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                touched: false,
            };
        apple_tree.transform.set_scale(1.0);

        Ok(apple_tree)
    }

    pub fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GameObject for AppleTree {
    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, _dt: f32) -> bool {
        if self.touched {
            // TODO: Make it look touched
            // let id = ctx.replace_sound(AudioLibrary::AccidentSong, self.sound_channel, 0).unwrap();
            // ctx.play(id);
            // self.prompted_for_response = false;
            event_mailbox.submit_event(
                EventType::Suck,
                EventReceiver::Scene
            );
        }
        let mut sprite_transform = self.transform.clone();
        sprite_transform.translate(Vec2::from_coords(0.0, -60.0));
        self.sprite.set_transform(&sprite_transform);

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
                self.touched = true;
                true
            },
            _ => {
                false
            }
        }
    }
}

impl PhysicalObject for AppleTree {
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

    // fn get_bounding_box(&self) -> Option<Box<dyn CollisionShape>> {
    //     let size = self.sprite.calculate_size() * 0.5;
    //     Some(Box::new(SquareShape::from_aabb(Rect2D::centered_square(size.x) + self.transform.get_translation())))
    // }
}
