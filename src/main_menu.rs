use crate::menu_screen::{MenuScreen, MenuChoice};
use crate::menu_screen;
use crate::RunningGameState;

use engine::prelude::*;

pub struct MainMenuState {
    main_menu_screen: MenuScreen,
    next_game_state: Option<Box<dyn GameState>>,
}

impl MainMenuState {
    pub fn new(ctx: &mut Engine) -> Result<Self, Error> {
        // Loading StaticSprites
        let tr = ctx.get_texture_registry();

        let main_menu_background = StaticSprite::new(1280, 720, tr.load("assets/images/main_menu_background.png")?)?;
        let start_game_sprite = StaticSprite::new(128, 64, tr.load("assets/images/start_button.png")?)?;
        let exit_sprite = StaticSprite::new(128, 64, tr.load("assets/images/exit_button.png")?)?;

        let main_menu_choices =
            [
                MenuChoice
                {
                    name: "Start Adventure".to_string(),
                    target_game_state: menu_screen::GameState::Gameplay,
                    sprite: start_game_sprite,
                },
                MenuChoice
                {
                    name: "Quit Game".to_string(),
                    target_game_state: menu_screen::GameState::Exit,
                    sprite: exit_sprite,
                },
            ].to_vec();

        let main_menu_screen =
            MenuScreen
            {
                name: "Main Menu".to_string(),
                background: main_menu_background,
                options: main_menu_choices,
            };

        let state =
            MainMenuState {
                main_menu_screen: main_menu_screen,
                next_game_state: None,
            };

        Ok(state)
    }
}

impl GameState for MainMenuState {
    fn update(self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        if let Some(state) = self.next_game_state {
            return Ok(state);
        }

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.draw(&self.main_menu_screen);
        Ok(())
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, click_x: i32, click_y: i32, _button: MouseButton)
        -> Result<(), Error>
    {
        if self.next_game_state.is_some() {
            return Ok(());
        }

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
                match game_state {
                    menu_screen::GameState::Gameplay => {
                        self.next_game_state = Some(Box::new(RunningGameState::new(ctx)?)); 
                    },
                    _ => { }
                }
            },
            _ => { }
        }

        Ok(())
    }
}

