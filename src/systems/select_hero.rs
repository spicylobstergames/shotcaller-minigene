use crate::*;

/// Selects a hero in the ui using input events.
pub fn select_hero_system(
    events: &Vec<InputEvent>,
    selected_hero: &mut SelectedLeader,
) -> SystemResult {
    for k in events.iter() {
        match *k {
            InputEvent::MenuNorth => {
                if selected_hero.0 > 0 {
                    selected_hero.0 -= 1;
                }
            }
            InputEvent::MenuSouth => {
                if selected_hero.0 < 4 {
                    selected_hero.0 += 1;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
