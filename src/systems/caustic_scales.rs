use crate::*;

/// Returns half of the received damage.
pub fn caustic_scales_system(
    skills: &Components<SkillSet<Skills>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut apply_acid = None;
    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(a, t, _dmg) = ev {
            if let Some(skills) = skills.get(*t) {
                if skills.skills.get(&Skills::CausticScales).is_some() {
                    // Here attacker entity should get status effect with DOT and lowered defence
                    // return_damage = Some(GameEvent::DamageEntity(*t, *a, dmg / 2.0));

                }
            }
        }
    }

    // this part necessary?
    if let Some(event) = apply_acid {
        game_events.push(event);
    }

    Ok(())
}
