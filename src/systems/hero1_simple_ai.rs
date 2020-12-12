use crate::*;

system!(
    Hero1SimpleMovementSystem,
    |entities: Entities<'a>,
     simple_movements: ReadStorage<'a, Hero1SimpleMovement>,
     targets: WriteStorage<'a, AiDestination>,
     stats: ReadStorage<'a, Comp<StatSet<Stats>>>,
     creeps: ReadStorage<'a, Creep>,
     positions: ReadStorage<'a, Point>| {
        for (e, _, pos) in
            (&*entities, &simple_movements, &positions).join() {
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
);