use crate::*;

system!(
    SimpleMovementSystem,
    |entities: Entities<'a>,
     simple_movements: ReadStorage<'a, SimpleMovement>,
     teams: ReadStorage<'a, Team>,
     targets: WriteStorage<'a, AiDestination>,
     stats: ReadStorage<'a, Comp<StatSet<Stats>>>,
     positions: ReadStorage<'a, Point>| {
        for (e, _, team, pos) in (&*entities, &simple_movements, &teams, &positions).join() {
            // find closest in other team
            // TODO: optimize
            let mut vec = (&teams, &positions, &stats)
                .join()
                .filter(|(t, _, _)| **t != *team)
                .map(|(_, p, _)| (dist(pos, p), p.clone()))
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
);
