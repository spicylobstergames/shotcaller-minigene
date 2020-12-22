use crate::*;

system!(ProximityAttackSystem, |entities: Entities<'a>,
                                proximity_attacks: ReadStorage<
    'a,
    ProximityAttack,
>,
                                stats: WriteStorage<
    'a,
    Comp<StatSet<Stats>>,
>,
                                teams: ReadStorage<'a, Team>,
                                positions: ReadStorage<
    'a,
    Point,
>| {
    let mut v = vec![];
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

    for (attacker, target, dmg) in v.into_iter() {
        increment_attacks_dealt(&mut stats.get_mut(attacker).unwrap().0);
        if damage(&mut stats.get_mut(target).unwrap().0, dmg) {
            entities.delete(target).unwrap();
        }
    }
});
