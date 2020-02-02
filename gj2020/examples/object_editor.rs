extern crate engine;

use engine::prelude::*;
use std::collections::HashMap;

struct ObjectEditorState {

}

impl ObjectEditorState {
    pub fn new(ctx: &mut Engine) -> Result<ObjectEditorState, Error> {
        let state =
            ObjectEditorState {
            };

        Ok(state)
    }
}


impl GameState for ObjectEditorState {
    fn update(self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<GameState>, Error> {
        Ok(self)
    }
    fn draw(&mut self, ctx: &mut Engine, dt: f32) -> Result<(), Error> {
        Ok(())
    }
}

struct ObjectEditor { }

impl GameInterface for ObjectEditor {
    fn get_title() -> &'static str {
        "Godsent"
    }

    fn create_starting_state(ctx: &mut Engine)
        -> Result<Box<dyn GameState>, Error>
    {
        Ok(Box::new(ObjectEditorState::new(ctx)?))
    }
}


fn main() {
    Engine::execute::<ObjectEditor>(1280, 720).unwrap();
}

