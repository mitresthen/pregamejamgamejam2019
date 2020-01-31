use engine::static_sprite::StaticSprite;
use engine::drawable::{DrawContext, Drawable};
use engine::prelude::*;

use crate::menu_screen::{MenuScreen, MenuChoice};
use crate::main_menu::MainMenuState;

#[derive(Clone)]
pub struct SplashScreen {
    pub background: StaticSprite,
    pub foreground: StaticSprite,
}

impl Drawable for SplashScreen {
    fn draw(&self, ctx: &mut DrawContext) {
        self.background.draw(ctx);
        self.foreground.draw(ctx);
    }
}

pub struct TitleScreenState {
    splash_screen: SplashScreen,
    next_game_state: Option<Box<GameState>>,
}

impl TitleScreenState {
    pub fn new(ctx: &mut Engine) -> Result<Self, Error> {
        // Loading StaticSprites
        let tr = ctx.get_texture_registry();

        let title_background = StaticSprite::new(1280, 720, tr.load("assets/image/title_background.png")?)?;
        let title_sprite = StaticSprite::new(128, 128, tr.load("assets/image/title.png")?)?;

        let title_screen =
            SplashScreen {
                background: title_background,
                foreground: title_sprite,
            };

        let title_screen_state =
            TitleScreenState {
                splash_screen: title_screen,
                next_game_state: None,
            };

        Ok(title_screen_state)
    }
}

impl GameState for TitleScreenState {
    fn update(self: Box<Self>, _ctx: &mut Engine, _dt: f32) -> Result<Box<dyn GameState>, Error> {
        if let Some(state) = self.next_game_state {
            return Ok(state);
        }

        Ok(self)
    }

    fn draw(&mut self, ctx: &mut Engine, _dt: f32) -> Result<(), Error> {
        ctx.draw(&self.splash_screen);
        Ok(())
    }

    fn on_mouse_button_up(&mut self, ctx: &mut Engine, click_x: i32, click_y: i32, _button: MouseButton)
        -> Result<(), Error>
    {
        if self.next_game_state.is_some() {
            return Ok(());
        }

        self.next_game_state = Some(Box::new(MainMenuState::new(ctx)?));

        Ok(())
    }


}

