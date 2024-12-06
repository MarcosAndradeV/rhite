use prelude::{
    colors, enums::KeyboardKey, is_key_down, DrawRectangleRec, RlRectangle, RlVector2
};

pub trait Entity {
    fn update(&mut self);
    fn draw(&self);
}

pub struct Player {
    rect: RlRectangle,
    speed: f32,
}

impl Player {
    pub fn new(rect: RlRectangle, speed: f32) -> Self {
        Self { rect, speed }
    }
    pub fn get_pos(&self) -> RlVector2 {
        RlVector2::new(self.rect.x, self.rect.y)
    }
}
impl Entity for Player {
    fn draw(&self) {
        unsafe { DrawRectangleRec(
            self.rect.into(),
            colors::RED,
        ) };
    }
    fn update(&mut self) {


        if is_key_down(KeyboardKey::Left) {
            self.rect.x -= self.speed;
        }
        if is_key_down(KeyboardKey::Right) {
            self.rect.x += self.speed;
        }
        if is_key_down(KeyboardKey::Up) {
            self.rect.y -= self.speed;
        }
        if is_key_down(KeyboardKey::Down) {
            self.rect.y += self.speed;
        }
    }

}
