use crate::*;

/// Returns half of the received damage.
pub fn venom_bite_system(
    skills: &Components<SkillSet<Skills>>,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let venom_effector = effector_defs
        .defs
        .get(&Effectors::HemotoxicVenom)
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
                    if current_effectors
                        .iter()
                        .filter(|x| x.effector_key == Effectors::HemotoxicVenom)
                        .count()
                        < 5
                    {
                        current_effectors.push(EffectorInstance::new(
                            Effectors::HemotoxicVenom,
                            venom_effector.duration,
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}
