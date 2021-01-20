use crate::*;

/// Returns half of the received damage.
pub fn return_damage_system(
    skills: &Components<SkillSet<Skills>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut return_damage = None;
    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(a, t, dmg) = ev {
            if skills.get(*t).unwrap().skills.get(&Skills::ReturnDamage).is_some() {
                return_damage = Some(GameEvent::DamageEntity(*t, *a, dmg / 2.0));
            }
        }
    }

    if let Some(event) = return_damage {
        game_events.push(event);
    }

    Ok(())
}
