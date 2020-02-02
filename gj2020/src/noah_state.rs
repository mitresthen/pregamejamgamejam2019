use engine::prelude::*;
use audio_library::AudioLibrary;
use crate::noah::Noah;
use crate::plank::Plank;
use crate::ladder::Ladder;
use crate::ocean::Ocean;

pub struct NoahState {
    level: Level2D,
    scene: Scene,
    noah_id: SceneObjectId,
    sea_level: f32,
    ocean_id: SceneObjectId,
    broken_planks: u32,
    total_planks: u32
 }

impl NoahState {
    pub fn new(_ctx: &mut Engine) -> Result<Self, Error> {
        let level = Level2D::load_from_file(_ctx, "assets/levels/Ark4.json");
        let mut _scene = Scene::new();
        println!("Welcome to the ark");

        let force = LinearForce::new(Vec2::from_coords(0.0, 400.0));
        _scene.add_force(force);

        let mut noah = Noah::new(_ctx)?;
        noah.set_scale(0.4);
        noah.set_position(Vec2::from_coords(300.0, 300.0));

        let noah_id = _scene.add_object(noah);

        let planks: Vec<_> = level.level_instance.object_instances.iter()
            .filter(|x| level.level_instance.object_types[x.object_id as usize].file == "Plank.png").collect();

        let plank_count = planks.len();

        for plank in planks {
            let mut new_plank = Plank::new(_ctx)?;
            let plank_transform: Transform = Transform {
                translation: plank.position,
                scale: plank.scale,
                angle: plank.rotation
            };
            new_plank.set_transform(plank_transform);
            _scene.add_object(new_plank);
        }

        let floor = level.level_instance.object_instances.iter()
        .filter(|x| level.level_instance.object_types[x.object_id as usize].file == "DarkPlank.png");

        for floor_plank in floor {
            let texture = level.object_textures.get("DarkPlank.png").unwrap().clone();

            let mut rigid_body = RigidBody::new(texture, ShapeFit::Rectangle(1.0));
            rigid_body.set_position(floor_plank.position);
            rigid_body.set_angle(floor_plank.rotation);
            rigid_body.set_friction(0.5);
            rigid_body.set_inertia(100.0);
            _scene.add_object(rigid_body);
        }

        let ladders = level.level_instance.object_instances.iter()
        .filter(|x| level.level_instance.object_types[x.object_id as usize].file == "ladder.png");
        
        for ladder in ladders {
            let mut new_ladder = Ladder::new(_ctx)?;

            let ladder_transform: Transform = Transform {
                translation: ladder.position,
                scale: ladder.scale,
                angle: ladder.rotation
            };
            new_ladder.set_transform(ladder_transform);
            _scene.add_object(new_ladder);
        }

        let mut ocean = Ocean::new(_ctx)?;
        let ocean_id = _scene.add_object(ocean);

        let state =
            NoahState {
                level,
                scene: _scene,
                noah_id,
                sea_level: 226.0,
                ocean_id,
                broken_planks: 0,
                total_planks: plank_count as u32
            };

        _ctx.replace_sound(AudioLibrary::Noah, 0, -1)?;

        Ok(state)
    }
}

impl GameState for NoahState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        let events = self.scene.update(ctx, None, _dt);
        for event in events {
            match event.event_type {
                EventType::PlankBroke => {
                    self.broken_planks += 1;
                },
                EventType::PlankRepaired => {
                    self.broken_planks -= 1;
                },
                _ => {}
            }
        }
        println!("Broken planks {}, total planks {}", self.broken_planks, self.total_planks);


        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.set_camera_zoom(2.0);
        let noah_position = self.scene.get(self.noah_id)
            .unwrap()
            .get_physical_object()
            .unwrap()
            .get_transform()
            .get_translation();

        ctx.set_camera_position(noah_position);
        let bounds = ctx.get_screen_bounds();

        ctx.get_draw_context().draw_rect(bounds, Color::RGB(0, 55, 55));

        ctx.draw(&self.level);
        self.scene.render(ctx);

        //ctx.get_draw_context().draw_rect(ocean_bounds, Color::RGBA(0, 0, 166, 150));

        Ok(())
    }
}
