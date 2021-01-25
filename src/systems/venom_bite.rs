use crate::*;

/// Every attack applies stacking (up to 5) poison and slow debuff.
pub fn venom_bite_system(
    skills: &Components<SkillSet<Skills>>,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {

    let dot_effector = effector_defs
        .defs
        .get(&Effectors::VenomDamage)
        .expect("Unknown effector key.");

    let slow_effector = effector_defs
        .defs
        .get(&Effectors::VenomSlow)
        .expect("Unknown effector key.");

    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(a, t, _dmg) = ev {
            if let Some(skills) = skills.get(*a) {
                if skills.skills.get(&Skills::VenomBite).is_some() {
                    if effectors.get(*t).is_none() {
                        effectors.insert(*t, EffectorSet::default());
                    }

                    let current_effectors = &mut effectors.get_mut(*t).unwrap().effectors;

                    // Check how many stacks of HemotoxicVenom target has.
                    // If over 5 then do nothing. Otherwise apply 1 more
                    // TODO: stack limit should be specified in data files, not hardcoded
                    if current_effectors
                        .iter()
                        .filter(|x| (x.effector_key == Effectors::VenomDamage) | 
                        (x.effector_key == Effectors::VenomDamage))
                        .count()
                        < 5
                    {
                        // Assume that VenomDamage and VenomSlow are always applied together
                        current_effectors.push(EffectorInstance::new(
                            Effectors::VenomDamage,
                            dot_effector.duration,
                        ));

                        current_effectors.push(EffectorInstance::new(
                            Effectors::VenomSlow,
                            slow_effector.duration,
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}
