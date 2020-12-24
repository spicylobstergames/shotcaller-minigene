use crate::*;

system!(
    HandleActionPointsSystem,
    |entities: Entities<'a>,
    stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
    simple_movements: ReadStorage<'a, SimpleMovement>,
    hero1_simple_movements: ReadStorage<'a, Hero1SimpleMovement>,
    targets: WriteStorage<'a, AiDestination>| {
        for (e, stat, _) in (&*entities, &mut stats, &simple_movements).join() {
            stat.0.stats.get_mut(&Stats::ActionPoints).unwrap().value +=
                stat.0.stats.get(&Stats::ActionPointRefillRate).unwrap().value;
            if stat.0.stats.get(&Stats::ActionPoints).unwrap().value >= 100.0 {
                stat.0.stats.get_mut(&Stats::ActionPoints).unwrap().value -= 100.0;
            } else {
                targets.remove(e);
            }
        }
        for (e, stat, _) in (&*entities, &mut stats, &hero1_simple_movements).join() {
            stat.0.stats.get_mut(&Stats::ActionPoints).unwrap().value +=
                stat.0.stats.get(&Stats::ActionPointRefillRate).unwrap().value;
            if stat.0.stats.get(&Stats::ActionPoints).unwrap().value >= 100.0 {
                stat.0.stats.get_mut(&Stats::ActionPoints).unwrap().value -= 100.0;
            } else {
                targets.remove(e);
            }
        }
    }
);
