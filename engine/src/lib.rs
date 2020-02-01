extern crate sdl2;
extern crate stb_image;
extern crate rand;
extern crate bincode;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate serde;


use std::collections::HashMap;
use std::hash::Hash;
use std::collections::HashSet;

pub mod audio_engine;
pub mod drawable;
pub mod static_sprite;
pub mod animated_sprite;
pub mod texture_registry;
pub mod timer;
pub mod vector;
pub mod rect;
pub mod offset;
pub mod extent;
pub mod transform;
pub mod grid2;
pub mod image;
pub mod game_object;
pub mod scene;
pub mod linear_force;
pub mod radial_force;

pub mod rigid_body;
pub mod physics;

pub mod transition_state;

pub mod axis_controller;
pub mod slider_controller;
pub mod trigger;

pub mod level;
#[allow(non_snake_case)]
pub mod level2D;

pub mod prelude;

pub mod dimmer;
pub mod square_shape;
pub mod ray_shape;

use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::mouse::MouseButton;
pub use sdl2::pixels::Color;

use audio_engine::WavError;

#[derive(Debug, Clone)]
pub enum Error {
    SDLError(String),
    IO { path: Option<String> },
    UnsupportedPixelFormat,
    InvalidTileSize,
    WavError(WavError),
    FatalError(String),
    IncompatiblePixelType,
    IncompletePixel,
    Unknown,
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::SDLError(s)
    }
}

impl From<WavError> for Error {
    fn from(e: WavError) -> Error {
        Error::WavError(e)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MouseDragState {
    pub start: vector::Vec2,
    pub current: vector::Vec2
}

#[derive(Debug, Copy, Clone)]
pub struct MousePosition {
    pub position: vector::Vec2,
}

impl MousePosition {
    pub fn new(new_position: vector::Vec2) -> MousePosition {
        MousePosition {
            position: new_position
        }
    }
}

impl MouseDragState {
    pub fn new(initial: vector::Vec2) -> MouseDragState {
        MouseDragState {
            start: initial,
            current: initial
        }
    }
}

pub trait GameInterface : Sized {
    fn get_title() -> &'static str;

    fn create_starting_state(ctx: &mut Engine) -> Result<Box<dyn GameState>, Error>;
}

pub trait GameState {
    fn update(self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error>;
    fn draw(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<(), Error>;

    fn on_key_down(&mut self, _ctx: &mut Engine, _keycode: Keycode, _is_repeated: bool) -> Result<(), Error> { Ok(()) }
    fn on_key_up(&mut self, _ctx: &mut Engine, _keycode: Keycode) -> Result<(), Error> { Ok(()) }

    fn on_mouse_button_down(&mut self, _ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton) -> Result<(), Error> { Ok(()) }
    fn on_mouse_button_up(&mut self, _ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton) -> Result<(), Error> { Ok(()) }
}

pub struct Engine<'t> {
    pub canvas: &'t mut sdl2::render::Canvas<sdl2::video::Window>,
    width: u32,
    height: u32,
    texture_registry: texture_registry::TextureRegistry<'t>,
    audio_engine: audio_engine::AudioEngine,
    keys_down: HashSet<Keycode>,
    camera: transform::Transform,
    drag_state: Option<MouseDragState>,
    mouse_position: MousePosition
}

impl<'t> Engine<'t> {
    pub fn get_texture_registry(&mut self) -> &mut texture_registry::TextureRegistry<'t> {
        &mut self.texture_registry
    }

    pub fn get_draw_context<'k>(&'k mut self) -> drawable::DrawContext<'k> {
        let bounds = self.get_screen_bounds();

        drawable::DrawContext::new(
            &mut self.canvas,
            &mut self.texture_registry,
            &self.camera,
            bounds
        )
    }

    pub fn draw<T: drawable::Drawable>(&mut self, drawable: &T) {
        let mut ctx = self.get_draw_context();
        drawable.draw(&mut ctx);
    }

    pub fn get_camera(&mut self) -> transform::Transform {
        self.camera.clone()
    }

