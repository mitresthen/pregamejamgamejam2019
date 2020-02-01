extern crate engine;

use engine::prelude::*;

pub struct LevelEditorState {
    controller: AxisController,
    zoom: SliderController,
    camera_velocity: Vec2,
    object_index: u32,
    level: Level2D
}

impl LevelEditorState {

    fn new(ctx: &mut Engine) -> Result<Self, Error> {
        use std::env;
        let args: Vec<String> = env::args().collect();

        let level_filename = args.iter().nth(1)
            .expect("First argument must be the filename of the level");

        let level_editor =
            LevelEditorState {
                level: Level2D::load_from_file(ctx, &level_filename),
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
                object_index: 0,

            };

        Ok(level_editor)
    }

}


impl GameState for LevelEditorState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error>
    {
        self.camera_velocity = self.controller.poll(ctx) * 800.0;
        ctx.move_camera(self.camera_velocity * dt);
        let zoom = self.zoom.poll(ctx, dt);
        ctx.set_camera_zoom(zoom);

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error>
    {


        Ok(())
    }

    fn on_key_down(&mut self, _ctx: &mut Engine, keycode: Keycode, _is_repeated: bool) -> Result<(), Error>
    {
        if keycode == Keycode::C {
            self.object_index = (self.object_index + 1) % 10;

            println!("Current object id: {}", self.object_index);
        }

        Ok(())
    }

    fn on_mouse_button_down(&mut self, _ctx: &mut Engine, x: i32, y: i32, _button: MouseButton) -> Result<(), Error>
    {
        Ok(())
    }

    fn on_mouse_button_up(&mut self, _ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton) -> Result<(), Error>
    {
        Ok(())
    }
}

impl Drop for LevelEditorState {
    fn drop(&mut self) {
        self.level.save();
    }
}

struct LevelEditor2D { }

impl GameInterface for LevelEditor2D {
    fn get_title() -> &'static str { "Level editor" }

    fn create_starting_state(ctx: &mut Engine) -> Result<Box<dyn GameState>, Error> {
        Ok(Box::new(LevelEditorState::new(ctx)?))
    }
}

fn main() {
    Engine::execute::<LevelEditor2D>(1280, 720).unwrap();
}


