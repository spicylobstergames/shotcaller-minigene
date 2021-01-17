use crate::*;

/// Teleports the closest enemy to the second closest enemy. Stun AOE is applied in `aoe_damage.rs`.
pub fn telekinesis_system(
    entities: &Entities,
    teams: &Components<Team>,
    events: &Vec<SkillTriggerEvent<Skills>>,
    positions: &mut Components<Point>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::Telekinesis {
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                let enemies_around = entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= RANGED_LEADER_ATTACK_RADIUS,
                );
                let closest_enemy = enemies_around.first().unwrap().0;
                let target_enemy = enemies_around.get(1).unwrap().0;

                positions.get_mut(closest_enemy).unwrap().x = positions.get(target_enemy).unwrap().x;
                positions.get_mut(closest_enemy).unwrap().y = positions.get(target_enemy).unwrap().y;
            }
        }
    }
    Ok(())
}
