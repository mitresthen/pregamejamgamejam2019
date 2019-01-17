use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use texture_registry::{Texture, TextureRegistry};

pub trait Drawable {
    fn draw(&self, ctx: &mut DrawContext);
}

pub struct DrawContext<'t> {
    canvas: &'t mut Canvas<Window>,
    texture_registry: &'t TextureRegistry<'t>
}

impl<'t> DrawContext<'t> {
    pub fn new(canvas: &'t mut Canvas<Window>, texture_registry: &'t TextureRegistry<'t>)
        -> DrawContext<'t>
    {
        DrawContext {
            canvas: canvas,
            texture_registry: texture_registry
        }
    }

    pub fn copy_ex(&mut self, texture: &Texture, src: Rect, dst: Rect) {
        self.canvas.copy_ex(
            self.texture_registry.get_internal_texture(&texture),
            Some(src),
            Some(dst),
            0.0,
            None,
            false,
            false
        ).unwrap();
    }
}
