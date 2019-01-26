use animated_sprite::AnimatedSprite;
use drawable::{DrawContext, Drawable};

#[derive(Clone)]
pub struct SplashScreen {
    pub background: AnimatedSprite,
    pub foreground: AnimatedSprite,
}

impl Drawable for SplashScreen {
    fn draw(&self, ctx: &mut DrawContext) {
        self.background.draw(ctx);
        self.foreground.draw(ctx);
    }
}

