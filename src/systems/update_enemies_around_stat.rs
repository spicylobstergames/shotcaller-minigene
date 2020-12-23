use crate::*;

pub fn update_enemies_around_system(entities: &Entities,
                                    positions: &Components<
    Point,
>,
                                    teams: &Components<Team>,
                                    stats: &mut Components<
    StatSet<Stats>,
>) -> SystemResult {
    for (pos, stat, team) in join!(&positions && &mut stats && &teams){
        let c = entities_in_radius(
            pos.unwrap(),
            &*entities,
            &positions,
            |e, _| teams.get(e).map(|t| t != team.unwrap()).unwrap_or(false),
            |_, _, d| d <= AOE_RADIUS,
        )
        .len() as f64;
        stat.unwrap()
            .stats
            .get_mut(&Stats::EnemiesAround)
            .expect("Failed to get EnemiesAround stat")
            .value = c;
    }
    Ok(())
}
