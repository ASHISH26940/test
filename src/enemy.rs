use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub struct Enemy {
    pub pos: Vec2,
    pub dir: f32,
    pub alive: bool,
    pub bullets: Vec<Vec2>,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        let dir = if gen_range(0, 2) == 0 { 1.0 } else { -1.0 };
        Self {
            pos: vec2(x, y),
            dir,
            alive: true,
            bullets: vec![],
        }
    }

    pub fn spawn_batch(n: usize) -> Vec<Enemy> {
        (0..n).map(|_| Self::new(gen_range(20.0, screen_width() - 20.0), gen_range(20.0, 100.0))).collect()
    }

    pub fn update(&mut self) {
        if self.alive {
            self.pos.x += self.dir;
            if self.pos.x < 10.0 || self.pos.x > screen_width() - 10.0 {
                self.dir *= -1.0;
            }

            if gen_range(0.0, 1.0) < 0.01 {
                self.bullets.push(vec2(self.pos.x, self.pos.y + 10.0));
            }

            for b in &mut self.bullets {
                b.y += 4.0;
            }
            self.bullets.retain(|b| b.y < screen_height());
        }
    }

    pub fn draw(&self) {
        if self.alive {
            let art = [" /V\\ ", "<o_o>", " |_| "];
            for (i, line) in art.iter().enumerate() {
                draw_text(
                    line,
                    self.pos.x - measure_text(line, None, 20, 1.0).width / 2.0,
                    self.pos.y + (i as f32 * 15.0),
                    20.0,
                    RED,
                );
            }
        }

        for b in &self.bullets {
            draw_circle(b.x, b.y, 2.0, RED);
        }
    }

    pub fn respawn(&mut self) {
        *self = Self::new(gen_range(20.0, screen_width() - 20.0), gen_range(20.0, 100.0));
    }
}