    pub fn move_camera(&mut self, translation: vector::Vec2) {
        self.camera.translate(translation);
    }

    pub fn set_camera_position(&mut self, p: vector::Vec2) {
        self.camera.set_translation(p);
    }

    pub fn get_camera_position(&self) -> vector::Vec2 {
        self.camera.get_translation()
    }

    pub fn set_camera_zoom(&mut self, value: f32) {
        self.camera.set_scale(value);
    }

    pub fn on_key_down(&mut self, keycode: Keycode) {
        self.keys_down.insert(keycode);
    }

    pub fn on_key_up(&mut self, keycode: Keycode) {
        self.keys_down.remove(&keycode);
    }

    pub fn screen_to_world(&self, x: i32, y: i32) -> vector::Vec2 {
        let mut screen_transform = transform::Transform::new();
        screen_transform.translate(self.get_screen_bounds().max * 0.5);

        let mut p = vector::Vec2::from_coords(x as f32, y as f32);

        p = screen_transform.transform_point_inv(p);
        p = self.camera.transform_point(p);
        p
    }

    pub fn on_mouse_button_down(&mut self, x: i32, y: i32) {
        let p = self.screen_to_world(x, y);
        self.drag_state = Some(MouseDragState::new(p));
    }

    pub fn on_mouse_move(&mut self, x: i32, y: i32) {
        let p = self.screen_to_world(x, y);
        self.mouse_position = MousePosition::new(p);
        if let Some(ref mut drag_state) = &mut self.drag_state {
            drag_state.current = p;
        }
    }

    pub fn on_mouse_button_up(&mut self, _x: i32, _y: i32) {
        self.drag_state = None;
    }

    pub fn get_mouse_drag_state(&self) -> Option<MouseDragState> {
        self.drag_state
    }

    pub fn get_mouse_position(&self) -> MousePosition {
        self.mouse_position
    }

    pub fn key_is_down(&self, keycode: Keycode) -> bool {
        self.keys_down.contains(&keycode)
    }

    pub fn load_sounds<T: Hash + Eq>(&mut self, sounds: HashMap<T, &str>) -> Result<(), Error> {
        Ok(self.audio_engine.pre_load_files(sounds)?)
    }

    pub fn play_sound<T: Hash>(&mut self, key: T) -> Result<usize, Error> {
        Ok(self.audio_engine.play_sound(key)?)
    }

    pub fn prepare_sound<T: Hash>(&mut self, key: T) -> Result<usize, Error> {
        Ok(self.audio_engine.prepare_sound(key)?)
    }
    pub fn loop_sound<T: Hash>(&mut self, key: T, repeats:i32) -> Result<usize, Error> {
        Ok(self.audio_engine.loop_sound(key, repeats)?)
    }

    pub fn reset_sound(&mut self) -> Result<(), Error> {
        Ok(self.audio_engine.reset()?)
    }

    pub fn increase_volume(&mut self) {
        self.audio_engine.increase_volume(0.1);
    }

    pub fn decrease_volume(&mut self) {
        self.audio_engine.decrease_volume(0.1);
    }

    pub fn mute_volume(&mut self) {
        self.audio_engine.mute_volume();
    }

    pub fn unmute_volume(&mut self) {
        self.audio_engine.unmute_volume();
    }

    pub fn toggle_mute(&mut self) {
        self.audio_engine.toggle_mute();
    }

    pub fn set_volume(&mut self, volume: f32, id: usize) {
        self.audio_engine.set_volume(volume, id);
    }

    pub fn pause(&mut self, id: usize) {
        self.audio_engine.pause(id);
    }

    pub fn stop(&mut self, id: usize) {
        self.audio_engine.stop(id);
    }

    pub fn stop_repetition(&mut self, id: usize) {
        self.audio_engine.stop_repetition(id);
    }

    pub fn play(&mut self, id: usize) {
        self.audio_engine.play(id);
    }

    pub fn toggle_pause(&mut self, id: usize) {
        self.audio_engine.toggle_pause(id);
    }

