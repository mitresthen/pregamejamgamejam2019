extern crate rand;
use self::rand::Rng;

use engine::prelude::*;
use audio_library::AudioLibrary;

pub struct Background {
    texture: Texture,
    transform: Transform,

}

impl Background {
    fn new(tex: Texture) -> Background {
        Background{ texture: tex, transform: Transform::new() }
    }
}

impl GameObject for Background {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, _dt: f32) -> bool {
        //let screen = Vec2{x:(_ctx.get_width()/2) as f32, y: (_ctx.get_height()/2) as f32};
        let x_factor = _ctx.get_width() as f32 / 1280_f32;
        let y_factor = _ctx.get_height() as f32 / 720_f32;
        let mut _factor = 1.0;
        if x_factor < y_factor {
            _factor = x_factor;
        }
        else {
            _factor = y_factor;
        }
        self.transform.set_translation(_ctx.screen_to_world((_ctx.get_width()/2) as i32, (_ctx.get_height()/2) as i32));
        self.transform.set_scale(_factor);
        true
    }

    fn render(&self, _ctx: &mut DrawContext) {
        _ctx.draw(&self.texture, &self.transform);
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { None }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { None }

    fn on_event(&mut self, _event: EventType, _sender: Option<SceneObjectId>) -> bool {
        false
    }

    fn get_z_index(&self) -> i32 { -69 }
}
pub struct Demon {
    texture: AggregatedAnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    distance: f32,
    health: f32,
}

impl Demon {
    fn new(tex: AggregatedAnimatedSprite) -> Demon {
        Demon { texture: tex, transform: Transform::new(), velocity: Vec2{x:0.0,y:0.0}, distance: 2.0, health: 1.0}
    }

    fn set_translation(&mut self, pos: Vec2) {
        self.transform.set_translation(pos);
        self.texture.set_transform(&self.transform);
    }
}

impl GameObject for Demon {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        self.distance -= dt;
        if self.distance < 0.0 {
            self.distance = 0.0;
        }
        self.texture.set_scale( 2.1 - self.distance );
        if self.health < 0.0 {
            _event_mailbox.submit_event(EventType::Attack{damage:1.0}, EventReceiver::Scene);
            _event_mailbox.submit_event(EventType::DeleteMe, EventReceiver::Scene);
        }
        true
    }

    fn render(&self, _ctx: &mut DrawContext) {
        self.texture.draw(_ctx);
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
    last_spawn: f32,
    kills: u8,
    return_to_state: Option<Box<dyn GameState>>,
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
}

impl GameObject for Club {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, _dt: f32) -> bool {
        self.set_translation(_ctx.get_mouse_position().position);
        match _ctx.get_mouse_drag_state() {
            Some(_x) => {
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
        true
    }

    fn render(&self, _ctx: &mut DrawContext) {
        self.texture.draw(_ctx);
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { None }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { None }

    fn on_event(&mut self, _event: EventType, _sender: Option<SceneObjectId>) -> bool {
        false
    }

    fn get_z_index(&self) -> i32 { 69 }
}

impl HellState {
    pub fn new(_ctx: &mut Engine, return_to_state: Box<dyn GameState>) -> Result<Self, Error> {
        let mut scene = Scene::new();

        _ctx.replace_sound(AudioLibrary::Hell, 0, -1)?;
        let tr = _ctx.get_texture_registry();
        let demon_texture = tr.load("assets/images/Demon/Demon.png")?;
        let demon1_texture = tr.load("assets/images/Demon/Demon1.png")?;

        let club_down_texture = tr.load("assets/images/Club/Club_dwn.png")?;
        let club_down_sprite = AnimatedSprite::new(Extent::new(240,240), club_down_texture)?;

        let club_up_texture = tr.load("assets/images/Club/Club_up.png")?;
        let club_up_sprite = AnimatedSprite::new(Extent::new(240,240), club_up_texture)?;

        let mut club_sprite = AggregatedAnimatedSprite::new();
        club_sprite.add(club_up_sprite);
        club_sprite.add(club_down_sprite);

        let club = Club::new( club_sprite );
        scene.add_object(club);
        let background_texture = tr.load("assets/images/Hell.png")?;
        let background = Background::new(background_texture);
        scene.add_object(background);
        let state =
            HellState {
                scene,
                demon_textures: vec![demon_texture, demon1_texture],
                demons: Vec::new(),
                last_spawn: 0.0,
                kills: 0,
                return_to_state: Some(return_to_state),
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
    fn update(mut self: Box<Self>, _ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {

        self.last_spawn += dt;

        while self.last_spawn > 2.0
        {
            self.last_spawn -= 2.0;
            let mut rng = rand::thread_rng();
            let mut demon = Demon::new(self.spawn_demon_sprite());
            let x = rng.gen::<u32>() % _ctx.get_width();
            let y = rng.gen::<u32>() % _ctx.get_height();
            let world_pos = _ctx.screen_to_world(x as i32,y as i32);
            demon.set_translation(world_pos);

            let id = self.scene.add_object(demon);
            self.demons.push(id);
        }
        let events = self.scene.update(_ctx, None, dt);
        for event in events {
            if let EventType::Attack{damage} = event.event_type {
                self.kills +=1;
                println!("{} monster killed!", damage);
                _ctx.play_sound(AudioLibrary::Kill)?;
            }
        }
        if self.kills >=10 ||  _ctx.key_is_down(Keycode::Q) {
            _ctx.reset_sound()?;
            let mut next_state = Some(self.return_to_state.take().unwrap());
            let transition_state = TransitionState::new(self, move |_, _| Ok(next_state.take().unwrap()));
            return Ok(Box::new(transition_state));
        }
        Ok(self)
    }

    fn draw(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        _ctx.set_camera_zoom(1.0);
        self.scene.render(_ctx);

        Ok(())
    }

    fn on_mouse_button_down(&mut self, _ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton)
        -> Result<(), Error>
    {
        Ok(())
    }
}
