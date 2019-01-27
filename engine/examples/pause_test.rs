extern crate engine;

use engine::prelude::*;
use std::vec::Vec;

pub struct ExampleGame{
    player_object: MovableObject,
    autonomous_moving_objects: Vec<MovableObject>,
    pause_sprite: StaticSprite,
    title_screen: SplashScreen,
    main_menu_screen: MenuScreen,
}

impl GameInterface for ExampleGame {
    fn get_title() -> &'static str {
        "ExampleGame"
    }

    fn get_title_screen(&self) -> Option<SplashScreen> {
        Some(self.title_screen.clone())
    }

    fn initialize(ctx: &mut Engine) -> Result<Self, Error> {
        let tr = ctx.get_texture_registry();

        let mut title_background = StaticSprite::new(640, 480, tr.load("assets/title_background.png")?)?;

        let mut title_sprite = StaticSprite::new(128, 128, tr.load("assets/title.png")?)?;

        let title_screen =
            SplashScreen {
                background: title_background,
                foreground: title_sprite,
            };

        let mut pause_sprite = StaticSprite::new(128, 64, tr.load("assets/paused.png")?)?;

        let mut sprite = AnimatedSprite::new(Extent::new(32, 32), tr.load("assets/characters.png")?)?;
        sprite.set_scale(4.0);
        sprite.set_position(Vec2::from_coords(100.0, 100.0));

        let mainchar = MovableObject::new(sprite).unwrap();

        let mut game_objects: Vec<MovableObject> = Vec::new();

        let roombatexture = tr.load("assets/characters.png")?;
        let mut roombasprite = AnimatedSprite::new(Extent::new(32, 32), roombatexture)?;
        roombasprite.set_scale(4.0);
        roombasprite.set_position(Vec2::from_coords(100.0, 100.0));

        let roomba = MovableObject::new(roombasprite).unwrap();
        game_objects.push(roomba);

        let mut main_menu_background = StaticSprite::new(640, 480, tr.load("assets/main_menu_background.png")?)?;
        let mut start_game_sprite = StaticSprite::new(128, 64, tr.load("assets/start_button.png")?)?;
        let mut exit_sprite = StaticSprite::new(128, 64, tr.load("assets/exit_button.png")?)?;

        let main_menu_choices =
            [
                MenuChoice
                {
                    name: "Start Adventure".to_string(),
                    target_game_state: GAMEPLAY_STATE,
                    sprite: start_game_sprite,
                    // sprite: pause_sprite.clone(),
                },
                MenuChoice
                {
                    name: "Quit Game".to_string(),
                    target_game_state: EXIT_STATE,
                    sprite: exit_sprite,
                    // sprite: pause_sprite.clone(),
                },
            ].to_vec();

        let main_menu_screen =
            MenuScreen
            {
                name: "Main Menu".to_string(),
                background: main_menu_background,
                options: main_menu_choices,
            };

        let game =
            ExampleGame
            {
                player_object: mainchar,
                autonomous_moving_objects: game_objects,
                title_screen: title_screen,
                main_menu_screen: main_menu_screen,
                pause_sprite: pause_sprite,
            };

        Ok(game)
    }

    fn update_gameplay(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        {
            let speed = 400.0;
            let mut new_speed = Vec2::new();

            if ctx.key_is_down(Keycode::Up) {
                new_speed.y = -speed;
            }
            if ctx.key_is_down(Keycode::Down) {
                new_speed.y = speed;
            }
            if ctx.key_is_down(Keycode::Left) {
                new_speed.x = -speed;
            }
            if ctx.key_is_down(Keycode::Right) {
                new_speed.x = speed;
            }

            self.player_object.set_target_velocity(new_speed);

            self.player_object.update(dt);
        }

        for object in self.autonomous_moving_objects.iter_mut() {
            let player_pos = self.player_object.get_position();
            let speed = 300.0;
            let mut new_speed = Vec2::new();
            let direction = self.player_object.get_position() -object.get_position();
            let velocity_scaling= (direction.len()/speed).abs();
            let target_vel = direction*velocity_scaling;
            object.set_target_velocity(target_vel);
            object.update(dt);
        }

        for object in self.autonomous_moving_objects.iter_mut() {
            let overlap = object.overlaps(self.player_object.bounding_box);
            // println!("Overlap: {:?}", overlap);
        }

        Ok(true)
    }

    fn draw_gameplay(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        {
            ctx.draw(&self.player_object.animated_sprite);
            // println!("Drew char at {}x{}", &self.player_object.get_position().y, &self.player_object.get_position().y)
        }

        for object in self.autonomous_moving_objects.iter_mut() {
            ctx.draw(&object.animated_sprite);
        }

        Ok(true)
    }

    fn draw_main_menu(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        ctx.draw(&self.main_menu_screen);

        Ok(true)
    }

    fn draw_pause_menu(&mut self, ctx: &mut Engine, dt: f32) -> Result<bool, Error> {
        ctx.draw(&self.pause_sprite);

        Ok(true)
    }

    fn on_key_down(&mut self, ctx: &mut Engine, keycode: Keycode, is_repeated: bool) -> Result<bool, Error> {
        if ctx.state.gameplay_displayed
        {
            if keycode == Keycode::P && !is_repeated {
                ctx.invert_paused_state();
                return Ok(true);
            }
        }
        if ctx.state.is_on(TITLE_STATE)
        {
            ctx.end_title_screen();
            return Ok(true);
        }
        if ctx.state.is_on(MAIN_MENU_STATE)
        {
            if ctx.state.go_to(GAMEPLAY_STATE, ctx.last_game_state_change.get_time())
            {
                ctx.last_game_state_change.reset();
            }
            return Ok(true);
        }

        Ok(true)
    }

    fn on_key_up(&mut self, ctx: &mut Engine, keycode: Keycode) -> Result<bool, Error> {
        self.on_key_down(ctx, keycode, true)
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, click_x: i32, click_y: i32, button: MouseButton)
        -> Result<bool, Error>
    {
        if ctx.state.is_on(TITLE_STATE)
        {
            ctx.end_title_screen();
            return Ok(true);
        }
        if ctx.state.is_on(MAIN_MENU_STATE)
        {
            // Click as "visible" in regards to camera.
            let mut cbc = Vec2 {
                x: click_x as f32,
                y: click_y as f32
            };
            let mut screen_transform = Transform::new();
            screen_transform.translate(ctx.get_screen_bounds().max * 0.5);
            cbc = screen_transform.transform_point_inv(cbc);
            cbc = ctx.get_camera().transform_point(cbc);

            match self.main_menu_screen.get_target_from_pos(cbc)
            {
                Some(game_state) => {
                    let gs_clone = game_state.clone();
                    ctx.state.go_to(game_state, ctx.last_game_state_change.get_time());
                    ctx.last_game_state_change.reset();
                    if (gs_clone.is_on(EXIT_STATE)) {
                        return Ok(false);
                    }
                    return Ok(true)
                },
                None => return Ok(true),
            }
        }
        Ok(true)
    }
}

fn main() {
    Engine::execute::<ExampleGame>(1920, 1680).unwrap();
}
