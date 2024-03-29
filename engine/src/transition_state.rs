use crate::{
    GameState,
    Engine,
    Error,
    Color,
};

type TransitionWithCallback = dyn FnOnce(Box<dyn GameState>, &mut Engine) -> Result<Box<dyn GameState>, Error>;

pub struct TransitionState {
    current_state: Box<dyn GameState>,
    create_target_callback: Option<Box<TransitionWithCallback>>,
    time: f32,
    duration: f32,
}

impl TransitionState {
    pub fn new<F>(source: Box<dyn GameState>, create_target_callback: F) -> TransitionState
        where F: FnOnce(Box<dyn GameState>, &mut Engine) -> Result<Box<dyn GameState>, Error> + 'static
    {
        TransitionState {
            current_state: source,
            create_target_callback: Some(Box::new(create_target_callback)),
            time: 0.0,
            duration: 1.0,
        }
    }
}

impl GameState for TransitionState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {

        let half_duration = self.duration / 2.0;
        if self.time + dt >= half_duration && self.time < half_duration {
            self.current_state = (self.create_target_callback.take().unwrap())(self.current_state, ctx)?;
            self.current_state = self.current_state.update(ctx, dt)?;
        }

        self.time += dt;

        if self.time >= self.duration {
            return Ok(self.current_state);
        }

        Ok(self)
    }

    fn get_background_color(&self) -> Color {
        self.current_state.get_background_color()
    }

    fn draw(&mut self, ctx: &mut Engine, dt: f32) -> Result<(), Error> {
        let half_duration = self.duration / 2.0;

        self.current_state.draw(ctx, dt)?;

        let black_alpha =
            if self.time > half_duration {
                1.0 - ((self.time - half_duration) / half_duration)
            }
            else
            {
                self.time / half_duration
            };

        {
            let screen_bounds = ctx.get_screen_bounds();
            let mut draw_context = ctx.get_draw_context();
            draw_context.draw_rect(screen_bounds, Color::RGBA(0, 0, 0, (255_f32 * black_alpha) as u8));
        }

        Ok(())
    }
}
