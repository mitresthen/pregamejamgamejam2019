use Keycode;
use Engine;

pub struct SliderController {
    increase: Keycode,
    decrease: Keycode,
    limits: (f32, f32),
    value: f32
}

impl SliderController {
    pub fn new(increase: Keycode, decrease: Keycode, limits: (f32, f32))
        -> SliderController
    {
        SliderController {
            increase,
            decrease,
            limits,
            value: (limits.0 + limits.1) * 0.5
        }
    }

    pub fn poll(&mut self, ctx: &Engine, step: f32) -> f32 {
        if ctx.key_is_down(self.increase) {
            self.value = (self.value + step).min(self.limits.1);
        }

        if ctx.key_is_down(self.decrease) {
            self.value = (self.value - step).max(self.limits.0);
        }

        self.value
    }
}
