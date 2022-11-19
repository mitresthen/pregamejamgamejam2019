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
    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl GameObject for Background {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, _dt: f32) -> bool {
        let _factor = _ctx.get_camera().get_scale() * _ctx.get_width() as f32 / 1600_f32;
        let mut transform = Transform::new();
        let translation = _ctx.screen_to_world((_ctx.get_width()/2) as i32, (_ctx.get_height()/2) as i32);
        transform.set_translation(translation);
        transform.set_scale(_factor);
        self.set_transform(transform);
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

pub struct EndState {
    hub_state: Option<Box<dyn GameState>>,
    scene: Scene,
 }

impl EndState {
    pub fn create(_ctx: &mut Engine, hub_state: Box<dyn GameState>, file: &str) -> Result<Box<dyn GameState>, Error>  {
        println!("Welcome to the end state");
        let tr = _ctx.get_texture_registry();
        let background_texture = tr.load(file)?;

        let background = Background::new(background_texture);

        let mut _scene = Scene::new();
        _scene.add_object(background);

        let state =
            EndState {
                hub_state: Some(hub_state),
                scene: _scene
            };
        let state = Box::new(state);

        _ctx.replace_sound(AudioLibrary::Noah, 0, -1)?;

        Ok(state)
    }
}

impl GameState for EndState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        if ctx.key_is_down(Keycode::Q) {
            let mut hub_state = Some(self.hub_state.take().unwrap());
            let transition_state = TransitionState::new(self, move |_, _| Ok(hub_state.take().unwrap()));
            return Ok(Box::new(transition_state));
        }
        self.scene.update(ctx, None, _dt);

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.set_camera_zoom(2.0);
        self.scene.render(ctx);

        Ok(())
    }
}
