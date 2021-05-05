use crate::*;

/// unit with hold-position order should stand still even if standing in acid or lava
pub fn holdpos_order_system(
    entities: &Entities,
    gamemode: &GameMode,
    order_queue: &Components<OrderQueue>,
    positions: &Components<Point>,
    targets: &mut Components<AiDestination>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    // This system should not run if current gamemode is shotcaller
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MicroInput => {}
    }

    for (e, orders, pos) in join!(&entities && &order_queue && &positions) {
        // Current order is moveto point
        let oq = orders.unwrap();
        if oq.orders.len() > 0 {
            if let UnitOrder::HoldPosition = oq.orders[0] {
                if let Some(curr_trg) = targets.get(e.unwrap()) {
                    // Movement destination should be the current position
                    if curr_trg.target != *pos.unwrap() {
                        targets.insert(e.unwrap(), AiDestination::new(pos.unwrap().clone()));
                        // TODO: I don't like this part. But it is here because if AIDestination == Current position,
                        // then minigene doesn't run pathfinding (meaning that old path doesn't get edited)
                        paths.remove(e.unwrap());
                    }
                }
            }
        }
    }

    Ok(())
}
