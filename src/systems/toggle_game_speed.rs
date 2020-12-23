use crate::*;

pub fn 
    toggle_game_speed_system(events: &Vec<InputEvent>,
     speed: &mut GameSpeed) -> SystemResult {
        for k in events.iter() {
            if k == &InputEvent::SpeedToggle {
                if speed.0 == 1.0 {
                    speed.0 = 2.0;
                } else {
                    speed.0 = 1.0;
                }
            }
        }
        Ok(())
    }
