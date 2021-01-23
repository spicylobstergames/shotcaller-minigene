use crate::*;

/// Returns half of the received damage.
pub fn caustic_scales_system(
    skills: &Components<SkillSet<Skills>>,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut apply_acid = None;

    let dot_effector = effector_defs
        .defs
        .get(&Effectors::DamageOverTime)
        .expect("Unknown effector key.");

    let asd_effector = effector_defs
        .defs
        .get(&Effectors::AttackSpeedDecrease)
        .expect("Unknown effector key.");

    

    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(a, t, _dmg) = ev {
            if let Some(skills) = skills.get(*t) {
                if skills.skills.get(&Skills::CausticScales).is_some() {
                    // Here attacker a should get the effectors of half attack speed and damage over time.

                    if effectors.get(*a).is_none() {
                        effectors.insert(*a, EffectorSet::default());
                    }

                    // TODO: maybe better to have a single effector for bpoth effects? Depends on design philosophy IMO.
                    // TODO: if effectors already exist, renew duration instead.
                    effectors
                    .get_mut(*a)
                    .unwrap()
                    .effectors
                    .push(EffectorInstance::new(
                        Effectors::DamageOverTime,
                        dot_effector.duration,
                    ));

                    // TODO: beautify code to not have repetitions like this
                    effectors
                    .get_mut(*a)
                    .unwrap()
                    .effectors
                    .push(EffectorInstance::new(
                        Effectors::AttackSpeedDecrease,
                        dot_effector.duration,
                    ));


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
