use crate::*;

/// Update additional defense effector.
pub fn additional_defense_system(
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let additional_defense_effector = effector_defs
        .defs
        .get(&Effectors::AdditionalDefense)
        .expect("Unknown effector key.");
    for ev in game_events.iter() {
        if let GameEvent::AdditionalDefense(e, a) = ev {
            for _ in 0..(*a as u64) {
                if effectors.get(*e).is_none() {
                    effectors.insert(*e, EffectorSet::default());
                }

                effectors
                    .get_mut(*e)
                    .unwrap()
                    .effectors
                    .push(EffectorInstance::new(
                        Effectors::AdditionalDefense,
                        additional_defense_effector.duration,
                    ))
            }
        }
    }

    Ok(())
}
