use crate::*;

/// Increase the soul stat whenever a entity was killed.
pub fn steal_soul_system(
    stats: &mut Components<StatSet<Stats>>,
    game_events: &mut Vec<GameEvent>,
    events: &mut Vec<SkillTriggerEvent<Skills>>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::StealSoul {
            let mut additional_attack = 0.0;
            for game_ev in game_events.iter() {
                if let GameEvent::KillEntity(_) = game_ev {
                    let entity_stats = stats.get_mut(ev.0).unwrap();
                    if entity_stats.stats.get(&Stats::Souls).unwrap().value < 20.0 {
                        entity_stats.stats.get_mut(&Stats::Souls).unwrap().value += 1.0;
                        additional_attack += 1.0;
                    }
                }
            }
            game_events.push(GameEvent::AdditionalAttack(ev.0, additional_attack));
        }
    }

    Ok(())
}
