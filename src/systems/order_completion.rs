use crate::*;

/// Checks if the current order in OrderQueue has been completed and removes it if yes.
pub fn order_completion_check_system(
    entities: &Entities,
    gamemode: &GameMode,
    positions: &Components<Point>,
    order_queue: &mut Components<OrderQueue>,
) -> SystemResult {
    // This system should not run if current gamemode is shotcaller
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MircoInput => {}
    }

    for (e, orders) in join!(&entities && &mut order_queue) {
        let mut is_completed = false;

        let orders = orders.unwrap();

        if orders.orders.len() > 0 {
            match orders.orders[0] {
                UnitOrder::MovetoPoint(trg_pt) => {
                    if positions.get(e.unwrap()).unwrap() == &trg_pt {
                        is_completed = true;
                    }
                }
                UnitOrder::AMovetoPoint(trg_pt) => {
                    // yes this was copy-pasted
                    if positions.get(e.unwrap()).unwrap() == &trg_pt {
                        is_completed = true;
                    }
                }
                UnitOrder::MovetoUnit(trg_entity) => {
                    // Assume that unit with this order has position component
                    let own_pos = positions.get(e.unwrap()).unwrap();
                    // Target entity might be dead, so can't assume it has position entity.
                    let trg_pos = positions.get(trg_entity);
                    if Some(own_pos) == trg_pos {
                        // Unit should be in melee distance to make order completed
                        if dist(own_pos, trg_pos.unwrap()) <= MELEE_LEADER_ATTACK_RADIUS {
                            is_completed = true;
                        }
                    } else {
                        // Should be true if entity no longer exists. Eg. is dead.
                        is_completed = true;
                    }
                }
                UnitOrder::HoldPosition => {
                    // This order only expires on death.
                }
            }
        }

        // If order done, then remove the current order
        if is_completed {
            orders.orders.pop_front();
        }
    }

    Ok(())
}
