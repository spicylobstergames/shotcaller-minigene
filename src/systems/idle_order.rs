use crate::*;

/// Applies to units with no orders in OrderQueue. Moves towards enemy if it is in aggro range, otherwise stands in place.
pub fn idle_order_system(
    entities: &Entities,
    gamemode: &GameMode,
    order_queue: &Components<OrderQueue>,
    teams: &Components<Team>,
    positions: &Components<Point>,
    stats: &Components<StatSet<Stats>>,
    targets: &mut Components<AiDestination>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    // This system should not run if current gamemode is shotcaller
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MircoInput => {}
    }

    // TODO: should be defined in data files.
    let aggro_range = 7.0;

    for (e, orders, pos, team) in join!(&entities && &order_queue && &positions && &teams) {
        // Current order is moveto point
        let oq = orders.unwrap();
        if oq.orders.len() == 0 {
            // find closest enemy
            let closest = find_closest_in_other_team(
                team.unwrap(),
                pos.unwrap(),
                &teams,
                &positions,
                &stats,
                &entities,
            );

            if let Some((_, c)) = closest {
                if dist(&c, pos.unwrap()) <= aggro_range {
                    // set destination to closest enemy
                    targets.insert(e.unwrap(), AiDestination::new(c.clone()));
                } else {
                    // stay in place
                    targets.insert(e.unwrap(), AiDestination::new(pos.unwrap().clone()));
                    // TODO: I don't like this part. But it is here because if AIDestination == Current position,
                    // then minigene doesn't run pathfinding (meaning that old path doesn't get edited)
                    paths.remove(e.unwrap());
                }
            }
        }
    }

    Ok(())
}
