use crate::*;

/// Update the `EnemiesAround` stat using the entities that are close to the entity.
pub fn update_enemies_around_system(
    entities: &Entities,
    positions: &Components<Point>,
    teams: &Components<Team>,
    skills: &Components<SkillSet<Skills>>,
    stats: &mut Components<StatSet<Stats>>,
) -> SystemResult {
    for (pos, stat, team, skill) in join!(&positions && &mut stats && &teams && &skills) {
        let mut radius = AOE_RADIUS;

        if let Some(_) = skill.unwrap().skills.get(&Skills::SlowAOE) {
            radius = SLOW_AOE_RADIUS;
        }

        let c = entities_in_radius(
            pos.unwrap(),
            &*entities,
            &positions,
            |e, _| teams.get(e).map(|t| t != team.unwrap()).unwrap_or(false),
            |_, _, d| d <= radius,
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
