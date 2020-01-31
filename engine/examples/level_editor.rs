extern crate engine;

use engine::prelude::*;

pub struct LevelEditorState {
    level: Level,
    controller: AxisController,
    zoom: SliderController,
    camera_velocity: Vec2,
    edit_layer: u32,
    painting_tile: Option<u32>
}

impl LevelEditorState {
    pub fn get_edit_layer(&mut self) -> &mut Grid2 {
        if self.edit_layer == 0 {
            &mut self.level.ground
        } else {
            &mut self.level.objects
        }
    }

    fn new(ctx: &mut Engine) -> Result<Self, Error> {
        use std::env;
        let args: Vec<String> = env::args().collect();

        let level_filename = args.iter().nth(1).unwrap();

        let level_editor =
            LevelEditorState {
                level: Level::load_from_file(ctx, &level_filename),
                controller: AxisController::new(
                    Keycode::Up,
                    Keycode::Down,
                    Keycode::Left,
                    Keycode::Right
                ),
                zoom: SliderController::new(
                    Keycode::Minus,
                    Keycode::Plus,
                    (1.0, 4.0)
                ),
                camera_velocity: Vec2::new(),
                edit_layer: 1,
                painting_tile: None
            };

        Ok(level_editor)
    }

}


impl GameState for LevelEditorState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error>
    {
        self.camera_velocity = self.controller.poll(ctx) * 400.0;
        ctx.move_camera(self.camera_velocity * dt);
        let zoom = self.zoom.poll(ctx, dt);
        ctx.set_camera_zoom(zoom);

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error>
    {
        if let Some(drag_state) = ctx.get_mouse_drag_state() {
            if (drag_state.start - drag_state.current).len() > 10.0 {
                let maybe_painting_tile = self.painting_tile;

                let edit_layer : &mut Grid2 = self.get_edit_layer();

                if let Some(painting_tile) = maybe_painting_tile {
                    edit_layer.set_tile_at(drag_state.current, painting_tile).unwrap()
                }
            }
        }

        ctx.draw(&self.level.ground);

        if self.edit_layer == 1 {
            ctx.draw(&self.level.objects);
        }

        Ok(())
    }

    fn on_key_down(&mut self, _ctx: &mut Engine, keycode: Keycode, _is_repeated: bool) -> Result<(), Error>
    {
        if keycode == Keycode::L {
            self.edit_layer = (self.edit_layer + 1) % 2;

            println!("Current edit layer: {}", if self.edit_layer == 0 { "ground" } else { "objects" });
        }

        Ok(())
    }

    fn on_mouse_button_down(&mut self, ctx: &mut Engine, x: i32, y: i32, _button: MouseButton) -> Result<(), Error>
    {
        let tile_index =
            {
                let edit_layer : &mut Grid2 = self.get_edit_layer();
                let p = ctx.screen_to_world(x, y);
                edit_layer.get_tile_at(p)
            };


        self.painting_tile = tile_index;

        Ok(())
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, _x: i32, _y: i32, button: MouseButton) -> Result<(), Error>
    {
        self.painting_tile = None;

        let edit_layer : &mut Grid2 = self.get_edit_layer();

        let step = if button == MouseButton::Left { 1 } else { edit_layer.get_tile_type_count() - 1 };

        if let Some(drag_state) = ctx.get_mouse_drag_state() {
            if (drag_state.start - drag_state.current).len() > 10.0 {
            } else {
                let maybe_tile = edit_layer.get_tile_at(drag_state.current);

                if let Some(mut tile_id) = maybe_tile {
                    tile_id = (tile_id + step) % edit_layer.get_tile_type_count();
                    edit_layer.set_tile_at(drag_state.current, tile_id).unwrap();
                }
            }
        }
        Ok(())
    }
}

struct LevelEditor { }

impl GameInterface for LevelEditor {
    fn get_title() -> &'static str { "Level editor" }

    fn create_starting_state(ctx: &mut Engine) -> Result<Box<dyn GameState>, Error> {
        Ok(Box::new(LevelEditorState::new(ctx)?))
    }
}

fn main() {
    Engine::execute::<LevelEditor>(1280, 720).unwrap();
}

