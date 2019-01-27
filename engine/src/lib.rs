extern crate sdl2;
extern crate stb_image;
extern crate rand;
extern crate bincode;
#[macro_use]
extern crate serde_derive;
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
pub mod grid2;
pub mod image;
pub mod splash_screen;
pub mod menu_screen;
pub mod game_state;
pub mod game_object;
pub mod scene;

pub mod axis_controller;
pub mod slider_controller;
pub mod trigger;

pub mod sat_collider;

pub mod level;

pub mod prelude;

use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::mouse::MouseButton;

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

impl MouseDragState {
    pub fn new(initial: vector::Vec2) -> MouseDragState {
        MouseDragState {
            start: initial,
            current: initial
        }
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
    drag_state: Option<MouseDragState>,
}

pub trait GameInterface : Sized {
    fn get_title() -> &'static str;

    fn get_title_screen(&self) -> Option<splash_screen::SplashScreen> { None }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error>;

    // Update - broken down into 2 stages for game engine: update and draw
    fn update_gameplay(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<bool, Error> { Ok(true) }
    fn draw_gameplay(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<bool, Error> { Ok(true) }

    // Optional part of update - drawing pause or main menu
    fn draw_pause_menu(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<bool, Error> { Ok(true) }
    fn draw_main_menu(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<bool, Error> { Ok(true) }

    fn on_key_down(&mut self, _ctx: &mut Engine, _keycode: Keycode, _is_repeated: bool) -> Result<bool, Error> { Ok(true) }
    fn on_key_up(&mut self, _ctx: &mut Engine, _keycode: Keycode) -> Result<bool, Error> { Ok(true) }

    fn on_mouse_button_down(&mut self, _ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton) -> Result<bool, Error> { Ok(true) }
    fn on_mouse_button_up(&mut self, _ctx: &mut Engine, _x: i32, _y: i32, _button: MouseButton) -> Result<bool, Error> { Ok(true) }

    fn on_exit(&mut self) { }
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


    pub fn key_is_down(&self, keycode: Keycode) -> bool {
        self.keys_down.contains(&keycode)
    }

    pub fn load_sounds<T: Hash + Eq>(&mut self, sounds: HashMap<T, &str>) -> Result<(), Error> {
        Ok(self.audio_engine.pre_load_files(sounds)?)
    }

    pub fn play_sound<T: Hash>(&mut self, key: T) -> Result<(), Error> {
        Ok(self.audio_engine.play_sound(key)?)
    }
    pub fn loop_sound<T: Hash>(&mut self, key: T, repeats:i32) -> Result<(), Error> {
        Ok(self.audio_engine.loop_sound(key, repeats)?)
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

        let starting_state =
            if std::env::var("QUICK_START").is_ok() {
                game_state::GAMEPLAY_STATE
            } else {
                game_state::TITLE_STATE
            };

        let mut engine =
            Engine {
                canvas: &mut canvas,
                width: width,
                height: height,
                texture_registry: texture_registry,
                audio_engine: audio_engine::AudioEngine::new(sdl_context.audio()?),
                keys_down: HashSet::new(),
                camera: transform::Transform::new(),
                state: starting_state,
                last_game_state_change: timer::Timer::new(),
                drag_state: None,
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

                        if !game.on_mouse_button_down(&mut engine, click_x, click_y, button)? {
                            break 'main_loop;
                        }
                    },
                    Event::MouseButtonUp {
                        x: click_x,
                        y: click_y,
                        mouse_btn: button,
                        ..
                    } => {
                        if !game.on_mouse_button_up(&mut engine, click_x, click_y, button)? {
                            break 'main_loop;
                        }

                        engine.on_mouse_button_up(click_x, click_y);
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
            } else if engine.state.is_on(game_state::RESET_GAME) {
                engine.camera = transform::Transform::new();
                game = <T as GameInterface>::initialize(&mut engine)?;
                if engine.state.go_to(game_state::MAIN_MENU_STATE, 60.0)
                {
                    engine.last_game_state_change.reset();
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

        game.on_exit();

        Ok(())
    }
}
