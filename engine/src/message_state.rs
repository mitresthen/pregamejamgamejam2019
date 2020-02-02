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
    animation: Animation,
    next_state: Box<dyn GameState>,
    message_texture: Texture,
    done: bool,
}

impl MessageState {
    pub fn new(
        ctx: &mut Engine,
        next_state: Box<dyn GameState>,
        animation: Animation,
        proceed_mode: ProceedMode,
        message_image_path: &str
    ) -> Result<Box<dyn GameState>, Error> {
        let tr = ctx.get_texture_registry();
        let message_texture = tr.load(message_image_path)?;

        let message_state =
            MessageState {
                proceed_mode,
                animation,
                message_texture,
                next_state: next_state,//.update(ctx, 0.01)?,
                done: false
            };

        Ok(Box::new(message_state))
    }
}

impl GameState for MessageState {
    fn update(self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        if self.done {
            return Ok(self.next_state);
        }

        Ok(self)
    }

    fn get_background_color(&self) -> Color {
        self.next_state.get_background_color()
    }

    fn draw(&mut self, ctx: &mut Engine, dt: f32) -> Result<(), Error> {
        self.next_state.draw(ctx, dt);

        let transform = Transform::new();
        ctx.set_camera_position(Vec2::new());
        ctx.set_camera_zoom(1.0);

        let mut draw_ctx = ctx.get_draw_context();
        draw_ctx.draw(&self.message_texture, &transform);

        Ok(())
    }

    fn on_mouse_button_up(
        &mut self,
        ctx: &mut Engine,
        x: i32, y: i32,
        button: MouseButton
    ) -> Result<(), Error> {
        match self.proceed_mode {
            ProceedMode::Click => {
                self.done = true;
            },
            _ => { }
        }

        Ok(())
    }
}
