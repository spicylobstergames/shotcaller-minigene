use crate::*;

/// Adds selected units to SelectedUnits resource and removes dead entities
pub fn unit_selection_system(
    mouse_events: &Vec<MouseEvent>,
    gamemode: &GameMode,
    input_state: &InputState,
    selected_units: &mut SelectedUnits,
) -> SystemResult {

    // Only run in MicroInput game mode
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MircoInput => {}
    }

    // Units should be selected only in default InputState
    match input_state {
        InputState::Default => {},
        _ => return Ok(()),
    }

    'events: for ev in mouse_events.iter() {
        match ev {
            MouseEvent::UnitSelected(e) => {
                // Don't select the unit twice:
                if !selected_units.units.iter().any(|&x| x == *e) {
                    selected_units.units.push(e.clone());

                    // Only select one unit in a frame. 
                    // This is here because sometimes units stack and I don't want to select 5 units with a single click.
                    // TODO: this doesn't work, multiple units still get selected. Maybe I just need to click faster :)
                    break 'events;
                }
            },
            MouseEvent::PositionClicked{
                pos: _,
                contains_entity: false,
            } => {
                // Empty space was clicked. Deselect all
                selected_units.units = vec![];
            },
            _ => {},
        }

        if let MouseEvent::UnitSelected(e) = ev {
            // Don't select the unit twice:
            if !selected_units.units.iter().any(|&x| x == *e) {
                selected_units.units.push(e.clone());
            }
        }
    }
    Ok(())
}
