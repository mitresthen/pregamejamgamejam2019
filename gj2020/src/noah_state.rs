use engine::prelude::*;

pub struct NoahState { }

impl NoahState {
    pub fn new(_engine: &mut Engine) -> Result<NoahState, Error> {
        let hub_state = NoahState { };

        Ok(hub_state)
    }
}

impl GameState for NoahState {
    fn load_level(ctx: &mut Engine, level_index: i32) -> Result<(Grid2, Grid2, Scene, SceneObjectId), Error> {

        let levels = ["assets/levels/Ark.json"];

        let level = Level2D::load_from_file(ctx, levels[level_index as usize], 120);

        let background = level.ground;
        let mut mid_level = level.level_instance.filter(x -> x.Layers.contains(1));

        let mut player = player::Player::new(ctx)?;
        player.get_transform_mut().set_translation(Vec2::from_coords(300.0, 300.0));
        
        let mut scene = Scene::new();

        let player_id = scene.add_object(player);

        Ok((low_level, mid_level, scene, player_id))
    }

    fn update(self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        Ok(self)
    }
    fn draw(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        Ok(())
    }
}
