use macroquad::prelude::*;
use macroquad::audio::*;

use crate::player::Ship;

pub async fn load_audio(path: &str) -> Sound {
    load_sound(path).await.unwrap_or_else(|_| {
        panic!("Audio file not found: {}", path);
    })
}

pub fn handle_player_shoot(player: &mut Ship, sound: &Sound, last_time: &mut f64) {
    if is_key_pressed(KeyCode::Space) && get_time() - *last_time > 0.5 {
        player.bullets.push(player.pos);
        play_sound(*sound, PlaySoundParams { looped: false, volume: 0.2 });
        *last_time = get_time();
    }
}
