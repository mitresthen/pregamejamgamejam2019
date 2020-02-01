use engine::prelude::*;

use crate::god::God;

pub struct SnekState {
    level: Level,
    god_id: SceneObjectId,
    scene: Scene
}

impl SnekState {
    pub fn new(ctx: &mut Engine) -> Result<SnekState, Error> {
        let mut level = Level::load_from_file(ctx, "assets/levels/snek.json", 120);

        let mut scene = Scene::new();

        let mut god = God::new(ctx)?;

        let tile_size = 240.0;
        god.set_position(Vec2::from_coords(1.5, 1.5) * tile_size);

        let god_id = scene.add_object(god);

        let snek_state =
            SnekState {
                level,
                god_id,
                scene
            };

        Ok(snek_state)
    }
}

impl GameState for SnekState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        self.scene.update(ctx, Some(&self.level.objects), dt);

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
