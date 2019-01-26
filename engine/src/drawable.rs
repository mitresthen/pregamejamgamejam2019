use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use transform::Transform;
use texture_registry::{Texture, TextureRegistry};
use vector::Vec2;
use extent::Extent;

pub trait Drawable {
    fn draw(&self, ctx: &mut DrawContext);
}

pub struct DrawContext<'t> {
    canvas: &'t mut Canvas<Window>,
    texture_registry: &'t TextureRegistry<'t>,
    camera: &'t Transform
}

impl<'t> DrawContext<'t> {
    pub fn new(
        canvas: &'t mut Canvas<Window>,
        texture_registry: &'t TextureRegistry<'t>,
        camera: &'t Transform
    )
        -> DrawContext<'t>
    {
        DrawContext {
            canvas: canvas,
            texture_registry: texture_registry,
            camera: camera
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

    pub fn draw(&mut self, texture: &Texture, transform: &Transform) {
        let src =
            Rect::new(
                texture.offset().x,
                texture.offset().y,
                texture.extent().width as u32,
                texture.extent().height as u32
            );


        let texture_size =
            Vec2::from_coords(
                texture.extent().width as f32,
                texture.extent().height as f32
            );

        let mut top_left = Vec2::from_coords(-0.5, -0.5) * texture_size;
        let mut bottom_right = Vec2::from_coords(0.5, 0.5) * texture_size;

        top_left = transform.transform_point(top_left);
        bottom_right = transform.transform_point(bottom_right);

        top_left = self.camera.transform_point_inv(top_left);
        bottom_right = self.camera.transform_point_inv(bottom_right);

        let size = bottom_right - top_left;

        let extent = Extent::new(size.x as i32, size.y as i32);

        let dst =
            Rect::new(
                top_left.x as i32,
                top_left.y as i32,
                extent.width as u32,
                extent.height as u32
            );

        self.copy_ex(texture, src, dst);
    }
}
