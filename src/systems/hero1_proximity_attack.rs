use crate::*;
use rand::thread_rng;
use rand::Rng;

system!(Hero1ProximityAttackSystem, |entities: Entities<'a>,
                                proximity_attacks: ReadStorage<
    'a,
    Hero1ProximityAttack,
>,
                                stats: WriteStorage<
    'a,
    Comp<StatSet<Stats>>,
>,
                                creeps: ReadStorage<
    'a,
    Creep,
>,
                                is_caught: WriteStorage<
    'a,
    IsCaught,
>,
                                leaders: ReadStorage<
    'a,
    Leader,
>,
                                teams: ReadStorage<'a, Team>,
                                positions: ReadStorage<
    'a,
    Point,
>| {
    let mut v = vec![];
    let mut rng = thread_rng();
    for (e, proximity, stat, pos, team) in
        (&*entities, &proximity_attacks, &stats, &positions, &teams).join()
    {
        let mut vec = (&*entities, &teams, &positions, &stats)
            .join()
            .filter(|(_e, t, _, _)| **t != *team)
            .map(|(e, _, p, _)| (dist(pos, p), e))
            .filter(|(d, _)| *d < proximity.radius)
            .collect::<Vec<_>>();
        vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
        let closest = vec.into_iter().next().map(|(_d, p)| p);
        if let Some(target) = closest {
            let damage = stat.0.stats.get(&Stats::Attack).unwrap().value;
            v.push((e.clone(), target.clone(), damage));
        }
    }
    // 5% chance of getting caught if leaders are in range of each other
    for (proximity, pos, team, _, caught) in
        (&proximity_attacks, &positions, &teams, &leaders, &mut is_caught).join()
    {
        let mut vec = (&*entities, &teams, &positions, &stats, &leaders)
            .join()
            .filter(|(_e, t, _, _, _)| **t != *team)
            .map(|(e, _, p, _, _)| (dist(pos, p), e))
            .filter(|(d, _)| *d < proximity.radius)
            .collect::<Vec<_>>();
        vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
        let closest = vec.into_iter().next().map(|(_d, p)| p);
        if let Some(_) = closest {
            // 5% chance of leaders getting caught
            if rng.gen_range(1, 21) == 1 {
                caught.0 = true;
            }
        }
    }

    for (attacker, target, dmg) in v.into_iter() {
        increment_attacks_dealt(&mut stats.get_mut(attacker).unwrap().0);
        if damage(&mut stats.get_mut(target).unwrap().0, dmg) {
            entities.delete(target).unwrap();
        }
    }
});
