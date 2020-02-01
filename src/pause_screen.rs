use crate::menu_screen::{MenuScreen, MenuChoice};
use engine::prelude::*;

use crate::menu_screen;
use crate::RunningGameState;
use crate::main_menu::MainMenuState;

pub struct PauseScreenState {
    pause_screen: MenuScreen,
    running_state: Box<RunningGameState>,
    goto_main_menu: bool,
    resume: bool,
}

impl PauseScreenState {
    pub fn new(ctx: &mut Engine, running_state: Box<RunningGameState>) -> Result<Self, Error> {
        // Loading StaticSprites
        let tr = ctx.get_texture_registry();

        let pause_menu_background = StaticSprite::new(1280, 720, tr.load("assets/images/pause_menu_background.png")?)?;
        let continue_sprite = StaticSprite::new(128, 64, tr.load("assets/images/continue_button.png")?)?;
        let return_to_menu_sprite = StaticSprite::new(128, 64, tr.load("assets/images/return_to_menu_button.png")?)?;

        let pause_menu_choices =
            [
                MenuChoice
                {
                    name: "Continue Adventure".to_string(),
                    target_game_state: menu_screen::GameState::Gameplay,
                    sprite: continue_sprite,
                },
                MenuChoice
                {
                    name: "Return to Main Menu".to_string(),
                    target_game_state: menu_screen::GameState::Mainmenu,
                    sprite: return_to_menu_sprite,
                },
            ].to_vec();

        let pause_screen =
            MenuScreen
            {
                name: "Pause Menu".to_string(),
                background: pause_menu_background,
                options: pause_menu_choices,
            };


        let pause_screen_state =
            PauseScreenState {
                pause_screen,
                running_state,
                resume: false,
                goto_main_menu: false,
            };

        Ok(pause_screen_state)
    }
}

impl GameState for PauseScreenState {
    fn update(self: Box<Self>, ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        if self.resume {
            return Ok(self.running_state);
        }

        if self.goto_main_menu {
            return Ok(Box::new(MainMenuState::new(ctx)?));
        }

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.set_camera_position(Vec2::from_coords(0.0, 0.0));
        ctx.set_camera_zoom(1.0);
        ctx.draw(&self.pause_screen);
        Ok(())
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, click_x: i32, click_y: i32, _button: MouseButton)
        -> Result<(), Error>
    {
        if self.resume || self.goto_main_menu {
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

        match self.pause_screen.get_target_from_pos(cbc)
        {
            Some(game_state) => {
                match game_state {
                    menu_screen::GameState::Gameplay => {
                        self.resume = true;
                    },
                    menu_screen::GameState::Mainmenu => {
                        self.goto_main_menu = true;
                    }
                    _ => { }
                }
            },
            _ => { }
        }

        Ok(())
    }
}


