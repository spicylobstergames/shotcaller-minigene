use crate::*;

pub fn aoe_damage_system(
    positions: &Components<Point>,
    teams: &Components<Team>,
    entities: &Entities,
    events: &Vec<SkillTriggerEvent<Skills>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::AOE {
            // Damage around
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                for (e, _, _) in entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= AOE_RADIUS,
                ) {
                    game_events.push(GameEvent::DamageEntity(e, AOE_DAMAGE));
                }
            }
        }
    }
    Ok(())
}
