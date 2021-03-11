use crate::*;

/// System for micro-input, that converts input events into unit orders.
pub fn order_generation_system(
    // entities: &Entities,
    gamemode: &GameMode,
    mouse_events: &Vec<MouseEvent>,
    selected_units: &SelectedUnits,
    input_state: &InputState,
    order_queue: &mut Components<OrderQueue>,
) -> SystemResult {

    // This system should not run if current gamemode is shotcaller
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MircoInput => {}
    }

    for ev in mouse_events.iter() {
        if let MouseEvent::PositionClicked{
            pos,
            contains_entity:  _} = ev {
            match input_state {
                InputState::Default => {},
                // M-Move needs to be ordered explicitly now 
                InputState::MMove => {
                    for e in selected_units.units.iter() {
                        // order_queue.insert(e, UnitOrder::MovetoPoint(pos));
                        if let Some(oq) = order_queue.get_mut(*e) {
                            oq.orders = vec![(UnitOrder::MovetoPoint(*pos))];
                        }
                        else {
                            order_queue.insert(*e, OrderQueue{orders: vec![UnitOrder::MovetoPoint(*pos)]});
                        }
                    }
                }
            }
        }
    }

    Ok(())
}