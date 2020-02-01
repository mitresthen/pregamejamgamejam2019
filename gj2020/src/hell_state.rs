extern crate rand;
use self::rand::Rng;

use engine::prelude::*;
use audio_library::AudioLibrary;

pub struct Demon {
    texture: Texture,
    position: Transform,
    distance: f32,
    health: i32,
}

impl Demon {
    fn new(tex: Texture) -> Demon {
        Demon { texture: tex, position: Transform::new(), distance: 1000.0, health: 1 }
    }

    fn set_translation(&mut self, pos: Vec2) {
        self.position.set_translation(pos);
    }
}

impl GameObject for Demon {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        self.distance -= dt * 100.0;
        if self.distance < 0.0 {
            self.distance = 0.0;
        }
        self.position.set_scale( 0.005 * (1000.0 - self.distance) );
        return true;
    }

    fn render(&self, ctx: &mut DrawContext) {
        ctx.draw(&self.texture, &self.position);
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
}

pub struct HellState {
    scene: Scene,
    demon_texture: Texture,
    demons: Vec<SceneObjectId>,
    club_id : SceneObjectId,
    last_spawn: f32,
}

pub struct Club {
    texture: Texture,
    position: Transform,
}

impl Club {
    fn new(tex: Texture) -> Club {
        let mut club = Club { texture: tex, position: Transform::new() };
        club.position.set_scale(1.0);
        club
    }
    fn set_translation(&mut self, pos: Vec2) {
        self.position.set_translation(pos);
    }
}

impl GameObject for Club {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        self.set_translation(_ctx.get_mouse_position().position);
        return true;
    }

    fn render(&self, ctx: &mut DrawContext) {
        ctx.draw(&self.texture, &self.position);
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
        let demon_texture = tr.load("assets/images/Lion/lion_idle.png")?;
        let club_texture = tr.load("assets/images/Club/Club_up.png")?;

        ctx.replace_sound(AudioLibrary::Hell, 0, -1)?;
        let club = Club::new( club_texture );
        let id = scene.add_object(club);
        let state =
            HellState {
                scene,
                demon_texture,
                demons: Vec::new(),
                club_id: id,
                last_spawn: 0.0,
            };

        Ok(state)
    }
}


impl GameState for HellState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {

        self.last_spawn += dt;

        while self.last_spawn > 2.0
        {
            self.last_spawn -= 2.0;
            let mut rng = rand::thread_rng();
            let mut demon = Demon::new(self.demon_texture.clone());
            let bounds = ctx.get_screen_bounds();
            let x = rng.gen::<u32>() % ctx.get_width();
            let y = rng.gen::<u32>() % ctx.get_height();
            let world_pos = ctx.screen_to_world(x as i32,y as i32);
            demon.set_translation(world_pos);

            let id = self.scene.add_object(demon);
            self.demons.push(id);
        }
        self.scene.update(ctx, None, dt);
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
