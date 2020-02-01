use engine::prelude::*;

use crate::god::God;
use crate::minigame::{
    Minigame,
    MinigameTrigger
};
use crate::babylon_state::BabylonState;

pub struct HubState {
    level: Level,
    scene: Scene,
    god_id: SceneObjectId,
    babylon_trigger: MinigameTrigger,
}

impl HubState {
    pub fn new(ctx: &mut Engine) -> Result<HubState, Error> {
        let mut level = Level::load_from_file(ctx, "assets/levels/hub.json", 120);

        let mut scene = Scene::new();

        let babylon_tile_id = level.special_blocks.remove("babylon").unwrap();
        let (babylon_texture, babylon_position) = level.objects.take_tile_with_id(babylon_tile_id)
            .into_iter()
            .next()
            .unwrap();


        let babylon_minigame =
            Minigame::new(
                ctx,
                babylon_texture,
                babylon_position,
            );

        let babylon_trigger = babylon_minigame.get_trigger();

        scene.add_object(babylon_minigame);

        let mut god = God::new(ctx)?;

        let tile_size = 240.0;
        god.set_position(Vec2::from_coords(8.0, 4.0) * tile_size);

        let god_id = scene.add_object(god);

        let hub_state =
            HubState {
                level,
                god_id,
                scene,
                babylon_trigger
            };

        Ok(hub_state)
    }
}

impl GameState for HubState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        self.scene.update(ctx, Some(&self.level.objects), dt);

        if self.babylon_trigger.is_triggered() {
            println!("Going to babylon bitches");
            let babylon_state = Box::new(BabylonState::new(ctx)?);
            let transition_state = Box::new(TransitionState::new(self, babylon_state));
            return Ok(transition_state);
        }

        Ok(self)
    }
    fn draw(&mut self, ctx: &mut Engine, dt: f32) -> Result<(), Error> {
        ctx.set_camera_position(Vec2::from_coords(240.0 * 5.0, 240.0 * 3.0));
        ctx.set_camera_zoom(4.0);
                                                  
        ctx.draw(&self.level.ground);
        ctx.draw(&self.level.objects.interleave_scene(&self.scene));

        Ok(())
    }
}
