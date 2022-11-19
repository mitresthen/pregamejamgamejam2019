use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use transform::Transform;
use texture_registry::{Texture, TextureRegistry};
use vector::Vec2;
use extent::Extent;
use rect::Rect2D;
use sdl2::rect::Point;
use crate::Color;

pub enum Origin {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center
}

impl Origin {
    fn tl(&self) -> Vec2 {
        match *self {
            Origin::TopLeft => Vec2::from_coords(0.0, 0.0),
            Origin::TopRight => Vec2::from_coords(-1.0, 0.0),
            Origin::BottomLeft => Vec2::from_coords(0.0, -1.0),
            Origin::BottomRight => Vec2::from_coords(-1.0, -1.0),
            Origin::Center => Vec2::from_coords(-0.5, -0.5),
        }
    }
    fn br(&self) -> Vec2 {
        Vec2::from_coords(1.0, 1.0) + self.tl()
    }
}

pub trait Drawable {
    fn draw(&self, ctx: &mut DrawContext);
}

pub struct DrawContext<'t> {
    canvas: &'t mut Canvas<Window>,
    texture_registry: &'t TextureRegistry<'t>,
    camera: &'t Transform,
    screen_bounds: Rect2D,
}

impl<'t> DrawContext<'t> {
    pub fn new(
        canvas: &'t mut Canvas<Window>,
        texture_registry: &'t TextureRegistry<'t>,
        camera: &'t Transform,
        screen_bounds: Rect2D
    )
        -> DrawContext<'t>
    {
        DrawContext {
            canvas,
            texture_registry,
            camera,
            screen_bounds
        }
    }


    pub fn copy_ex(&mut self, texture: &Texture, src: Rect, dst: Rect, rotation: f64) {
        self.canvas.copy_ex(
            self.texture_registry.get_internal_texture(texture),
            Some(src),
            Some(dst),
            -rotation,
            None,
            false,
            false
        ).unwrap();
    }

    pub fn draw(&mut self, texture: &Texture, transform: &Transform) {
        self.draw2(texture, transform, Origin::Center);
    }

    pub fn draw_rect(&mut self, dst: Rect2D, color: Color) {
        self.canvas.set_blend_mode(BlendMode::Blend);
        let rect =
            Rect::new(
                dst.min.x as i32,
                dst.min.y as i32,
                dst.max.x as u32,
                dst.max.y as u32
            );

        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
        self.canvas.set_blend_mode(BlendMode::None);
    }

    pub fn draw_point(&mut self, point: Vec2, color: Color) {

        let mut screen_transform = Transform::new();
        screen_transform.translate(self.screen_bounds.max * 0.5);

        let point = self.camera.transform_point_inv(point);
        let point = screen_transform.transform_point(point);

        let point = Point::new(point.x as i32, point.y as i32);

        self.canvas.set_draw_color(color);
        let rect =
            Rect::new(
                point.x - 5,
                point.y - 5,
                10,
                10,
            );

        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn draw2(&mut self, texture: &Texture, transform: &Transform, origin: Origin) {
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

        let mut top_left = origin.tl() * texture_size;
        let mut bottom_right = origin.br() * texture_size;

        let mut transform = transform.clone();
        let angle = transform.get_angle() * 180.0 / std::f32::consts::PI;
        transform.set_angle(0.0);

        top_left = transform.transform_point(top_left);
        bottom_right = transform.transform_point(bottom_right);

        // World space

        top_left = self.camera.transform_point_inv(top_left);
        bottom_right = self.camera.transform_point_inv(bottom_right);

        let mut screen_transform = Transform::new();
        screen_transform.translate(self.screen_bounds.max * 0.5);

        top_left = screen_transform.transform_point(top_left);
        bottom_right = screen_transform.transform_point(bottom_right);

        top_left = top_left.round();
        bottom_right = bottom_right.round();

        let size = bottom_right - top_left;

        let extent = Extent::new(size.x.round() as i32, size.y.round() as i32);

        let dst =
            Rect::new(
                top_left.x as i32,
                top_left.y as i32,
                extent.width as u32,
                extent.height as u32
            );

        self.copy_ex(texture, src, dst, angle as f64);
    }
}
