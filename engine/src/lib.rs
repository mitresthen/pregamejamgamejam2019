extern crate sdl2;
extern crate stb_image;

pub mod audio_engine;
pub mod drawable;
pub mod animated_sprite;
pub mod texture_registry;

pub mod prelude;

use sdl2::event::Event;
use std::time::Duration;
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
}

pub trait GameInterface : Sized {
    fn get_title() -> &'static str;

    fn initialize(ctx: &mut Engine) -> Result<Self, Error>;

    fn update(&mut self, ctx: &mut Engine) -> Result<bool, Error>;

    fn on_key_down(&mut self, keycode: Keycode) -> Result<bool, Error>;
}

impl<'t> Engine<'t> {
    pub fn get_texture_registry(&mut self) -> &mut texture_registry::TextureRegistry<'t> {
        &mut self.texture_registry
    }

    pub fn draw<T: drawable::Drawable>(&mut self, drawable: &T) {
        let mut ctx = drawable::DrawContext::new(&mut self.canvas, &mut self.texture_registry);

        drawable.draw(&mut ctx);
    }

    pub fn play_sound(&mut self, filename: &str) -> Result<(), Error> {
        Ok(self.audio_engine.play_sound_from_file(filename)?)
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
                audio_engine: audio_engine::AudioEngine::new(sdl_context.audio()?)
            };

        let mut game = <T as GameInterface>::initialize(&mut engine)?;

        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'main_loop;
                    },
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => {
                        if !game.on_key_down(key)? {
                            break 'main_loop;
                        }
                    },
                    _ => { }
                };
            }

            engine.canvas.clear();
            if !game.update(&mut engine)? {
                break 'main_loop;
            }

            engine.canvas.present();
            std::thread::sleep(Duration::from_millis(40));
        }

        Ok(())
    }
}
