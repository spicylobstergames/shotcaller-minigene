use crate::*;

/// Increases the defense for each received attack.
pub fn back_endurance_system(
    skills: &Components<SkillSet<Skills>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut additional_defense = Vec::new();
    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(_, t, _) = ev {
            if let Some(skills) = skills.get(*t) {
                if skills.skills.get(&Skills::BackEndurance).is_some() {
                    additional_defense.push(GameEvent::AdditionalDefense(*t, 1.0));
                }
            }
        }
    }
    for d in additional_defense {
        game_events.push(d);
    }
    Ok(())
}
