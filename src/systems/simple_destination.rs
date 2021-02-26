use crate::*;

/// Sets AIDestination towards the closest enemy
pub fn simple_destination_system(
    entities: &Entities,
    simple_movements: &Components<SimpleMovement>,
    teams: &Components<Team>,
    positions: &Components<Point>,
    stats: &mut Components<StatSet<Stats>>,
    targets: &mut Components<AiDestination>,
) -> SystemResult {
    'query: for (e, _, team, pos) in join!(&entities && &simple_movements && &teams && &positions) {
        // For optimisation purposes runs only if unit has action points to move on this frame
        if stats
            .get(e.unwrap())
            .unwrap()
            .stats
            .get(&Stats::ActionPoints)
            .unwrap()
            .value
            < ACTION_POINT_MOVE_COST
        {
            continue 'query;
        }
        let closest = find_closest_in_other_team(
            team.unwrap(),
            pos.unwrap(),
            &teams,
            &positions,
            &stats,
            &entities,
        );
        if let Some((_, c)) = closest {
            targets.insert(e.unwrap(), AiDestination::new(c.clone()));
        }
    }

    Ok(())
}
