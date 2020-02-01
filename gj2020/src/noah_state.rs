use engine::prelude::*;
use audio_library::AudioLibrary;

pub struct NoahState { }

impl NoahState {
    pub fn new(_ctx: &mut Engine) -> Result<Self, Error> {
        let mut _scene = Scene::new();
        println!("Congratulations you are in the ark");


        let state =
            NoahState {
            };

        ctx.replace_sound(AudioLibrary::Space, 0, -1)?;

        Ok(state)
    }
}

impl GameState for NoahState {
    fn update(self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        let bounds = ctx.get_screen_bounds();

        ctx.get_draw_context().draw_rect(bounds, Color::RGB(0, 55, 55));

        Ok(())
    }
}
