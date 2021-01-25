use crate::*;

/// Deals AOE damage for each received attack
pub fn thorn_volley_system(
    game_events: &mut Vec<GameEvent>,
    events: &mut Vec<SkillTriggerEvent<Skills>>,
) -> SystemResult {
    let mut aoe_skill_trigger = Vec::new();
    for game_ev in game_events.iter() {
        if let GameEvent::DamageEntity(_, t, _) = game_ev {
            for ev in events.iter() {
                if ev.1 == Skills::ThornVolley {
                    aoe_skill_trigger.push(SkillTriggerEvent(*t, Skills::AOE));
                }
            }
        }
    }
    for t in aoe_skill_trigger {
        events.push(t);
    }
    Ok(())
}
