use crate::*;

/// Changes the game speed using the received `SpeedToggle` event.
pub fn toggle_game_speed_system(events: &Vec<InputEvent>, time: &mut Time) -> SystemResult {
    for k in events.iter() {
        if k == &InputEvent::SpeedToggle {
            if time.time_scale() == 1.0 {
                // TODO toggle back to 2.0
                //speed.0 = 2.0;
                time.set_time_scale(0.0);
            } else {
                time.set_time_scale(1.0);
            }
        }
    }
    Ok(())
}
