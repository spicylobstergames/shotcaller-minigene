use crate::*;

pub fn simple_movement_system(
    entities: &Entities,
    simple_movements: &Components<SimpleMovement>,
    teams: &Components<Team>,
    stats: &Components<StatSet<Stats>>,
    positions: &Components<Point>,
    targets: &mut Components<AiDestination>,
) -> SystemResult {
    for (e, _, team, pos) in join!(&entities && &simple_movements && &teams && &positions) {
        // find closest in other team
        // TODO: optimize
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
        } else {
            targets.remove(e.unwrap());
        }
    }
    Ok(())
}
