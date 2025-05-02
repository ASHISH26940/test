use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub struct PowerUp {
    pub pos: Vec2,
    pub active: bool,
}

impl PowerUp {
    pub fn new() -> Self {
        Self {
            pos: vec2(gen_range(20.0, screen_width() - 20.0), -20.0),
            active: true,
        }
    }

    pub fn update(&mut self) {
        if self.active {
            self.pos.y += 2.0;
            if self.pos.y > screen_height() {
                self.active = false;
            }
        }
    }

    pub fn draw(&self) {
        if self.active {
            draw_text("🛡", self.pos.x, self.pos.y, 30.0, GREEN);
        }
    }
}
