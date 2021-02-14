use crate::*;

/// Moves the entity with M-move order towards the destination, provided we have enough action points to do so.
pub fn mmove_order_system(
    entities: &Entities,
    gamemode: &GameMode,
    order_queue: &Components<OrderQueue>,
    stats: &mut Components<StatSet<Stats>>,
    targets: &mut Components<AiDestination>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    // This system should not run if current gamemode is shotcaller
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MircoInput => {}
    }

    for (e, orders) in join!(&entities && &order_queue) {
        // Current order is moveto point
        if let UnitOrder::MovetoPoint(trg_pt) = orders.unwrap().orders[0] {
            //Copied from simple_movement.rs:
            if stats
                .get(e.unwrap())
                .unwrap()
                .stats
                .get(&Stats::ActionPoints)
                .unwrap()
                .value
                >= ACTION_POINT_MOVE_COST
            {
                stats
                    .get_mut(e.unwrap())
                    .unwrap()
                    .stats
                    .get_mut(&Stats::ActionPoints)
                    .unwrap()
                    .value -= ACTION_POINT_MOVE_COST;
                targets.insert(e.unwrap(), AiDestination::new(trg_pt));
            } else {
                targets.remove(e.unwrap());
                paths.remove(e.unwrap());
            }
        }
    }

    Ok(())
}
