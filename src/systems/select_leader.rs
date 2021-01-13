use crate::*;

/// Selects a leader in the ui using input events.
pub fn select_leader_system(
    events: &Vec<InputEvent>,
    selected_leader: &mut SelectedLeader,
) -> SystemResult {
    for k in events.iter() {
        match *k {
            InputEvent::MenuNorth => {
                if selected_leader.0 > 0 {
                    selected_leader.0 -= 1;
                }
            }
            InputEvent::MenuSouth => {
                if selected_leader.0 < 4 {
                    selected_leader.0 += 1;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
