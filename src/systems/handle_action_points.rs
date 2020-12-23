use crate::*;

system!(
    HandleActionPointsSystem,
    |entities: Entities<'a>,
    stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
    simple_movements: ReadStorage<'a, SimpleMovement>,
    hero1_simple_movements: ReadStorage<'a, Hero1SimpleMovement>,
    targets: WriteStorage<'a, AiDestination>| {
        for (e, _) in (&*entities, &simple_movements).join() {
            stats.get_mut(e).unwrap().0.stats.get_mut(&Stats::ActionPoints).unwrap().value +=
                stats.get(e).unwrap().0.stats.get(&Stats::ActionPointRefillRate).unwrap().value;
            if stats.get(e).unwrap().0.stats.get(&Stats::ActionPoints).unwrap().value >= 100.0 {
                stats.get_mut(e).unwrap().0.stats.get_mut(&Stats::ActionPoints).unwrap().value -= 100.0;
            } else {
                targets.remove(e);
            }
        }
        for (e, _) in (&*entities, &hero1_simple_movements).join() {
            stats.get_mut(e).unwrap().0.stats.get_mut(&Stats::ActionPoints).unwrap().value +=
                stats.get(e).unwrap().0.stats.get(&Stats::ActionPointRefillRate).unwrap().value;
            if stats.get(e).unwrap().0.stats.get(&Stats::ActionPoints).unwrap().value >= 100.0 {
                stats.get_mut(e).unwrap().0.stats.get_mut(&Stats::ActionPoints).unwrap().value -= 100.0;
            } else {
                targets.remove(e);
            }
        }
    }
);
