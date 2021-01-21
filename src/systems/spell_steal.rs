use crate::*;

/// Gives the caster access to an ability belonging to another leader within range.
pub fn spell_steal_system(
    entities: &Entities,
    teams: &Components<Team>,
    positions: &Components<Point>,
    events: &Vec<SkillTriggerEvent<Skills>>,
    leaders: &Components<Leader>,
    spell_steal: &mut Components<SpellSteal>,
    skillsets: &mut Components<SkillSet<Skills>>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::SpellSteal {
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                for (e, pos) in join!(&entities && &positions) {
                    let e = e.unwrap();
                    let pos = pos.unwrap();
                    if pos == from && !spell_steal.get(e).unwrap().0 {
                        let closest_leader = entities_in_radius(
                            pos,
                            &*entities,
                            &positions,
                            |e, _| teams.get(e).map(|t| t != team).unwrap() && leaders.get(e).is_some(),
                            |_, _, d| d <= RANGED_LEADER_ATTACK_RADIUS,
                        ).first().map(|t| t.0).unwrap();

                        let skill = skillsets.get(closest_leader).unwrap().clone().skills.into_values().next().unwrap();

                        skillsets.get_mut(e).unwrap().skills.insert(skill.skill_key, skill);
                        spell_steal.get_mut(e).unwrap().0 = true;
                    }
                }
            }
        }
    }
    Ok(())
}