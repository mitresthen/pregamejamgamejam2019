use Keycode;
use vector::Vec2;
use Engine;

pub struct AxisController {
    up_key: Keycode,
    down_key: Keycode,
    left_key: Keycode,
    right_key: Keycode
}

impl AxisController {
    pub fn new(up: Keycode, down: Keycode, left: Keycode, right: Keycode)
        -> AxisController
    {
        AxisController {
            up_key: up,
            down_key: down,
            left_key: left,
            right_key: right,
        }
    }

    pub fn poll(&self, ctx: &Engine) -> Vec2 {
        let mut axis = Vec2::new();
        if ctx.key_is_down(self.up_key) {
            axis.y -= 1.0;
        }
        if ctx.key_is_down(self.down_key) {
            axis.y += 1.0;
        }
        if ctx.key_is_down(self.left_key) {
            axis.x -= 1.0;
        }
        if ctx.key_is_down(self.right_key) {
            axis.x += 1.0;
        }
        axis
    }
}
