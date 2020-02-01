use engine::prelude::*;

use crate::god::God;
use crate::minigame::{
    Minigame,
    MinigameTrigger
};
use crate::babylon_state::BabylonState;
use crate::noah_state::NoahState;

pub struct HubState {
    level: Level,
    scene: Scene,
    _god_id: SceneObjectId,
    babylon_trigger: MinigameTrigger,
    noah_trigger: MinigameTrigger,
}

impl HubState {

    pub fn create_minigame_for_block(ctx: &mut Engine, block_name: &str, level: &mut Level) -> Minigame {
        let tile_id = level.special_blocks.remove(block_name).unwrap();
        let (texture, position) = level.objects.take_tile_with_id(tile_id)
            .into_iter()
            .next()
            .unwrap();


        let minigame =
            Minigame::new(
                ctx,
                texture,
                position,
            );
        minigame
    }

    pub fn new(ctx: &mut Engine) -> Result<HubState, Error> {
        let mut level = Level::load_from_file(ctx, "assets/levels/hub.json", 120);

        let mut scene = Scene::new();

        let babylon_minigame = HubState::create_minigame_for_block(ctx, "babylon", &mut level);
        let babylon_trigger = babylon_minigame.get_trigger();
        scene.add_object(babylon_minigame);

        let noah_minigame = HubState::create_minigame_for_block(ctx, "noah", &mut level);
        let noah_trigger = noah_minigame.get_trigger();
        scene.add_object(noah_minigame);

        let mut god = God::new(ctx)?;

        let tile_size = 240.0;
        god.set_position(Vec2::from_coords(8.0, 4.0) * tile_size);

        let god_id = scene.add_object(god);

        let hub_state =
            HubState {
                level,
                _god_id: god_id,
                scene,
                babylon_trigger,
                noah_trigger
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

        if self.noah_trigger.is_triggered() {
            println!("Going to noah ");
            let noah_state = Box::new(NoahState::new(ctx)?);
            let transition_state = Box::new(TransitionState::new(self, noah_state));
            return Ok(transition_state);
        }


        Ok(self)
    }
    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.set_camera_position(Vec2::from_coords(240.0 * 5.0, 240.0 * 3.0));
        ctx.set_camera_zoom(4.0);
                                                  
        ctx.draw(&self.level.ground);
        ctx.draw(&self.level.objects.interleave_scene(&self.scene));

        Ok(())
    }
}
