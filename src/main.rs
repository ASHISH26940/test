mod game_state;
mod player;
mod enemy;
mod powerup;
mod utils;
mod ship;
mod menu;

use crate::game_state::GameState;
use crate::player::Ship;
use crate::enemy::Enemy;
use crate::powerup::PowerUp;
use crate::utils::{load_audio, handle_player_shoot};
use macroquad::prelude::*;
use macroquad::audio::*;
use macroquad::rand::gen_range;

pub async fn load_texture(path: &str) -> Texture2D {
    let image = load_image(path).await.unwrap();
    Texture2D::from_image(&image)
}

pub async fn load_and_flip_texture(path: &str) -> Texture2D {
    let image = load_image(path).await.unwrap();
    let width = image.width() as usize;
    let height = image.height() as usize;
    let data = image.bytes;
    let mut flipped = vec![0u8; data.len()];

    // Flip 180 degrees (both horizontally and vertically)
    for y in 0..height {
        for x in 0..width {
            let src_index = (y * width + x) * 4;
            let dst_index = ((height - 1 - y) * width + (width - 1 - x)) * 4;
            flipped[dst_index..dst_index + 4].copy_from_slice(&data[src_index..src_index + 4]);
        }
    }

    let flipped_image = Image {
        bytes: flipped,
        width: width as u16,
        height: height as u16,
    };

    Texture2D::from_image(&flipped_image)
}

#[macroquad::main("Space Warriors")]
async fn main() {
    // Load normal ship textures for player
    let falcon_texture = load_texture("assets/falcon.png").await;
    let warlord_texture = load_texture("assets/warlord.png").await;
    let basic_fighter_texture = load_texture("assets/basic.png").await;

    // Load flipped ship textures for enemies (180 degrees rotation)
    let falcon_texture_flipped = load_and_flip_texture("assets/falcon.png").await;
    let warlord_texture_flipped = load_and_flip_texture("assets/warlord.png").await;
    let basic_fighter_texture_flipped = load_and_flip_texture("assets/basic.png").await;

    // Initialize the player ship
    let mut player = ship::Ship::new(falcon_texture.clone());

    // Generate enemies with random flipped textures
    let mut enemies: Vec<ship::Enemy> = (0..5)
        .map(|_| {
            let texture = match gen_range(0, 3) {
                0 => falcon_texture_flipped.clone(),
                1 => warlord_texture_flipped.clone(),
                _ => basic_fighter_texture_flipped.clone(),
            };
            ship::Enemy::new(gen_range(20.0, screen_width() - 20.0), gen_range(20.0, 100.0), texture)
        })
        .collect();

    // Set up the game variables
    let mut score = 0;
    let mut powerup = PowerUp::new();
    let mut game_state = menu::GameState::Menu;
    let mut settings = menu::Settings::new();

    // Load background music
    let bg_music = load_sound("assets/music.ogg").await.unwrap();
    play_sound(bg_music, PlaySoundParams { looped: true, volume: 0.5 });

    loop {
        clear_background(BLACK);

        match game_state {
            menu::GameState::Menu => {
                menu::draw_main_menu(&mut game_state, &mut settings);
            }
            menu::GameState::Settings => {
                menu::draw_settings_screen(&mut settings, &mut game_state);
            }
            menu::GameState::Playing => {
                player.update(get_frame_time(), &mut enemies);
                for enemy in &mut enemies {
                    enemy.update();
                }

                // Bullet vs enemy logic
                for bullet in &player.bullets {
                    for enemy in &mut enemies {
                        if enemy.alive && bullet.distance(enemy.pos) < 12.0 {
                            enemy.alive = false;
                            score += 1;
                            enemy.respawn(); // Respawn the enemy
                        }
                    }
                }

                // Power-up logic
                if powerup.active && player.pos.distance(powerup.pos) < 30.0 {
                    player.activate_shield();
                    powerup.active = false;
                }

                // Draw the player and enemies
                player.draw();
                for e in &enemies {
                    e.draw();
                }

                // Update and draw power-up
                powerup.update();
                powerup.draw();

                // Draw the HUD
                draw_text(&format!("Score: {}", score), 10.0, 30.0, 25.0, WHITE);
                draw_text(&format!("Health: {}", player.health), 10.0, 60.0, 25.0, WHITE);

                if player.health <= 0 {
                    game_state = menu::GameState::GameOver;
                }
            }
            menu::GameState::GameOver => {
                draw_text("GAME OVER", screen_width() / 2.0 - 100.0, screen_height() / 2.0, 40.0, WHITE);
                draw_text(&format!("Final Score: {}", score), screen_width() / 2.0 - 100.0, screen_height() / 2.0 + 50.0, 30.0, WHITE);
                if is_key_pressed(KeyCode::Enter) {
                    game_state = menu::GameState::Menu;
                }
            }
        }

        next_frame().await;
    }
}
