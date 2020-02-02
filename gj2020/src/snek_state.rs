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

        let tile_size = 240.0;
        let apple_tree = level.objects.take_tile_with_id(*level.special_blocks.get("apple_tree").unwrap());
        for (_, position) in apple_tree.iter() {
            let mut apple_tree = AppleTree::new(ctx)?;
            let apple_tree_mut = apple_tree.get_transform_mut();
            let correct_pos = Vec2 { x: -0.5 * tile_size, y: -0.5 * tile_size };
            apple_tree_mut.set_translation(*position + correct_pos);
            scene.add_object(apple_tree);
        }

        let mut snek = Snek::new(ctx)?;
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
                    let mut next_state = Some(self.return_to_state.take().unwrap());
                    let transition_state = TransitionState::new(self, move |_, _| Ok(next_state.take().unwrap()));
                    ctx.reset_sound()?;
                    ctx.play_sound(AudioLibrary::Fall)?;
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
    sprite: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    touched: i32,
}

impl AppleTree {
    pub fn new(ctx: &mut Engine) -> Result<AppleTree, Error> {
        let tr = ctx.get_texture_registry();
        let texture = tr.load("assets/images/AppleTree/apple_tree_full.png")?;

        let full_texture = texture.sub_texture(Offset::from_coords(0, 0), Extent::new(480 * 1, 480 * 1))?;
        let full_sprite = AnimatedSprite::new(Extent::new(480, 480), full_texture)?;

        let fall_texture = texture.sub_texture(Offset::from_coords(480, 0), Extent::new(480 * 1, 480 * 1))?;
        let fall_sprite = AnimatedSprite::new(Extent::new(480, 480), fall_texture)?;

        let mut sprite = AggregatedAnimatedSprite::new();
        sprite.add(full_sprite);
        sprite.add(fall_sprite);

        let mut apple_tree =
        AppleTree {
                sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                touched: 0,
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
        if self.touched == 1 {
            event_mailbox.submit_event(
                EventType::Suck,
                EventReceiver::Scene
            );
        }
        let sprite_transform = self.transform.clone();
        // sprite_transform.translate(Vec2::from_coords(0.0, 5.0));
        self.sprite.set_mode(self.touched);
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
                self.touched = 1;
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
}
