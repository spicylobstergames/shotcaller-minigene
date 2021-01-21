use crate::*;

/// Update additional attack effector.
pub fn additional_attack_system(
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let additional_attack_effector = effector_defs
        .defs
        .get(&Effectors::AdditionalAttack)
        .expect("Unknown effector key.");
    for ev in game_events.iter() {
        if let GameEvent::AdditionalAttack(e, a) = ev {
            for _ in 0..(*a as u64) {
                if effectors.get(*e).is_none() {
                    effectors.insert(*e, EffectorSet::default());
                }

                effectors
                    .get_mut(*e)
                    .unwrap()
                    .effectors
                    .push(EffectorInstance::new(
                        Effectors::AdditionalAttack,
                        additional_attack_effector.duration,
                    ))
            }
        }
    }

    Ok(())
}
