use crate::*;

system!(TowerProjectileSystem, |projectiles: ReadStorage<
    'a,
    TowerProjectile,
>,
                                entities: Entities<'a>,
                                positions: ReadStorage<
    'a,
    Point,
>,
                                teams: ReadStorage<'a, Team>,
                                gotos: ReadStorage<
    'a,
    GotoStraight,
>,
                                stats: WriteStorage<
    'a,
    Comp<StatSet<Stats>>,
>| {
    for (e, pos, goto, _, team) in (&*entities, &positions, &gotos, &projectiles, &teams).join() {
        let dmg = stats
            .get(e)
            .expect("Add a statset to the projectile.")
            .0
            .stats
            .get(&Stats::Attack)
            .unwrap()
            .value;
        if *pos == goto.target {
            for (e, _, _) in entities_in_radius(
                pos,
                &*entities,
                &positions,
                |e, p| teams.get(e).map(|t| t != team).unwrap_or(false),
                |e, p, d| d <= TOWER_PROJECTILE_EXPLOSION_RADIUS,
            ) {
                // damage around
                if let Some(mut stat) = stats.get_mut(e).as_mut().map(|c| &mut c.0) {
                    if damage(&mut stat, dmg) {
                        entities.delete(e).unwrap();
                    }
                }
            }
        }
    }
});
