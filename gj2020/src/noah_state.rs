use engine::prelude::*;

pub struct HubState { }

impl HubState {
    pub fn new(_engine: &mut Engine) -> Result<HubState, Error> {
        let hub_state = HubState { };

        Ok(hub_state)
    }
}

impl GameState for HubState {
    fn load_level(ctx: &mut Engine, level_index: i32) -> Result<(Grid2, Grid2, Scene, SceneObjectId), Error> {

        let levels = ["assets/levels/Ark.json"];

        let level = Level::load_from_file(ctx, levels[level_index as usize], 120);

        let low_level = level.ground;
        let mut mid_level = level.objects;

        let mut player = player::Player::new(ctx)?;
        player.get_transform_mut().set_translation(Vec2::from_coords(300.0, 300.0));
        
        let mut scene = Scene::new();

        let alex_in_level = mid_level.take_tile_with_id(*level.special_blocks.get("alex").unwrap());
        for (_, position) in alex_in_level.iter() {
            let mut alex = alex::Alex::new(ctx)?;
            println!("creating alexa at position {:#?}", position);
            alex.get_transform_mut().set_translation(*position);
            scene.add_object(alex);
        }

        let player_id = scene.add_object(player);

        let roombas_in_level = mid_level.take_tile_with_id(*level.special_blocks.get("roomba").unwrap());

        for (_, position) in roombas_in_level.iter() {
            let mut roomba = roomba::Roomba::new(ctx)?;
            roomba.get_transform_mut().set_translation(*position);
            scene.add_object(roomba);
        }

        let dust = mid_level.take_tile_with_id(*level.special_blocks.get("dust").unwrap());

        for (_, position) in dust.iter() {


            let mut key = key::Key::new(ctx)?;
            key.get_transform_mut().set_translation(*position);
            scene.add_object(key);

            let mut dust = dust::Dust::new(ctx)?;
            dust.get_transform_mut().set_translation(*position);
            scene.add_object(dust);
        }

        for (index, (texture, position)) in mid_level.take_tile_with_id(*level.special_blocks.get("door").unwrap()).into_iter().enumerate() {
            let mut door = door::Door::new(ctx, texture);

            if index == 2 {
                door = door.with_key_requirement();
            }
            door.get_transform_mut().set_translation(position);
            scene.add_object(door);
        }

        let fuse_box = mid_level.take_tile_with_id(*level.special_blocks.get("fusebox").unwrap());
        for (_, position) in fuse_box.iter() {
            let mut fuse_box = fuse_box::FuseBox::new(ctx)?;
            fuse_box.get_transform_mut().set_translation(*position);
            scene.add_object(fuse_box);
        }




        Ok((low_level, mid_level, scene, player_id))
    }

    fn update(self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        // TODO: Implement God Hub State
        Ok(self)
    }
    fn draw(&mut self, _ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        // TODO: DRaw God Hub State
        Ok(())
    }
}
