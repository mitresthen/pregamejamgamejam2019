extern crate rand;
use self::rand::Rng;

use std::path::PathBuf;

use engine::prelude::*;
use audio_library::AudioLibrary;

pub struct BabylonState {
    scene: Scene,
    box_texture: Texture,
    hub_state: Option<Box<dyn GameState>>,
}


impl BabylonState {
    pub fn new(ctx: &mut Engine, hub_state: Box<dyn GameState>) -> Result<Self, Error> {
        let mut scene = Scene::new();

        let level = Level2D::load_from_file(ctx, "assets/levels/tower.json");

        let tr = ctx.get_texture_registry();
        let box_texture = tr.load("assets/images/box.png")?;

        let force = LinearForce::new(Vec2::from_coords(0.0, 400.0));
        scene.add_force(force);

        for instance in level.level_instance.object_instances.iter() {
            let object_type = level.level_instance.object_types.get(instance.object_id as usize).unwrap();

            let texture = level.object_textures.get(&object_type.file).unwrap().clone();

            let mut rigid_body = RigidBody::new(texture);
            rigid_body.set_position(instance.position);
            rigid_body.set_angle(instance.rotation);
            rigid_body.set_mass(1.0);
            rigid_body.set_inertia(100.0);
            scene.add_object(rigid_body);

        }
    
        /*
        let ground_texture = tr.load("assets/images/ground.png")?;
        let mut ground = RigidBody::new(ground_texture);
        ground.set_position(Vec2::from_coords(0.0, 1800.0));
        ground.set_scale(8.0);
        scene.add_object(ground);
        */

        ctx.replace_sound(AudioLibrary::Babylon, 0, -1)?;
        let state =
            BabylonState {
                scene,
                box_texture,
                hub_state: Some(hub_state),
            };

        Ok(state)
    }
}


impl GameState for BabylonState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        self.scene.update(ctx, None, dt);

        if ctx.key_is_down(Keycode::Q) {
            let mut hub_state = Some(self.hub_state.take().unwrap());
            let transition_state = TransitionState::new(self, move |_, _| Ok(hub_state.take().unwrap()));
            return Ok(Box::new(transition_state));
        }

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.set_camera_position(Vec2::from_coords(0.0, 0.0));
        ctx.set_camera_zoom(3.0);
        self.scene.render(ctx);

        Ok(())
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, x: i32, y: i32, _button: MouseButton)
        -> Result<(), Error>
    {
        let world_pos = ctx.screen_to_world(x,y);

        let mut rng = rand::thread_rng();

        let mut rigid_body = RigidBody::new(self.box_texture.clone());
        rigid_body.set_position(world_pos);
        rigid_body.set_mass(1.0);
        rigid_body.set_angle(rng.gen::<f32>() * std::f32::consts::PI * 2.0);
        rigid_body.set_inertia(10000.0);
        rigid_body.set_spin(1.0);

        self.scene.add_object(rigid_body);

        Ok(())
    }
}
