use Engine;

use texture_registry::Texture;
use sdl2::rect::Rect;

pub struct Dimmer {
    texture: Texture,
    value: f32,
    target_value: f32
}

impl Dimmer {
    pub fn new(ctx: &mut Engine) -> Dimmer {
        let texture = { ctx.get_texture_registry().load("assets/images/black_gradient_wAlpha.png").unwrap() };
        Dimmer {
            texture: texture,
            value: 1.0,
            target_value: 1.0
        }
    }

    pub fn with_initial_value(mut self, value: f32) -> Dimmer {
        self.value = value;
        self
    }

    pub fn with_target_value(mut self, value: f32) -> Dimmer {
        self.target_value = value;
        self
    }

    pub fn update(&mut self, dt: f32) {
        if self.value < self.target_value {
            self.value = (self.value + dt).min(self.target_value);
        }
        if self.value > self.target_value {
            self.value = (self.value - dt).max(self.target_value);
        }
    }

    pub fn set_target(&mut self, value: f32) {
        self.target_value = value;
    }

    pub fn get_value(&self) -> f32 { self.value }

    pub fn set_value(&mut self, value: f32) {
        self.value = self.value.max(0.0).min(value);
    }

    pub fn draw(&self, ctx: &mut Engine) {
        let screen_bounds = ctx.get_screen_bounds();

        let width = screen_bounds.max.x as u32;

        let w = self.texture.extent().width;
        let offset = ((self.value * w as f32) as i32).min(w).max(0);

        let src = Rect::new(offset, 0, 1, 1);
        let dst = Rect::new(0, 0, width, width);

        ctx.get_draw_context().copy_ex(&self.texture, src, dst);

    }
}