    pub fn is_done(& self, id: usize) -> bool {
        self.audio_engine.is_done(id)
    }

    pub fn is_playing(& self, id: usize) -> bool {
        self.audio_engine.is_playing(id)
    }

    pub fn replace_sound<T: Hash>(&mut self, key: T, id: usize, repeats: i32) -> Result<usize, Error> {
        Ok(self.audio_engine.replace_sound(key, id, repeats)?)
    }

    // TODO: Make it work with moving camera
    pub fn get_screen_bounds(&self) -> rect::Rect2D {
        rect::Rect2D {
            min: vector::Vec2::new(),
            max: vector::Vec2 {
                x: self.width as f32,
                y: self.height as f32
            }
        }
    }

    pub fn get_width(&self) -> u32 { self.width }

    pub fn get_height(&self) -> u32 { self.height }

    pub fn execute<T: GameInterface>(width: u32, height: u32) -> Result<(), Error> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window(<T as GameInterface>::get_title(), width, height)
            .position_centered().opengl().build().map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas()
            .accelerated().build().map_err(|e| e.to_string())?;

        let mut event_pump = sdl_context.event_pump()?;

        let texture_creator = canvas.texture_creator();
        let texture_registry = texture_registry::TextureRegistry::new(&texture_creator);

        let mut engine =
            Engine {
                canvas: &mut canvas,
                width: width,
                height: height,
                texture_registry: texture_registry,
                audio_engine: audio_engine::AudioEngine::new(sdl_context.audio()?),
                keys_down: HashSet::new(),
                camera: transform::Transform::new(),
                drag_state: None,
                mouse_position: MousePosition::new(vector::Vec2{x: 0.0, y: 0.0})
            };

        let mut current_game_state = <T as GameInterface>::create_starting_state(&mut engine)?;

        let mut timer = timer::Timer::new();

        'main_loop: loop {
            let dt = timer.get_time().max(0.0000001);
            timer.reset();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'main_loop;
                    },
                    Event::KeyDown {
                        keycode: Some(key),
                        repeat: is_repeated,
                        ..
                    } => {
                        if key == Keycode::Escape {
                            // Every game wants to quit on escape right?
                            break 'main_loop;
                        }

                        if key == Keycode::F {
                            let curr_fullscreen_state = engine.canvas.window().fullscreen_state();
                            if curr_fullscreen_state != sdl2::video::FullscreenType::True {
                                engine.canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::True).unwrap();
                            }
                            else
                            {
                                engine.canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::Off).unwrap();
                            }
                            let window_size = engine.canvas.window().size();
                            engine.width = window_size.0;
                            engine.height = window_size.1;

                            timer.reset();
                        }
                        engine.on_key_down(key);

                        current_game_state.on_key_down(&mut engine, key, is_repeated)?;
                    },
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        engine.on_key_up(key);

                        current_game_state.on_key_up(&mut engine, key)?;
                    },
                    Event::MouseMotion {
                        x: move_x,
                        y: move_y,
                        ..
                    } => {
                        engine.on_mouse_move(move_x, move_y);

                    },
                    Event::MouseButtonDown {
                        x: click_x,
                        y: click_y,
                        mouse_btn: button,
                        ..
                    } => {
                        engine.on_mouse_button_down(click_x, click_y);

                        current_game_state.on_mouse_button_down(&mut engine, click_x, click_y, button)?;
                    },
                    Event::MouseButtonUp {
                        x: click_x,
                        y: click_y,
                        mouse_btn: button,
                        ..
                    } => {
                        current_game_state.on_mouse_button_up(&mut engine, click_x, click_y, button)?;

                        engine.on_mouse_button_up(click_x, click_y);
                    },
                    _ => { }
                };
            }

            engine.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
            engine.canvas.clear();


            current_game_state = current_game_state.update(&mut engine, dt)?;

            current_game_state.draw(&mut engine, dt)?;

            engine.canvas.present();

            // Limit framerate to 100 fps
            // std::thread::sleep(Duration::from_millis(10));
        }

        Ok(())
    }
}
