use crate::*;

system!(UpdateEnemiesAroundSystem, |entities: Entities<'a>,
                                    positions: ReadStorage<
    'a,
    Point,
>,
                                    teams: ReadStorage<'a, Team>,
                                    stats: WriteStorage<
    'a,
    Comp<StatSet<Stats>>,
>| {
    for (e, pos, stat, team) in (&*entities, &positions, &mut stats, &teams).join() {
        let c = entities_in_radius(
            pos,
            &*entities,
            &positions,
            |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
            |_, _, d| d <= AOE_RADIUS,
        )
        .len() as f64;
        stat.0
            .stats
            .get_mut(&Stats::EnemiesAround)
            .expect("Failed to get EnemiesAround stat")
            .value = c;
    }
});
