extern crate rand;

use engine::prelude::*;
use audio_library::AudioLibrary;

struct SpewBloodData {
    origin: Vec2
}

struct Victim {
    animated_sprite: AnimatedSprite,
    transform: Transform,
    velocity: Vec2,
    direction: f32,
    standing_still: f32,
    standing_position: Vec2,
    shape: Rc<dyn CollisionShape>,
    dead: bool,
    friction: f32,
    outgoing_events: Vec<EventType>,
}

impl Victim {
    pub fn new(ctx: &mut Engine) -> Result<Victim, Error> {
         let tr = ctx.get_texture_registry();
         let texture = tr.load("assets/images/tower/victim.png")?;


         let extent = Extent::new(64, 128);
         let sprite = AnimatedSprite::new(extent, texture)?;

         let size = Vec2::from_coords(extent.width as f32, extent.height as f32);
         let rect = Rect2D::centered_rectangle(size);
         let shape = Rc::new(SquareShape::from_aabb(rect));

         let victim =
             Victim {
                animated_sprite: sprite,
                transform: Transform::new(),
                velocity: Vec2::new(),
                shape,
                direction: 1.0,
                standing_still: 0.0,
                standing_position: Vec2::new(),
                dead: false,
                friction: 0.0,
                outgoing_events: Vec::new(),
             };

         Ok(victim)
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.transform.set_translation(position)
    }

    pub fn kill(&mut self) {
        if ! self.dead {
            self.dead = true;

            let size = self.animated_sprite.get_size();

            let rect =
                Rect2D::new(
                    Vec2::from_coords(-size.x, -size.y * 0.125),
                    Vec2::from_coords(size.x, size.y * 0.125)
                );

            self.shape = Rc::new(SquareShape::from_aabb(rect));
            self.friction = 100.0;

            let data = SpewBloodData { origin: self.transform.get_translation() };

            self.outgoing_events.push(EventType::Custom { data: Rc::new(data) });
        }
    }
}

impl GameObject for Victim {
    fn update(&mut self, ctx: &mut Engine, event_mailbox: &mut dyn EventMailbox, dt: f32) -> bool {
        for event in self.outgoing_events.drain(..) {
            event_mailbox.submit_event(event, EventReceiver::Scene);
        }

        if self.dead {
            self.animated_sprite.set_mode(2);
            self.animated_sprite.set_transform(&self.transform);
            return true;
        }

        let target_velocity = Vec2::from_coords(self.direction * 100.0, 0.0);

        if (self.standing_position - self.transform.get_translation()).len() > 5.0 {
            self.standing_position = self.transform.get_translation();
            self.standing_still = 0.0;
        }

        self.standing_still += dt;

        let bounds = ctx.get_visible_area();

        if self.direction > 0.0 && self.transform.get_translation().x > bounds.max.x {
            self.standing_still = 1000.0;
        }

        if self.direction < 0.0 && self.transform.get_translation().x < bounds.min.x {
            self.standing_still = 1000.0;
        }

        if self.standing_still > 1.0 {
            self.direction = self.direction * -1.0;
            self.standing_still = 0.0;
            println!("Standing still!");
        }

        if self.direction > 0.0 {
            self.animated_sprite.set_mode(0);
        } else {
            self.animated_sprite.set_mode(1);
        }

        self.velocity.approach(target_velocity, dt * 400.0);
        self.animated_sprite.set_transform(&self.transform);
        true
    }

    fn render(&self, ctx: &mut DrawContext) {
        self.animated_sprite.draw(ctx);
    }

    fn get_physical_object(&self) -> Option<&dyn PhysicalObject> { Some(self) }

    fn get_physical_object_mut(&mut self) -> Option<&mut dyn PhysicalObject> { Some(self) }

    fn on_event(&mut self, event: EventType, _sender: Option<SceneObjectId>) -> bool {
        match event {
            EventType::Collide { force } => {
                if force.y > 0.5 {
                    self.kill();
                }

                true
            },
            _ => { false }
        }
    }
}

impl PhysicalObject for Victim {
    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    fn get_velocity(&self) -> &Vec2 {
        &self.velocity
    }

    fn get_velocity_mut(&mut self) -> &mut Vec2 {
        &mut self.velocity
    }

    fn get_collision_shape(&self) -> Option<Rc<dyn CollisionShape>> {
        Some(self.shape.clone())
    }

    fn get_inv_mass(&self) -> f32 { 1.0 }

    fn get_friction(&self) -> f32 { self.friction }

    fn get_src_mask(&self) -> u32 { 1 }

    fn get_dst_mask(&self) -> u32 { 1 }
}

pub struct BabylonState {
    scene: Scene,
    cannon_ball_texture: Texture,
    hub_state: Option<Box<dyn GameState>>,
    blood_texture: Texture,
}


