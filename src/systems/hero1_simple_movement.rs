use crate::*;

system!(
    Hero1SimpleMovementSystem,
    |entities: Entities<'a>,
     simple_movements: ReadStorage<'a, Hero1SimpleMovement>,
     teams: ReadStorage<'a, Team>,
     targets: WriteStorage<'a, AiDestination>,
     is_caught: ReadStorage<'a, IsCaught>,
     stats: ReadStorage<'a, Comp<StatSet<Stats>>>,
     creeps: ReadStorage<'a, Creep>,
     leaders: ReadStorage<'a, Leader>,
     retreats: ReadStorage<'a, FleeToBase>,
     cores: ReadStorage<'a, Core>,
     positions: ReadStorage<'a, Point>| {
        for (e, leader, flee, leader_team, caught) in 
            (&*entities, &leaders, &retreats, &teams, &is_caught).join() 
        {
            if leader.0 == 1 {
                if caught.0 {
                    for (e, _, team, pos) in 
                        (&*entities, &simple_movements, &teams, &positions).join()
                    {
                        // find closest leader in other team
                        // TODO: optimize
                        let mut vec = (&teams, &positions, &stats, &leaders)
                            .join()
                            .filter(|(t, _, _, _)| **t != *team)
                            .map(|(_, p, _, _)| (dist(pos, p), p.clone()))
                            .collect::<Vec<_>>();
                        vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
                        let closest = vec.into_iter().next().map(|(d, p)| p);
                        if let Some(c) = closest {
                            targets.insert(e, AiDestination::new(c.clone())).unwrap();
                        } else {
                            targets.remove(e);
                        }
                    }
                } else {
                    // retreat if health is low
                    if stats.get(e).unwrap().0.stats.get(&Stats::Health).unwrap().value <= flee.0 {
                        for (point, team, _) in (&positions, &teams, &cores).join() {
                            if team == leader_team {
                                targets.insert(e, AiDestination::new(point.clone())).unwrap();
                            }
                        }
                    } else {
                        for (e, _, pos) in
                            (&*entities, &simple_movements, &positions).join()
                        {
                            // find closest creep
                            // TODO: optimize
                            let mut vec = (&positions, &stats, &creeps)
                                .join()
                                .map(|(p, _, _)| (dist(pos, p), p.clone()))
                                .collect::<Vec<_>>();
                            vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
                            let closest = vec.into_iter().next().map(|(d, p)| p);
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
    }
);