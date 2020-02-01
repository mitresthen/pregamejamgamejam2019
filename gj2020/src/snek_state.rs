use engine::prelude::*;

use crate::snek::Snek;

pub struct SnekState {
    level: Level,
    snek_id: SceneObjectId,
    scene: Scene
}

impl SnekState {
    pub fn new(ctx: &mut Engine) -> Result<SnekState, Error> {
        let level = Level::load_from_file(ctx, "assets/levels/snek.json", 120);

        let mut scene = Scene::new();

        let mut snek = Snek::new(ctx)?;

        let tile_size = 240.0;
        snek.set_position(Vec2::from_coords(1.5, 1.5) * tile_size);

        let snek_id = scene.add_object(snek);

        let snek_state =
            SnekState {
                level,
                snek_id,
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

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        let snek_position = self.scene.get(self.snek_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_transform()
            .get_translation();

        ctx.set_camera_position(snek_position);
        // ctx.set_camera_position(Vec2::from_coords(240.0 * 5.0, 240.0 * 3.0));
        ctx.set_camera_zoom(1.0);

        ctx.draw(&self.level.ground);
        ctx.draw(&self.level.objects.interleave_scene(&self.scene));

        Ok(())
    }
}
