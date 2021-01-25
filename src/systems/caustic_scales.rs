use crate::*;

/// Every unit that attacks this leader gets poisoned and has attack speed slowed.
pub fn caustic_scales_system(
    skills: &Components<SkillSet<Skills>>,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let dot_effector = effector_defs
        .defs
        .get(&Effectors::CausticDamage)
        .expect("Unknown effector key.");

    let slow_effector = effector_defs
        .defs
        .get(&Effectors::CausticSlow)
        .expect("Unknown effector key.");

    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(a, t, _dmg) = ev {
            if let Some(skills) = skills.get(*t) {
                if skills.skills.get(&Skills::CausticScales).is_some() {
                    if effectors.get(*a).is_none() {
                        effectors.insert(*a, EffectorSet::default());
                    }

                    let current_effectors = &mut effectors.get_mut(*a).unwrap().effectors;

                    // If CausticScales already applied, then just refresh the duration
                    if current_effectors.iter().any(|x| match x.effector_key {
                        Effectors::CausticDamage => true,
                        Effectors::CausticSlow => true,
                        _ => false,
                    }) {
                        for i in 0..current_effectors.len() {
                            if current_effectors[i].effector_key == Effectors::CausticDamage {
                                current_effectors[i].disable_in = dot_effector.duration;
                            }
                            if current_effectors[i].effector_key == Effectors::CausticSlow {
                                current_effectors[i].disable_in = slow_effector.duration;
                            }
                        }
                    } else {
                        effectors.get_mut(*a).unwrap().effectors.append(&mut vec![
                            EffectorInstance::new(Effectors::CausticDamage, dot_effector.duration),
                            EffectorInstance::new(Effectors::CausticSlow, slow_effector.duration),
                        ]);
                    }
                }
            }
        }
    }

    Ok(())
}
