extern crate engine;

use engine::prelude::*;

// Hub modules
pub mod hub_state;
pub mod god;
pub mod minigame;

// Babylon modules
pub mod babylon_state;

struct GodSend { }

impl GameInterface for GodSend {
    fn get_title() -> &'static str {
        "God sent"
    }

    fn create_starting_state(ctx: &mut Engine)
        -> Result<Box<dyn GameState>, Error>
    {
        Ok(Box::new(hub_state::HubState::new(ctx)?))
    }
}


fn main() {
    Engine::execute::<GodSend>(1280, 720).unwrap();
}
