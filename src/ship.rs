use macroquad::{prelude::*, rand::gen_range};

pub struct Ship {
    pub pos: Vec2,
    pub bullets: Vec<Vec2>,
    pub health: i32,
    pub shield_active: bool,
    pub shield_timer: f64, // Timer for shield duration
    pub texture: Texture2D,
    pub last_shoot_time: f64, // To prevent overlapping shooting sound
}

impl Ship {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            pos: vec2(screen_width() / 2.0, screen_height() - 40.0),
            bullets: vec![],
            health: 3,
            shield_active: false,
            shield_timer: 0.0,
            texture,
            last_shoot_time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32, enemies: &mut Vec<Enemy>) {
        // Handle shield expiration
        if self.shield_active && get_time() > self.shield_timer {
            self.shield_active = false; // Shield ends after timer expires
        }

        // Update movement, ensuring the ship stays within the window bounds
        if is_key_down(KeyCode::Left) && self.pos.x > 0.0 {
            self.pos.x -= 4.0;
        }
        if is_key_down(KeyCode::Right) && self.pos.x < screen_width() - self.texture.width() {
            self.pos.x += 4.0;
        }

        // Shooting logic with a cooldown of 0.5 seconds to prevent overlapping sounds
        if is_key_pressed(KeyCode::Space) && get_time() - self.last_shoot_time > 0.5 {
            self.bullets.push(self.pos);
            self.last_shoot_time = get_time(); // Update last shoot time
        }

        // Move bullets
        for b in &mut self.bullets {
            b.y -= 5.0;
        }
        self.bullets.retain(|b| b.y > 0.0); // Retain bullets that are within the screen

        // Check for collisions with enemy bullets
        for enemy in enemies.iter_mut() {
            // Use a separate variable to track bullets to be removed
            let mut bullets_to_remove = Vec::new();

            for (i, bullet) in enemy.bullets.iter().enumerate() {
                if self.pos.distance(*bullet) < 20.0 { // Collision detected
                    if !self.shield_active {
                        self.health -= 1; // Reduce health if shield is not active
                    }
                    bullets_to_remove.push(i); // Store index of bullets to remove
                }
            }

            // Remove the bullets after checking for collisions
            for index in bullets_to_remove.iter().rev() {
                enemy.bullets.remove(*index); // Remove bullet by index (avoid invalidation)
            }
        }
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.pos.x - self.texture.width() / 2.0, self.pos.y, WHITE);
        for b in &self.bullets {
            draw_circle(b.x, b.y, 2.0, WHITE);
        }
    }

    pub fn activate_shield(&mut self) {
        self.shield_active = true;
        self.shield_timer = get_time() + 5.0; // 5 seconds of invulnerability
    }
}

pub struct Enemy {
    pub pos: Vec2,
    pub dir: f32,
    pub alive: bool,
    pub bullets: Vec<Vec2>,
    pub texture: Texture2D,
    pub rotation: f32, // Rotation for flipping
}

impl Enemy {
    pub fn new(x: f32, y: f32, texture: Texture2D) -> Self {
        let dir = if gen_range(0, 2) == 0 { 1.0 } else { -1.0 };

        // Initialize rotation based on direction
        let rotation = if dir < 0.0 {
            std::f32::consts::PI // Flip texture horizontally
        } else {
            0.0 // No flip
        };

        Self {
            pos: vec2(x, y),
            dir,
            alive: true,
            bullets: vec![],
            texture,
            rotation,
        }
    }

    pub fn update(&mut self) {
        if self.alive {
            // Move the enemy left and right within the screen bounds
            self.pos.x += self.dir;
            if self.pos.x < 10.0 || self.pos.x > screen_width() - 10.0 {
                self.dir *= -1.0; // Reverse direction when hitting the edge
                self.rotation = std::f32::consts::PI; // Flip the enemy ship
            }

            // Fire bullets with a small chance each frame
            if gen_range(0.0, 1.0) < 0.01 {
                self.bullets.push(vec2(self.pos.x, self.pos.y + 10.0));
            }

            // Move enemy bullets
            for b in &mut self.bullets {
                b.y += 4.0;
            }

            // Retain bullets that are within the screen
            self.bullets.retain(|b| b.y < screen_height());
        }
    }

    pub fn draw(&self) {
        if self.alive {
            let scale_x = if self.dir < 0.0 { -1.0 } else { 1.0 };
    
            draw_texture_ex(
                self.texture,
                self.pos.x - self.texture.width() / 2.0,
                self.pos.y,
                WHITE,
                DrawTextureParams {
                    flip_x: scale_x < 0.0,
                    ..Default::default()
                },
            );
        }
    
        for b in &self.bullets {
            draw_circle(b.x, b.y, 2.0, RED);
        }
    }

    pub fn respawn(&mut self) {
        self.alive = true;
        self.pos = vec2(gen_range(20.0, screen_width() - 20.0), gen_range(20.0, 100.0));
        self.rotation = 0.0; // Reset rotation
    }
}
