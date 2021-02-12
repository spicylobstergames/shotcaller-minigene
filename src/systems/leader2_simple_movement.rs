use crate::*;

/// Moves ranged leaders on the map.
pub fn leader2_simple_movement_system(
    entities: &Entities,
    simple_movements: &Components<MovementSystems>,
    teams: &Components<Team>,
    is_caught: &Components<IsCaught>,
    stats: &Components<StatSet<Stats>>,
    leaders: &Components<Leader>,
    retreats: &Components<FleeToBase>,
    cores: &Components<Core>,
    positions: &Components<Point>,
    targets: &mut Components<AiDestination>,
) -> SystemResult {
    for (e, flee, leader_team, caught) in join!(&entities && &retreats && &teams && &is_caught) {
        let e = e.unwrap();
        let flee = flee.unwrap();
        let leader_team = leader_team.unwrap();
        let caught = caught.unwrap();
        if caught.0 {
            for (e, movement, team, pos) in
                join!(&entities && &simple_movements && &teams && &positions)
            {
                if let MovementSystems::Leader2SimpleMovement = movement.unwrap() {
                    let e = e.unwrap();
                    let team = team.unwrap();
                    let pos = pos.unwrap();
                    // find closest leader in other team
                    // TODO: optimize
                    let mut vec = join!(&teams && &positions && &stats && &leaders)
                        .filter(|(t, _, _, _)| *t.unwrap() != *team)
                        .map(|(_, p, _, _)| (dist(pos, p.unwrap()), p.unwrap().clone()))
                        .collect::<Vec<_>>();
                    vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
                    let closest = vec.into_iter().next().map(|(_d, p)| p);
                    if let Some(c) = closest {
                        targets.insert(e, AiDestination::new(c.clone())).unwrap();
                    } else {
                        targets.remove(e);
                    }
                }
            }
        } else {
            // retreat if health is low
            if stats
                .get(e)
                .unwrap()
                .stats
                .get(&Stats::Health)
                .unwrap()
                .value
                <= flee.0
            {
                for (point, team, _) in join!(&positions && &teams && &cores) {
                    if team.unwrap() == leader_team {
                        targets
                            .insert(e, AiDestination::new(point.unwrap().clone()))
                            .unwrap();
                    }
                }
            } else {
                for (e, movement, team, pos) in
                    join!(&entities && &simple_movements && &teams && &positions)
                {
                    if let MovementSystems::Leader2SimpleMovement = movement.unwrap() {
                        let e = e.unwrap();
                        let pos = pos.unwrap();
                        let team = team.unwrap();
                        // find closest enemy
                        let closest = find_closest_in_other_team(
                            team, pos, &teams, &positions, &stats, &entities,
                        );
                        if dist(&closest.unwrap().1, pos) > RANGED_LEADER_ATTACK_RADIUS {
                            if let Some((_, c)) = closest {
                                targets.insert(e, AiDestination::new(c.clone())).unwrap();
                            } else {
                                targets.remove(e);
                            }
                        } else {
                            targets.remove(e);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
