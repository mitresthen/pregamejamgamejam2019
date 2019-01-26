use static_sprite::StaticSprite;
use drawable::{DrawContext, Drawable};

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

