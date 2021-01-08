use crate::*;

/// Increase the attack speed of both leader and companion in 5 points.
pub fn savagery_system(
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    companions: &Components<Companion>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    events: &mut Vec<SkillTriggerEvent<Skills>>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::Savagery {
            let savagery_effector_def = effector_defs
                .defs
                .get(&Effectors::Savagery)
                .expect("Unknown effector key.");

            let savagery_effector = EffectorInstance::new(Effectors::Savagery, savagery_effector_def.duration);

            if let Some(c) = companions.get(ev.0) {
                match c.0 {
                    Unit::Bear(e) => {
                        if effectors.get(e).is_none() {
                            effectors.insert(e, EffectorSet::default());
                        }
                        effectors.get_mut(e).unwrap().effectors.push(savagery_effector);
                    }
                }
            }
        }
    }
    Ok(())
}
