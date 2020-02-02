use crate::prelude::*;

pub struct DecorationObject {
    texture: Texture,
    transform: Transform,
    z_index: i32
}

impl DecorationObject {
    pub fn new(texture: Texture) -> DecorationObject {
        DecorationObject {
            texture,
            transform: Transform::new(),
            z_index: 0
        }
    }

    pub fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }
}

impl GameObject for DecorationObject {
    fn update(&mut self, _ctx: &mut Engine, _event_mailbox: &mut dyn EventMailbox, _dt: f32) -> bool {
        true
    }

    fn render(&self, ctx: &mut DrawContext) {
        ctx.draw(&self.texture, &self.transform);
    }

    fn get_z_index(&self) -> i32 { self.z_index }
}
