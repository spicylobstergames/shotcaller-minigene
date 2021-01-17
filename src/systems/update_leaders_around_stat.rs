use crate::*;

/// Update the `LeadersAround` stat using the entities that are close to the entity.
pub fn update_leaders_around_system(
    entities: &Entities,
    positions: &Components<Point>,
    teams: &Components<Team>,
    skills: &Components<SkillSet<Skills>>,
    leaders: &Components<Leader>,
    stats: &mut Components<StatSet<Stats>>,
) -> SystemResult {
    for (pos, stat, team, skill) in join!(&positions && &mut stats && &teams && &skills) {
        let mut radius = AOE_RADIUS;

        if let Some(_) = skill.unwrap().skills.get(&Skills::SlowAOE) {
            radius = SLOW_AOE_RADIUS;
        } else if let Some(_) = skill.unwrap().skills.get(&Skills::Telekinesis) {
            radius = RANGED_LEADER_ATTACK_RADIUS;
        }

        let c = entities_in_radius(
            pos.unwrap(),
            &*entities,
            &positions,
            |e, _| teams.get(e).map(|t| t != team.unwrap()).unwrap_or(false),
            |_, _, d| d <= radius,
        );

        let mut leaders_around = vec![];
        for e in c {
            if let Some(_) = leaders.get(e.0) {
                leaders_around.push((e.0, e.1));
            }
        }

        stat.unwrap()
            .stats
            .get_mut(&Stats::LeadersAround)
            .expect("Failed to get LeadersAround stat")
            .value = leaders_around.len() as f64;
    }
    Ok(())
}
