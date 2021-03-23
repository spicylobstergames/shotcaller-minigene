use crate::*;

/// System for micro-input, that converts input events into unit orders.
pub fn order_generation_system(
    // entities: &Entities,
    gamemode: &GameMode,
    mouse_events: &Vec<MouseEvent>,
    input_events: &Vec<InputEvent>,
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
        if let MouseEvent::PositionClicked { pos, entities } = ev {
            match (input_state, entities) {
                (InputState::Default, _) => {}
                (InputState::MMove, None) => {
                    for e in selected_units.units.iter() {
                        if let Some(oq) = order_queue.get_mut(*e) {
                            oq.orders = vec![(UnitOrder::MovetoPoint(*pos))];
                        } else {
                            order_queue
                                .insert(*e, OrderQueue::new(vec![UnitOrder::MovetoPoint(*pos)]));
                        }
                    }
                }
                // follow order:
                (InputState::MMove, Some(trg_e)) => {
                    for e in selected_units.units.iter() {
                        if let Some(oq) = order_queue.get_mut(*e) {
                            oq.orders = vec![UnitOrder::MovetoUnit(trg_e[0].clone())];
                        } else {
                            order_queue.insert(
                                *e,
                                OrderQueue::new(vec![UnitOrder::MovetoUnit(trg_e[0].clone())]),
                            );
                        }
                    }
                }
                (InputState::AMove, _) => {
                    for e in selected_units.units.iter() {
                        if let Some(oq) = order_queue.get_mut(*e) {
                            oq.orders = vec![(UnitOrder::AMovetoPoint(*pos))];
                        } else {
                            order_queue
                                .insert(*e, OrderQueue::new(vec![UnitOrder::AMovetoPoint(*pos)]));
                        }
                    }
                }
            }
        }
    }

    // Some orers are generated without mouse
    for ev in input_events.iter() {
        match ev {
            InputEvent::HoldPos => {
                // currently HoldPosition order works in any input state
                for e in selected_units.units.iter() {
                    if let Some(oq) = order_queue.get_mut(*e) {
                        oq.orders = vec![(UnitOrder::HoldPosition)];
                    } else {
                        order_queue.insert(*e, OrderQueue::new(vec![UnitOrder::HoldPosition]));
                    }
                }
            }
            InputEvent::StopOrder => {
                for e in selected_units.units.iter() {
                    // clears order queue
                    if let Some(oq) = order_queue.get_mut(*e) {
                        oq.orders = vec![];
                    } else {
                        order_queue.insert(*e, OrderQueue::new(vec![]));
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
