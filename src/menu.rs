use macroquad::prelude::*;

pub enum GameState {
    Menu,
    Settings,
    Playing,
    GameOver,
}

pub struct Settings {
    pub spaceship_index: usize, // Player's selected spaceship
    pub ship_changed: bool,     // Flag to indicate if the ship selection has changed
}

impl Settings {
    pub fn new() -> Self {
        Self {
            spaceship_index: 0,
            ship_changed: false,
        }
    }
}

pub fn draw_main_menu(game_state: &mut GameState, settings: &mut Settings) {
    let options = ["START", "Settings", "EXIT"];
    static mut SELECTED: usize = 0;

    draw_text("CowboyBepop", screen_width() / 2.0 - 130.0, 100.0, 60.0, ORANGE);

    for (i, option) in options.iter().enumerate() {
        let color = if unsafe { SELECTED } == i { YELLOW } else { GRAY };
        let text_x = screen_width() / 2.0 - measure_text(option, None, 40, 1.0).width / 2.0;
        let text_y = 200.0 + i as f32 * 60.0;
        draw_text(option, text_x, text_y, 40.0, color);
    }

    if is_key_pressed(KeyCode::Up) {
        unsafe {
            if SELECTED > 0 {
                SELECTED -= 1;
            }
        }
    }

    if is_key_pressed(KeyCode::Down) {
        unsafe {
            if SELECTED < options.len() - 1 {
                SELECTED += 1;
            }
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        match unsafe { SELECTED } {
            0 => {
                *game_state = GameState::Playing;
            }
            1 => {
                *game_state = GameState::Settings;
            }
            2 => {
                std::process::exit(0); // Exit the game
            }
            _ => {}
        }
    }
}

pub fn draw_settings_screen(settings: &mut Settings, game_state: &mut GameState) {
    let options = ["Falcon", "Warlord", "Basic Fighter", "Go Back"];
    draw_text("Choose your Spaceship", screen_width() / 2.0 - 160.0, 100.0, 40.0, ORANGE);

    for (i, option) in options.iter().enumerate() {
        let color = if settings.spaceship_index == i { YELLOW } else { GRAY };
        let text_x = screen_width() / 2.0 - measure_text(option, None, 40, 1.0).width / 2.0;
        let text_y = 200.0 + i as f32 * 60.0;
        draw_text(option, text_x, text_y, 40.0, color);
    }

    if is_key_pressed(KeyCode::Up) {
        if settings.spaceship_index > 0 {
            settings.spaceship_index -= 1;
            // Set flag to indicate settings have changed
            if settings.spaceship_index < 3 { // Only set changed if we're on a ship option
                settings.ship_changed = true;
            }
        }
    }

    if is_key_pressed(KeyCode::Down) {
        if settings.spaceship_index < options.len() - 1 {
            settings.spaceship_index += 1;
            // Set flag to indicate settings have changed
            if settings.spaceship_index < 3 { // Only set changed if we're on a ship option
                settings.ship_changed = true;
            }
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        match settings.spaceship_index {
            0 | 1 | 2 => {
                // Spaceship selected, just mark it as changed
                settings.ship_changed = true;
            }
            3 => {
                *game_state = GameState::Menu; // Go back to the main menu
            }
            _ => {}
        }
    }
}