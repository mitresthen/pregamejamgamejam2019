extern crate sdl2;
extern crate stb_image;

use std::collections::HashSet;

pub mod audio_engine;
pub mod drawable;
pub mod animated_sprite;
pub mod texture_registry;
pub mod timer;
pub mod vector;

pub mod prelude;

use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;

use audio_engine::WavError;

#[derive(Debug)]
pub enum Error {
    SDLError(String),
    IO { path: Option<String> },
    UnsupportedPixelFormat,
    InvalidTileSize,
    WavError(WavError),
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
    canvas: &'t mut sdl2::render::Canvas<sdl2::video::Window>,
    texture_registry: texture_registry::TextureRegistry<'t>,
    audio_engine: audio_engine::AudioEngine,
    keys_down: HashSet<Keycode>,
    pub paused: bool,
    last_paused_change: timer::Timer,
}

pub trait GameInterface : Sized {
    fn get_title() -> &'static str;

    fn initialize(ctx: &mut Engine) -> Result<Self, Error>;

    fn update(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error>;

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode) -> Result<bool, Error>;
}

impl<'t> Engine<'t> {
    pub fn get_texture_registry(&mut self) -> &mut texture_registry::TextureRegistry<'t> {
        &mut self.texture_registry
    }

    pub fn draw<T: drawable::Drawable>(&mut self, drawable: &T) {
        let mut ctx = drawable::DrawContext::new(&mut self.canvas, &mut self.texture_registry);

        drawable.draw(&mut ctx);
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

    pub fn try_to_change_paused(&mut self) {
        if self.last_paused_change.get_time() > 1.0
        {
            self.paused = !self.paused;
            self.last_paused_change.reset();
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
                texture_registry: texture_registry,
                audio_engine: audio_engine::AudioEngine::new(sdl_context.audio()?),
                keys_down: HashSet::new(),
                paused: false,
                last_paused_change: timer::Timer::new(),
            };

        let mut game = <T as GameInterface>::initialize(&mut engine)?;


        let mut timer = timer::Timer::new();

        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'main_loop;
                    },
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        engine.on_key_down(key);

                        if !game.on_key_down(&mut engine, key)? {
                            break 'main_loop;
                        }
                    },
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => {
                        engine.on_key_up(key);

                        if !game.on_key_down(&mut engine, key)? {
                            break 'main_loop;
                        }
                    },
                    _ => { }
                };
            }

            engine.canvas.clear();
            if !game.update(&mut engine, timer.get_time())? {
                break 'main_loop;
            }

            timer.reset();

            engine.canvas.present();
        }

        Ok(())
    }
}
