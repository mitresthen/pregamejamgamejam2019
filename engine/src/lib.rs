extern crate sdl2;
extern crate stb_image;

use std::collections::HashSet;
use std::time::Duration;

pub mod audio_engine;
pub mod drawable;
pub mod static_sprite;
pub mod animated_sprite;
pub mod movable_object;
pub mod texture_registry;
pub mod bounding_box;
pub mod timer;
pub mod vector;
pub mod rect;
pub mod offset;
pub mod extent;
pub mod transform;
pub mod grid;
pub mod image;
pub mod splash_screen;
pub mod menu_screen;
pub mod game_state;
pub mod game_object;
pub mod scene;

pub mod axis_controller;
pub mod slider_controller;


pub mod prelude;

use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;

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

pub struct Engine<'t> {
    pub canvas: &'t mut sdl2::render::Canvas<sdl2::video::Window>,
    width: u32,
    height: u32,
    texture_registry: texture_registry::TextureRegistry<'t>,
    audio_engine: audio_engine::AudioEngine,
    keys_down: HashSet<Keycode>,
    camera: transform::Transform,
    pub state: game_state::GameState,
    pub last_game_state_change : timer::Timer,
}

pub trait GameInterface : Sized {
    fn get_title() -> &'static str;

    fn get_title_screen(&self) -> Option<splash_screen::SplashScreen> { None }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error>;

    // Update - broken down into 2 stages for game engine: update and draw
    fn update_gameplay(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> { Ok(true) }
    fn draw_gameplay(  &mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> { Ok(true) }
    // Optional part of update - drawing pause or main menu
    fn draw_pause_menu(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> { Ok(true) }
    fn draw_main_menu( &mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> { Ok(true) }

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode, is_repeated: bool) -> Result<bool, Error> { Ok(true) }

    fn on_key_up(&mut self, ctx: &mut Engine, keycode: Keycode) -> Result<bool, Error> { Ok(true) }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, x: i32, y: i32) -> Result<bool, Error> { Ok(true) }
}

impl<'t> Engine<'t> {
    pub fn get_texture_registry(&mut self) -> &mut texture_registry::TextureRegistry<'t> {
        &mut self.texture_registry
    }

    pub fn draw<T: drawable::Drawable>(&mut self, drawable: &T) {
        let bounds = self.get_screen_bounds();

        let mut ctx =
            drawable::DrawContext::new(
                &mut self.canvas,
                &mut self.texture_registry,
                &self.camera,
                bounds
            );

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

    pub fn key_is_down(&self, keycode: Keycode) -> bool {
        self.keys_down.contains(&keycode)
    }

    pub fn play_sound(&mut self, filename: &str) -> Result<(), Error> {
        Ok(self.audio_engine.play_sound_from_file(filename)?)
    }
    pub fn loop_sound(&mut self, filename: &str, repeats:i32) -> Result<(), Error> {
        Ok(self.audio_engine.loop_sound_from_file(filename, repeats)?)
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

    pub fn invert_paused_state(&mut self)
    {
        if self.last_game_state_change.get_time() >= 1.0
        {
            self.state.invert_paused_state();
            self.last_game_state_change.reset();
        }
    }

    // End showing the title screen - switch to Main Menu
    pub fn end_title_screen(&mut self) {
        if self.state.go_to(game_state::MAIN_MENU_STATE, self.last_game_state_change.get_time())
        {
            self.last_game_state_change.reset();
        }
    }

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
                state: game_state::TITLE_STATE,
                last_game_state_change: timer::Timer::new(),
            };

        let mut game = <T as GameInterface>::initialize(&mut engine)?;


        let mut timer = timer::Timer::new();

        'main_loop: loop {
            let dt = timer.get_time();
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
                                engine.canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::True);
                            }
                            else
                            {
                                engine.canvas.window_mut().set_fullscreen(sdl2::video::FullscreenType::Off);
                            }
                            let window_size = engine.canvas.window().size();
                            engine.width = window_size.0;
                            engine.height = window_size.1;

                            timer.reset();
                        }
                        engine.on_key_down(key);

                        if !game.on_key_down(&mut engine, key, is_repeated)? {
                            break 'main_loop;
                        }
                    },
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        engine.on_key_up(key);

                        if !game.on_key_up(&mut engine, key)? {
                            break 'main_loop;
                        }
                    },
                    Event::MouseButtonUp {
                        x: click_x,
                        y: click_y,
                        ..
                    } => {
                        if !game.on_mouse_button_up(&mut engine, click_x, click_y)? {
                            break 'main_loop;
                        }
                    },
                    _ => { }
                };
            }

            engine.canvas.clear();

            if engine.state.is_on(game_state::TITLE_STATE)
            {
                let potential_title_screen = game.get_title_screen();
                match potential_title_screen {
                    // Title screen exists - show it.
                    Some(ref title_screen) => engine.draw(title_screen),
                    // No title screen defined - jump to next state.
                    None               => {
                        if engine.state.go_to(game_state::GAMEPLAY_STATE, engine.last_game_state_change.get_time())
                        {
                            engine.last_game_state_change.reset();
                        }
                    },
                    // None               => engine.state.go_to(game_state::MAIN_MENU_STATE, engine.last_game_state_change.get_time()),
                }
            } else {
                if engine.state.gameplay_running
                {
                    if !game.update_gameplay(&mut engine, dt)? {
                        break 'main_loop;
                    }
                }
                if engine.state.gameplay_displayed
                {
                    if !game.draw_gameplay(&mut engine, dt)? {
                        break 'main_loop;
                    }
                }
                if engine.state.presents_menu
                {
                    if engine.state.gameplay_displayed
                    {
                        if !game.draw_pause_menu(&mut engine, dt)? {
                            break 'main_loop;
                        }
                    } else {
                        if !game.draw_main_menu(&mut engine, dt)? {
                            break 'main_loop;
                        }
                    }
                }
            }

            engine.canvas.present();

            // Limit framerate to 100 fps
            // std::thread::sleep(Duration::from_millis(10));
        }

        Ok(())
    }
}
