use crate::*;

/// Increase the attack speed of both leader and companion in 5 points.
pub fn savagery_system(
    entities: &Entities,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    events: &mut Vec<SkillTriggerEvent<Skills>>,
    companions: &mut Components<Companion<Entity>>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::Savagery {
            let attack_speed_effector_def = effector_defs
                .defs
                .get(&Effectors::AttackSpeedIncrease)
                .expect("Unknown effector key.");

            let attack_speed_effector = EffectorInstance::new(Effectors::AttackSpeedIncrease, attack_speed_effector_def.duration);

            match companions.get(ev.0) {
                Some(c) => {
                    if effectors.get(c.get()).is_none() {
                        effectors.insert(companion, EffectorSet::default());
                    }

                    effectors.get_mut(companion).unwrap().effectors.push(attack_speed_effector);
                },
                _ => ..
            }
        }
    }
    Ok(())
}
