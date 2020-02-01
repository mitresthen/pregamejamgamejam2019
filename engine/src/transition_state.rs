use crate::{
    GameState,
    Engine,
    Error,
    Color
};

pub struct TransitionState {
    source_state: Box<dyn GameState>,
    target_state: Box<dyn GameState>,
    time: f32,
    duration: f32,
}

impl TransitionState {
    pub fn new(source: Box<dyn GameState>, target: Box<dyn GameState>) -> TransitionState {
        TransitionState {
            source_state: source,
            target_state: target,
            time: 0.0,
            duration: 1.0,
        }
    }
}

impl GameState for TransitionState {
    fn update(mut self: Box<Self>, _ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        self.time += dt;

        if self.time >= self.duration {
            return Ok(self.target_state);
        }

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, dt: f32) -> Result<(), Error> {
        let half_duration = self.duration / 2.0;

        let black_alpha = 
            if self.time > half_duration {
                self.target_state.draw(ctx, dt)?;
                1.0 - ((self.time - half_duration) / half_duration)
            } 
            else
            {
                self.source_state.draw(ctx, dt)?;
                self.time / half_duration
            };

        {
            let screen_bounds = ctx.get_screen_bounds();
            let mut draw_context = ctx.get_draw_context();
            draw_context.draw_rect(screen_bounds, Color::RGBA(0, 0, 0, (255 as f32 * black_alpha) as u8));
        }

        Ok(())
    }
}
