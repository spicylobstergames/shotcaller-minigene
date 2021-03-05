use crate::*;

/// Moves the entity with M-move order towards the destination, provided we have enough action points to do so.
pub fn mmove_order_system(
    entities: &Entities,
    gamemode: &GameMode,
    order_queue: &Components<OrderQueue>,
    targets: &mut Components<AiDestination>,
) -> SystemResult {
    // This system should not run if current gamemode is shotcaller
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MircoInput => {}
    }

    for (e, orders) in join!(&entities && &order_queue) {
        // Current order is moveto point
        let oq = orders.unwrap();
        if oq.orders.len() > 0 {
            if let UnitOrder::MovetoPoint(trg_pt) = oq.orders[0] {
                if let Some(curr_trg) = targets.get(e.unwrap()) {
                    if curr_trg.target != trg_pt {
                        targets.insert(e.unwrap(), AiDestination::new(trg_pt.clone()));
                    }
                }
            }
        }
    }

    Ok(())
}
