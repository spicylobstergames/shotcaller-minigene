use crate::*;

/// Applies to units with no orders in OrderQueue. Moves towards enemy if it is in aggro range, otherwise stands in place.
pub fn idle_order_system(
    entities: &Entities,
    gamemode: &GameMode,
    order_queue: &Components<OrderQueue>,
    teams: &Components<Team>,
    positions: &Components<Point>,
    stats: &Components<StatSet<Stats>>,
    rng: &mut RNG,
    targets: &mut Components<AiDestination>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    // This system should not run if current gamemode is shotcaller
    match gamemode {
        GameMode::Shotcaller => return Ok(()),
        GameMode::MicroInput => {}
    }

    'query: for (e, orders, pos, team, stts) in
        join!(&entities && &order_queue && &positions && &teams && &stats)
    {
        // Current order is moveto point
        let oq = orders.unwrap();
        if oq.orders.len() == 0 {
            // find aggro range:
            let aggro_range = stts
                .unwrap()
                .stats
                .get(&Stats::AggroRange)
                .unwrap()
                .value
                .clone() as f32;

            // find closest enemy
            let closest = find_closest_in_other_team(
                team.unwrap(),
                pos.unwrap(),
                &teams,
                &positions,
                &stats,
                &entities,
            );

            // Attack enemies within aggro range
            if let Some((_, c)) = closest {
                if dist(&c, pos.unwrap()) <= aggro_range {
                    // set destination to closest enemy
                    targets.insert(e.unwrap(), AiDestination::new(c.clone()));
                    continue 'query;
                }
                // Else move if stacked with other units:
                // TODO: optimise. Currently it performs grid search. If performance too bad, then just comment out.
                for (e1, pos1) in join!(&entities && &positions) {
                    if e1.unwrap() != e.unwrap() && pos.unwrap() == pos1.unwrap() {
                        // choose random adjacent tile as a destination to unstack entities:
                        let trg_pos = Point::new(
                            pos.unwrap().x - (rng.rng.rand_range(0..3) as i32) + 1,
                            pos.unwrap().y - (rng.rng.rand_range(0..3) as i32) + 1,
                        );
                        targets.insert(e.unwrap(), AiDestination::new(trg_pos));
                        continue 'query;
                    }
                }
                // If no previous conditions apply, then stay in place
                targets.insert(e.unwrap(), AiDestination::new(pos.unwrap().clone()));
                // TODO: I don't like this part. But it is here because if AIDestination == Current position,
                // then minigene doesn't run pathfinding (meaning that old path doesn't get edited)
                paths.remove(e.unwrap());
            }
        }
    }

    Ok(())
}
