use crate::*;

/// Changes the game mode using the received `GameModeToggle` event.
pub fn toggle_game_mode_system(events: &Vec<InputEvent>, mode: &mut GameMode) -> SystemResult {
    for k in events.iter() {
        if k == &InputEvent::GameModeToggle {
            if mode == &GameMode::Shotcaller {
                *mode = GameMode::MicroInput;
            } else {
                *mode = GameMode::Shotcaller;
            }
        }
    }
    Ok(())
}
