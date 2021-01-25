use crate::*;

/// Returns half of the received damage.
pub fn caustic_scales_system(
    skills: &Components<SkillSet<Skills>>,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let dot_effector = effector_defs
        .defs
        .get(&Effectors::CausticScales)
        .expect("Unknown effector key.");

    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(a, t, _dmg) = ev {
            if let Some(skills) = skills.get(*t) {
                if skills.skills.get(&Skills::CausticScales).is_some() {
                    // Here attacker a should get the effectors of half attack speed and damage over time.

                    if effectors.get(*a).is_none() {
                        effectors.insert(*a, EffectorSet::default());
                    }

                    let current_effectors = &mut effectors.get_mut(*a).unwrap().effectors;

                    // If CausticScales already applied, then just refresh the duration
                    if current_effectors
                        .iter()
                        .any(|x| x.effector_key == Effectors::CausticScales)
                    {
                        for i in 0..current_effectors.len() {
                            if current_effectors[i].effector_key == Effectors::CausticScales {
                                current_effectors[i].disable_in = dot_effector.duration;
                            }
                        }
                    }
                    // else add the effector:
                    else {
                        effectors
                            .get_mut(*a)
                            .unwrap()
                            .effectors
                            .push(EffectorInstance::new(
                                Effectors::CausticScales,
                                dot_effector.duration,
                            ));
                    }
                }
            }
        }
    }

    Ok(())
}
