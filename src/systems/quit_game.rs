use crate::*;

/// In the case we receive the `Quit` event, update the `QuitGame` resource.
pub fn quit_game_system(events: &Vec<InputEvent>, quit: &mut QuitGame) -> SystemResult {
    for k in events.iter() {
        if k == &InputEvent::Quit {
            quit.0 = true;
        }
    }
    Ok(())
}
