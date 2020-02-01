extern crate engine;

use engine::prelude::*;
use std::collections::HashMap;

// Audio Modules
mod audio_library;
use audio_library::AudioLibrary;

// Hub modules
pub mod hub_state;
pub mod god;
pub mod minigame;

// Babylon modules
pub mod babylon_state;

// Noah modules
pub mod noah_state;

struct GodSend { }

impl GameInterface for GodSend {
    fn get_title() -> &'static str {
        "Godsent"
    }

    fn create_starting_state(ctx: &mut Engine)
        -> Result<Box<dyn GameState>, Error>
    {
        let mut sounds = HashMap::new();
        sounds.insert(AudioLibrary::HubWorld, "assets/music/godstheme.wav");
        sounds.insert(AudioLibrary::Hell, "assets/music/hell.wav");
        sounds.insert(AudioLibrary::Babylon, "assets/music/thetowerofbabylon.wav");
        sounds.insert(AudioLibrary::Space, "assets/music/toomuchspacetoolittlelove.wav");
        ctx.load_sounds(sounds)?;

        ctx.reset_sound()?;

        if std::env::var("BABYLON").is_ok() {
            Ok(Box::new(babylon_state::BabylonState::new(ctx)?))
        } else {
            Ok(Box::new(hub_state::HubState::new(ctx)?))
        }
    }
}


fn main() {
    Engine::execute::<GodSend>(1280, 720).unwrap();
}
