extern crate engine;

use engine::prelude::*;

pub struct LevelEditorState {
    controller: AxisController,
    zoom: SliderController,
    camera_velocity: Vec2,
    object_index: usize,
    level: Level2D,
    rotation: f32,
    layers_to_draw: Vec<u32>,
    scale: f32
}

impl LevelEditorState {

    fn new(ctx: &mut Engine) -> Result<Self, Error> {
        use std::env;
        let args: Vec<String> = env::args().collect();

        let level_filename = args.iter().nth(1)
            .expect("First argument must be the filename of the level");

        let loaded_level =Level2D::load_from_file(ctx, &level_filename);
        let layers_to_draw = loaded_level.layers_to_draw.clone();

        let level_editor =
            LevelEditorState {
                level: loaded_level,
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
                rotation: 0.0,
                layers_to_draw,
                scale: 1.0
            };

        Ok(level_editor)
    }

}


impl GameState for LevelEditorState {
    fn update(mut self: Box<Self>, _ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error>
    {
        self.camera_velocity = self.controller.poll(_ctx) * 800.0;
        _ctx.move_camera(self.camera_velocity * dt);
        let zoom = self.zoom.poll(_ctx, dt);
        _ctx.set_camera_zoom(zoom);

        Ok(self)
    }

    fn draw(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<(), Error>
    {
        _ctx.draw(&self.level);
        let mut transf: Transform = Transform::new();
        transf.set_translation(_ctx.get_mouse_position().position);
        transf.set_angle(self.rotation);
        transf.set_scale(self.scale);
        let object_filename = &self.level.level_instance.object_types[self.object_index].file;
        _ctx.get_draw_context().draw(&self.level.object_textures[&object_filename.clone()], &transf);

        Ok(())
    }

    fn on_key_down(&mut self, _ctx: &mut Engine, keycode: Keycode, _is_repeated: bool) -> Result<(), Error>
    {
        if keycode == Keycode::C {
            self.object_index = (self.object_index + 1) % self.level.object_textures.len();

            println!("Current object id: {}", self.object_index);
        }

        if keycode == Keycode::R && _ctx.key_is_down(Keycode::LShift) {
            self.rotation = (self.rotation - 0.05);
            if(self.rotation < 0.0) {
                self.rotation = (2.0*3.14);
            }
        } else if keycode == Keycode::R && _ctx.key_is_down(Keycode::LCtrl) {
            self.rotation = (self.rotation + (2.0*3.14/4.0)) % (2.0*3.14);
        }else if keycode == Keycode::R {
            self.rotation = (self.rotation + 0.05) % (2.0*3.14);
        }

        if keycode == Keycode::S && _ctx.key_is_down(Keycode::LShift) {
            self.scale = (self.scale - 0.1).max(0.2);
        } else if keycode == Keycode::S {
            self.scale = (self.scale + 0.1).min(2.0);
        }

        let keycode_num_signed: i32 = (keycode as i32)-48;
        if(keycode_num_signed >= 0) {
            let keycode_num: u32 = (keycode as u32)-48;
            
            if (keycode_num >= 0 && keycode_num <= 9) {
                if(self.layers_to_draw.contains(&keycode_num)) {
                    self.layers_to_draw.retain(|&x| x != keycode_num);
                } else{
                    self.layers_to_draw.push(keycode_num);
                }
                self.level.set_layers_to_draw(self.layers_to_draw.clone());
            }
        }

        Ok(())
    }

    fn on_mouse_button_down(&mut self, _ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton) -> Result<(), Error>
    {
        Ok(())
    }

    fn on_mouse_button_up(&mut self, _ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton) -> Result<(), Error>
    {
        let instance = ObjectInstance {
            object_id: self.object_index as u32,
            position: _ctx.get_mouse_position().position,
            rotation: self.rotation,
            scale: self.scale
        };
        self.level.level_instance.object_instances.push(instance);
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


