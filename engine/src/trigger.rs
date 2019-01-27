use Keycode;
use Engine;

pub struct Trigger {
    keycode: Keycode,
    state: bool
}

impl Trigger {
    pub fn new(keycode: Keycode) -> Trigger {
        Trigger { keycode, state: false }
    }
    pub fn poll(&mut self, ctx: &Engine) -> bool {
        let previous_state = self.state;
        self.state = ctx.key_is_down(self.keycode);

        self.state != previous_state && self.state
    }

}
