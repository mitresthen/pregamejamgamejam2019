extern crate rand;
use self::rand::Rng;
use std::mem;

use engine::prelude::*;
use audio_library::AudioLibrary;
use crate::hub_state::HubState;

pub struct Demon {
    texture: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    distance: f32,
    health: f32,
}

impl Demon {
    fn new(tex: AggregatedAnimatedSprite) -> Demon {
        Demon { texture: tex, transform: Transform::new(), velocity: Vec2{x:0.0,y:0.0}, distance: 1000.0, health: 1.0}
    }

    fn set_translation(&mut self, pos: Vec2) {
        self.transform.set_translation(pos);
        self.texture.set_transform(&self.transform);
    }
}

impl GameObject for Demon {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        self.distance -= dt * 100.0;
        if self.distance < 0.0 {
            self.distance = 0.0;
        }
        self.texture.set_scale( 0.005 * (1000.0 - self.distance) );
        if self.health < 0.0 {
            _event_mailbox.submit_event(EventType::Attack{damage:1.0}, EventReceiver::Scene);
            _event_mailbox.submit_event(EventType::DeleteMe, EventReceiver::Scene);
        }
        return true;
    }

    fn render(&self, ctx: &mut DrawContext) {
        self.texture.draw(ctx);
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { Some(self) }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { Some(self) }

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Attack{ damage } => {
                self.health -= damage;
                self.texture.set_mode(1);
                self.texture.set_transform(&self.transform);
                true},
            _ => false
        }
    }
}

impl PhysicalObject for Demon {
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


pub struct HellState {
    scene: Scene,
    demon_textures: Vec<Texture>,
    demons: Vec<SceneObjectId>,
    club_id : SceneObjectId,
    last_spawn: f32,
    kills: u8,
}

pub struct Club {
    texture: AggregatedAnimatedSprite,
    down: bool,
}

impl Club {
    fn new(tex: AggregatedAnimatedSprite) -> Club {
        let mut club = Club { texture: tex, down: false };
        club.texture.set_scale(1.0);
        club
    }
    fn set_translation(&mut self, pos: Vec2) {
        self.texture.set_position(pos);
    }
    fn hit(&mut self) {
    }
}

impl GameObject for Club {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        self.set_translation(_ctx.get_mouse_position().position);
        match _ctx.get_mouse_drag_state() {
            Some(x) => {
                    self.texture.set_mode(1);
                    if !self.down {
                        self.down = true;
                        let origin = _ctx.get_mouse_position().position;
                        let max_distance = Some(100.0);
                        _event_mailbox.submit_event(EventType::Attack{damage:1.0}, EventReceiver::Nearby{ origin, max_distance });
                    }
            }
            None => {
                self.down = false;
                self.texture.set_mode(0);
            }
        }
        return true;
    }

    fn render(&self, ctx: &mut DrawContext) {
        self.texture.draw(ctx);
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { None }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { None }

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        match event {
            _ => {
                false
            }
        }
    }

    fn get_z_index(&self) -> i32 { 69 }
}

impl HellState {
    pub fn new(ctx: &mut Engine) -> Result<Self, Error> {
        let mut scene = Scene::new();

        let tr = ctx.get_texture_registry();
        let demon_texture = tr.load("assets/images/Demon/Demon.png")?;
        let demon1_texture = tr.load("assets/images/Demon/Demon1.png")?;

        let club_down_texture = tr.load("assets/images/Club/Club_dwn.png")?;
        let club_down_sprite = AnimatedSprite::new(Extent::new(240,240), club_down_texture)?;

        let club_up_texture = tr.load("assets/images/Club/Club_up.png")?;
        let club_up_sprite = AnimatedSprite::new(Extent::new(240,240), club_up_texture)?;

        let mut club_sprite = AggregatedAnimatedSprite::new();
        club_sprite.add(club_up_sprite);
        club_sprite.add(club_down_sprite);

        ctx.replace_sound(AudioLibrary::Hell, 0, -1)?;
        let club = Club::new( club_sprite );
        let id = scene.add_object(club);
        let state =
            HellState {
                scene,
                demon_textures:vec![demon_texture, demon1_texture],
                demons: Vec::new(),
                club_id: id,
                last_spawn: 0.0,
                kills:0,
            };

        Ok(state)
    }
    fn spawn_demon_sprite(&self) -> AggregatedAnimatedSprite {
        let mut demon_sprite = AggregatedAnimatedSprite::new();
        for texture in &self.demon_textures {
            let sprite = AnimatedSprite::new(Extent::new(240,240), texture.clone()).unwrap();
            demon_sprite.add(sprite);
        }
        demon_sprite
    }
}


impl GameState for HellState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {

        self.last_spawn += dt;

        while self.last_spawn > 2.0
        {
            self.last_spawn -= 2.0;
            let mut rng = rand::thread_rng();
            let mut demon = Demon::new(self.spawn_demon_sprite());
            let bounds = ctx.get_screen_bounds();
            let x = rng.gen::<u32>() % ctx.get_width();
            let y = rng.gen::<u32>() % ctx.get_height();
            let world_pos = ctx.screen_to_world(x as i32,y as i32);
            demon.set_translation(world_pos);

            let id = self.scene.add_object(demon);
            self.demons.push(id);
        }
        let events = self.scene.update(ctx, None, dt);
        for event in events {
            match event.event_type {
                EventType::Attack{damage} =>  { self.kills +=1; print!("{} monster killed!\n", damage) },
                _ => ()
            }
        }
        if self.kills >=10 {
            return Ok(Box::new(HubState::new(ctx)?));
        }
        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.set_camera_zoom(1.0);
        self.scene.render(ctx);

        Ok(())
    }

    fn on_mouse_button_down(&mut self, ctx: &mut Engine, x: i32, y: i32, _button: MouseButton)
        -> Result<(), Error>
    {
        Ok(())
    }
}
