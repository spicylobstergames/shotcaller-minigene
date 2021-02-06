use crate::*;

/// Moves the entity towards the closest enemy, provided we have enough action points to do so.
pub fn simple_movement_system(
    entities: &Entities,
    gamemode: &GameMode,
    simple_movements: &Components<SimpleMovement>,
    teams: &Components<Team>,
    positions: &Components<Point>,
    stats: &mut Components<StatSet<Stats>>,
    targets: &mut Components<AiDestination>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    for (e, _, team, pos) in join!(&entities && &simple_movements && &teams && &positions) {

        // This system should not run if current gamemode is micro input:
        match gamemode {
            GameMode::Shotcaller => {},
            GameMode::MircoInput => return Ok(()),
        }

        // find closest in other team
        // TODO: optimize
        // TODO: fix assumption that if you have a movement and team you have stats
        if stats
            .get(e.unwrap())
            .unwrap()
            .stats
            .get(&Stats::ActionPoints)
            .unwrap()
            .value
            >= ACTION_POINT_MOVE_COST
        {
            let closest = find_closest_in_other_team(
                team.unwrap(),
                pos.unwrap(),
                &teams,
                &positions,
                &stats,
                &entities,
            );
            if let Some((_, c)) = closest {
                stats
                    .get_mut(e.unwrap())
                    .unwrap()
                    .stats
                    .get_mut(&Stats::ActionPoints)
                    .unwrap()
                    .value -= ACTION_POINT_MOVE_COST;
                targets.insert(e.unwrap(), AiDestination::new(c.clone()));
            } else {
                targets.remove(e.unwrap());
            }
        } else {
            targets.remove(e.unwrap());
            paths.remove(e.unwrap());
        }
    }
    Ok(())
}
