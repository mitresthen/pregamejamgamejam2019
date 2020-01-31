use engine::prelude::*;

pub struct HubState { }

impl HubState {
    pub fn new(_engine: &mut Engine) -> Result<HubState, Error> {
        let hub_state = HubState { };

        Ok(hub_state)
    }
}

impl GameState for HubState {
    fn update(self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        // TODO: Implement God Hub State
        Ok(self)
    }
    fn draw(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        // TODO: DRaw God Hub State
        Ok(())
    }
}
