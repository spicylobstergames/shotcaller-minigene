use crate::*;

pub fn tower_projectile_system(
    projectiles: &Components<TowerProjectile>,
    positions: &Components<Point>,
    teams: &Components<Team>,
    gotos: &Components<GotoStraight>,
    entities: &mut Entities,
    stats: &mut Components<StatSet<Stats>>,
) -> SystemResult {
    let mut kill = vec![];
    for (e, pos, goto, _, team) in
        join!(&entities && &positions && &gotos && &projectiles && &teams)
    {
        let pos = pos.unwrap();
        let team = team.unwrap();
        let dmg = stats
            .get(e.unwrap())
            .expect("Add a statset to the projectile.")
            .stats
            .get(&Stats::Attack)
            .unwrap()
            .value;
        if *pos == goto.unwrap().target {
            for (e, _, _) in entities_in_radius(
                pos,
                &*entities,
                &positions,
                |e, _p| teams.get(e).map(|t| t != team).unwrap_or(false),
                |_e, _p, d| d <= TOWER_PROJECTILE_EXPLOSION_RADIUS,
            ) {
                // damage around
                if let Some(mut stat) = stats.get_mut(e).as_mut() {
                    if damage(&mut stat, dmg) {
                        kill.push(e);
                    }
                }
            }
        }
    }
    for k in kill {
        entities.kill(k);
    }
    Ok(())
}