impl BabylonState {
    pub fn new(ctx: &mut Engine, hub_state: Box<dyn GameState>) -> Result<Box<dyn GameState>, Error> {
        let mut scene = Scene::new();

        let level = Level2D::load_from_file(ctx, "assets/levels/tower.json");

        let tr = ctx.get_texture_registry();
        let cannon_ball_texture = tr.load("assets/images/cannon_ball.png")?;
        let blood_texture = tr.load("assets/images/tower/blood.png")?;

        let force = LinearForce::new(Vec2::from_coords(0.0, 400.0));
        scene.add_force(force);

        for instance in level.level_instance.object_instances.iter() {
            let object_type = level.level_instance.object_types.get(instance.object_id as usize).unwrap();

            let texture = level.object_textures.get(&object_type.file).unwrap().clone();

            let mut transform = Transform::new();
            transform.set_translation(instance.position);
            transform.set_angle(instance.rotation);
            transform.set_scale(instance.scale);

            if object_type.layers.contains(&1) {
                let mut rigid_body = RigidBody::new(texture, ShapeFit::Rectangle(1.0));
                rigid_body.set_transform(transform);
                rigid_body.set_friction(100.0);
                rigid_body.set_scale(instance.scale);
                if !object_type.fixed {
                    rigid_body.set_mass(1.0);
                }
                rigid_body.set_inertia(100.0);
                scene.add_object(rigid_body);
            } else {
                let mut object = DecorationObject::new(texture);
                object.set_transform(transform);
                object.set_z_index(-1);
                scene.add_object(object);
            }
        }

        let bounds = ctx.get_visible_area() * 2.0;

        use self::rand::Rng;
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let r = rng.gen::<f32>();
            let x = (r * -1400.0) + ((1.0 - r) * -800.0);

            println!("x = {}", x);

            let mut victim = Victim::new(ctx)?;
            victim.set_position(Vec2::from_coords(x, 540.0));
            scene.add_object(victim);
        }

        for _ in 0..10 {
            let r = rng.gen::<f32>();
            let x = (r * 1400.0) + ((1.0 - r) * 400.0);

            println!("x = {}", x);

            let mut victim = Victim::new(ctx)?;
            victim.set_position(Vec2::from_coords(x, 540.0));
            scene.add_object(victim);
        }


        ctx.replace_sound(AudioLibrary::Babylon, 0, -1)?;
        let state =
            BabylonState {
                scene,
                cannon_ball_texture,
                hub_state: Some(hub_state),
                blood_texture,
            };


        let state = Box::new(state);

        let state =
            MessageState::new(
                ctx,
                state,
                Animation::PopInAndOut,
                ProceedMode::Click, 
                "assets/images/tower/mission_info.png"
            )?;

        Ok(state)
    }

    fn spew_blood(&mut self, origin: Vec2) {
        use self::rand::Rng;
        let mut rng = rand::thread_rng();

        for i in 0..20 {
            let mut rigid_body =
                RigidBody::new(
                    self.blood_texture.clone(),
                    ShapeFit::Rectangle(0.5)
                );

            let speed = 400.0;
            let mut velocity = Vec2::new();
            velocity.x = (rng.gen::<f32>() - 0.5) * speed;
            velocity.y = (rng.gen::<f32>() - 1.0) * speed;

            rigid_body.set_mass(0.01);
            rigid_body.set_inertia(100.0);
            rigid_body.set_position(origin);
            rigid_body.set_velocity(velocity);
            rigid_body.set_dst_mask(5);
            rigid_body.set_src_mask(5);
            self.scene.add_object(rigid_body);
        }
    }
}


impl GameState for BabylonState {
    fn update(mut self: Box<Self>, ctx: &mut Engine, dt: f32) -> Result<Box<dyn GameState>, Error> {
        let events = self.scene.update(ctx, None, dt);

        for event in events.into_iter() {
            match event.event_type {
                EventType::Custom { data } => {
                    if let Ok(data) = data.downcast::<SpewBloodData>() {
                        println!("Got spew blood data");
                        self.spew_blood(data.origin);
                    }
                },
                _ => {
                    println!("Unknown event received");
                }
            }
        }

        if ctx.key_is_down(Keycode::Q) {
            let mut hub_state = Some(self.hub_state.take().unwrap());
            let transition_state = TransitionState::new(self, move |_, _| Ok(hub_state.take().unwrap()));
            return Ok(Box::new(transition_state));
        }

        Ok(self)
    }

    fn get_background_color(&self) -> Color { Color::RGB(215, 224, 255) }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.set_camera_position(Vec2::from_coords(0.0, 0.0));
        ctx.set_camera_zoom(2.0);
        self.scene.render(ctx);

        Ok(())
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, x: i32, y: i32, _button: MouseButton)
        -> Result<(), Error>
    {
        let world_pos = ctx.screen_to_world(x,y);

        let mut rigid_body = RigidBody::new(self.cannon_ball_texture.clone(), ShapeFit::Sphere(0.8));
        let area = ctx.get_visible_area();

        let origin = Vec2::from_coords(area.max.x, area.center().y);
        let velocity = (world_pos - origin) * 0.5;

        rigid_body.set_position(origin);
        rigid_body.set_mass(3.0);
        rigid_body.set_inertia(100.0);
        rigid_body.set_spin(1.0);

        rigid_body.set_velocity(velocity);

        self.scene.add_object(rigid_body);

        Ok(())
    }
}
