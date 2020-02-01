use engine::prelude::*;

pub struct BabylonState { }


impl BabylonState {
    pub fn new(ctx: &mut Engine) -> Result<Self, Error> {
        let state =
            BabylonState {
            };

        Ok(state)
    }
}


impl GameState for BabylonState {
    fn update(mut self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        println!("Congratulations you are in babylon");

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        let bounds = ctx.get_screen_bounds();

        ctx.get_draw_context().draw_rect(bounds, Color::RGB(0, 0, 255));

        Ok(())
    }
}
