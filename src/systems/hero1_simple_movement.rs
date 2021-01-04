use crate::*;

/// Moves heroes on the map.
pub fn hero1_simple_movement_system(
    entities: &Entities,
    simple_movements: &Components<Hero1SimpleMovement>,
    teams: &Components<Team>,
    is_caught: &Components<IsCaught>,
    stats: &Components<StatSet<Stats>>,
    creeps: &Components<Creep>,
    leaders: &Components<Leader>,
    retreats: &Components<FleeToBase>,
    cores: &Components<Core>,
    positions: &Components<Point>,
    targets: &mut Components<AiDestination>,
) -> SystemResult {
    for (e, leader, flee, leader_team, caught) in
        join!(&entities && &leaders && &retreats && &teams && &is_caught)
    {
        let e = e.unwrap();
        let leader = leader.unwrap();
        let flee = flee.unwrap();
        let leader_team = leader_team.unwrap();
        let caught = caught.unwrap();
        if leader.0 == 1 || leader.0 == 2 {
            if caught.0 {
                for (e, _, team, pos) in
                    join!(&entities && &simple_movements && &teams && &positions)
                {
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
                    for (e, _, pos) in join!(&entities && &simple_movements && &positions) {
                        let e = e.unwrap();
                        let pos = pos.unwrap();
                        // find closest creep
                        // TODO: optimize
                        let mut vec = join!(&positions && &stats && &creeps)
                            .map(|(p, _, _)| (dist(pos, p.unwrap()), p.unwrap().clone()))
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
            }
        }
    }
    Ok(())
}
