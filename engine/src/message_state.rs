use crate::prelude::*;

pub enum ProceedMode {
    Click,
    Timer(f32)
}

pub enum Animation {
    FadeAndZoom,
    PopInAndOut
}

pub struct MessageState {
    proceed_mode: ProceedMode,
    // animation_type: Animation,
    next_state: Box<dyn GameState>,
    message_texture: Texture,
    done: bool,
    animation: f32,
    start_transform: Transform,
    target_transform: Transform,
    screen_height: f32,
}

impl MessageState {
    pub fn create(
        ctx: &mut Engine,
        next_state: Box<dyn GameState>,
        // animation_type: Animation,
        proceed_mode: ProceedMode,
        message_image_path: &str
    ) -> Result<Box<dyn GameState>, Error> {
        let tr = ctx.get_texture_registry();
        let message_texture = tr.load(message_image_path)?;

        let screen_height = 1200.0;

        let mut start_transform = Transform::new();
        start_transform.set_translation(Vec2::from_coords(0.0, screen_height));

        let target_transform = Transform::new();

        let message_state =
            MessageState {
                proceed_mode,
                // animation_type,
                message_texture,
                next_state: next_state.update(ctx, 0.0001)?,
                done: false,
                animation: 0.0,
                start_transform,
                target_transform,
                screen_height,
            };

        Ok(Box::new(message_state))
    }

    fn trigger_out_animation(&mut self) {
        if self.done {
            return;
        }

        self.done = true;
        self.animation = 0.0;
        self.start_transform = Transform::new();

        self.target_transform = Transform::new();
        self.target_transform.set_translation(Vec2::from_coords(0.0, self.screen_height));
    }

}

impl GameState for MessageState {
    fn update(mut self: Box<Self>, _ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        self.animation += dt;

        if self.done {
            if self.animation >= 1.0 {
                return Ok(self.next_state);
            }
        } else if let ProceedMode::Timer { .. } = self.proceed_mode {
            if self.animation >= 1.0 {
                self.trigger_out_animation();
            }
        }

        self.animation = self.animation.min(1.0);

        Ok(self)
    }

    fn get_background_color(&self) -> Color {
        self.next_state.get_background_color()
    }

    fn draw(&mut self, ctx: &mut Engine, dt: f32) -> Result<(), Error> {
        let _ignored = self.next_state.draw(ctx, dt);

        let mut draw_ctx = ctx.get_draw_context();
        let ramp = (1.0 - (self.animation * std::f32::consts::PI).cos()) * 0.5;
        draw_ctx.draw(&self.message_texture, &self.start_transform.interpolate(&self.target_transform, ramp));

        Ok(())
    }

    fn on_mouse_button_up(
        &mut self,
        _ctx: &mut Engine,
        _x: i32, _y: i32,
        _button: MouseButton
    ) -> Result<(), Error> {
        if self.animation > 0.999 {
            if let ProceedMode::Click = self.proceed_mode {
                self.trigger_out_animation();
            }
        }

        Ok(())
    }
}
