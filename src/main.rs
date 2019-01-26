extern crate engine;

use engine::prelude::*;

mod player;

struct GoogleHomeopathicMedicine {
    level: Grid,
    player_id: SceneObjectId,
    scene: Scene,
    zoom_controller: SliderController,
    camera_velocity: Vec2
}

impl GameInterface for GoogleHomeopathicMedicine {
    fn get_title() -> &'static str {
        "Google Homopathic Medicine"
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let level : Image<RGBA> = Image::load("src/resources/image/temp_level.png")?;

        let mut grid = Grid::new(level, 120);

        {
            let tr = ctx.get_texture_registry();

            grid.register_tile_type(
                RGBA { r: 0, g: 0, b: 0, a: 255 },
                tr.load("src/resources/image/tile_Yellow_2.png")?
            );

            grid.register_tile_type(
                RGBA { r: 255, g: 0, b: 0, a: 255 },
                tr.load("src/resources/image/wall_with_dark_top.png")?
            );
            grid.register_tile_type(
                RGBA { r: 254, g: 0, b: 0, a: 255 },
                tr.load("src/resources/image/wall_dark_only.png")?
            );
            grid.register_tile_type(
                RGBA { r: 253, g: 0, b: 0, a: 255 },
                tr.load("src/resources/image/single_dark_tile.png")?
            );
        }

        let player = player::Player::new(ctx)?;

        let mut scene = Scene::new();
        let player_id = scene.add_object(player);

        let game =
            GoogleHomeopathicMedicine {
                level: grid,
                scene: scene,
                player_id: player_id,
                zoom_controller: SliderController::new(
                    Keycode::Plus,
                    Keycode::Minus,
                    (0.5, 2.0)
                ),
                camera_velocity: Vec2::new(),
            };

        Ok(game)
    }

    fn update(&mut self, ctx: &mut Engine, dt: f32)
        -> Result<bool, Error>
    {
        let player_position = self.scene.get(self.player_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_transform()
            .get_translation();

        ctx.set_camera_position(player_position);

        let zoom = self.zoom_controller.poll(&ctx, dt);
        ctx.set_camera_zoom(zoom);

        self.scene.update(ctx, dt);

        ctx.draw(&self.level);

        self.scene.render(ctx);

        let fps = (1.0 / dt) as i32;

        Ok(true)
    }
}

fn main() {
    Engine::execute::<GoogleHomeopathicMedicine>(1280, 720).unwrap();
}
