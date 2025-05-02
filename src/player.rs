use macroquad::prelude::*;

pub struct Ship {
    pub pos: Vec2,
    pub bullets: Vec<Vec2>,
    pub health: i32,
    pub shield_active: bool,
    pub shield_timer: f64,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            pos: vec2(screen_width() / 2.0, screen_height() - 40.0),
            bullets: vec![],
            health: 3,
            shield_active: false,
            shield_timer: 0.0,
        }
    }

    pub fn update(&mut self, _dt: f32) {
        if self.shield_active && get_time() > self.shield_timer {
            self.shield_active = false;
        }

        if is_key_down(KeyCode::Left) {
            self.pos.x -= 4.0;
        }
        if is_key_down(KeyCode::Right) {
            self.pos.x += 4.0;
        }

        for b in &mut self.bullets {
            b.y -= 5.0;
        }
        self.bullets.retain(|b| b.y > 0.0);
    }

    pub fn draw(&self) {
        let art = ["  ^  ", " /_\\ ", "|_|_|"];
        for (i, line) in art.iter().enumerate() {
            draw_text(
                line,
                self.pos.x - measure_text(line, None, 20, 1.0).width / 2.0,
                self.pos.y + (i as f32 * 15.0),
                20.0,
                if self.shield_active { BLUE } else { WHITE },
            );
        }
        for b in &self.bullets {
            draw_circle(b.x, b.y, 2.0, WHITE);
        }
    }

    pub fn activate_shield(&mut self) {
        self.shield_active = true;
        self.shield_timer = get_time() + 5.0;
    }
}
