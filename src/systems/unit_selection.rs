use crate::*;

/// Adds selected units to SelectedUnits resource and removes dead entities
pub fn unit_selection_system(
    gamemode: &GameMode,
    input_state: &InputState,
    mouse_events: &Vec<MouseEvent>,
    selected_units: &mut SelectedUnits,
) -> SystemResult {
    // Only run in MicroInput game mode
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MircoInput => {}
    }

    // Units should be selected only in default InputState
    match input_state {
        InputState::Default => {}
        _ => return Ok(()),
    }

    'events: for ev in mouse_events.iter() {
        match ev {
            MouseEvent::PositionClicked {
                pos: _,
                entities: Some(entities),
            } => {
                for e in entities {
                    if selected_units.units.iter().all(|&x| x != *e) {
                        selected_units.units.push(e.clone());

                        // Only select one unit in a frame.
                        // This is here because sometimes units stack and I don't want to select 5 units with a single click.
                        // TODO: this doesn't work, multiple units still get selected. Maybe I just need to click faster :)
                        break 'events;
                    }
                }
            }
            MouseEvent::PositionClicked {
                pos: _,
                entities: None,
            } => {
                // Empty space was clicked. Deselect all
                selected_units.units = vec![];
            }
            _ => {}
        }
    }

    Ok(())
}

/// Auto-control groups. Press buttons 1-5 to select correspondoing leaders and their summons.
pub fn control_group_system(
    gamemode: &GameMode,
    events: &Vec<InputEvent>,
    leaders: &Components<Leader>,
    companions: &Components<Companion>,
    entities: &Entities,
    selected_units: &mut SelectedUnits,
) -> SystemResult {
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MircoInput => {}
    }

    for k in events.iter() {
        if let InputEvent::AutoSelect(id0) = k {
            // Find entity id of this leader
            // Also find and select all summons
            let mut leader_e: Option<Entity> = None;
            for (e, l) in join!(&entities && &leaders) {
                let Leader(id) = l.unwrap();
                if id0 == id {
                    leader_e = Some(e.unwrap());
                }
            }
            // Select that leader:
            if let Some(e) = leader_e {
                selected_units.units = vec![e];
                // find the summons and companions:
                for (e0, c) in join!(&entities && &companions) {
                    if let Some(Companion::Elephant(summon_e)) = c {
                        if e == e0.unwrap() {
                            selected_units.units.push(summon_e.clone());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
