use engine::drawable::{DrawContext, Drawable};
use engine::static_sprite::StaticSprite;
use engine::vector::Vec2;


#[derive(Copy, Clone)]
pub enum GameState {
    Gameplay,
    Mainmenu,
    Exit
}

#[derive(Clone)]
pub struct MenuChoice {
    pub name:          String,
    pub target_game_state: GameState,
    pub sprite: StaticSprite,
}

#[derive(Clone)]
pub struct MenuScreen {
    pub name:       String,
    // If transparent - should be ok to use as an overlay as well(eg. for pause screen)
    pub background: StaticSprite,
    pub options:    Vec<MenuChoice>,
}

impl MenuScreen {
    pub fn get_target_from_pos(&self, click: Vec2) -> Option<GameState>
    {
        let mut no = 1.0;
        for choice in &self.options {
            let click_for_options = click - Vec2 { x: 0.0, y: (128.0 * no)};
            if choice.sprite.is_clicked(click_for_options)
            {
                return Some(choice.target_game_state);
            }
            no += 1.0;
        }
        None
    }
}

impl Drawable for MenuScreen {
    fn draw(&self, ctx: &mut DrawContext) {
        self.background.draw(ctx);
        let first_pos = self.background.get_position();
        let mut no = 1.0;
        for choice in &self.options {
            let mut moved_clone = choice.sprite.clone();
            moved_clone.set_position(first_pos.shifted(0.0, 128.0 * no));
            moved_clone.draw(ctx);
            no += 1.0;
        }
    }
}

