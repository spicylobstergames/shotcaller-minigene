use crate::*;

/// Set the AdditionalAttack stat.
pub fn additional_attack_system(
    effectors: &mut Components<EffectorSet<Effectors>>,
    stats: &mut Components<StatSet<Stats>>,
    game_events: &mut Vec<GameEvent>,
    events: &mut Vec<SkillTriggerEvent<Skills>>,
) -> SystemResult {
    for e in join!(effectors) {

    }
    effectors.get(*to) {
        for e in &effectors.effectors {
            if e.effector_key == Effectors:: {
                earned_gold *= 2.0;
            }
        }

    }
    Ok(())
}
