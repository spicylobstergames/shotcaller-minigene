use crate::*;

/// Update additional attack stat.
pub fn additional_attack_system(
    stats: &mut Components<StatSet<Stats>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    for ev in game_events.iter() {
        if let GameEvent::AdditionalAttack(e, a) = ev {
            let entity_stats = stats.get_mut(*e).unwrap();
            entity_stats.stats.get_mut(&Stats::AdditionalAttack).unwrap().value += a;
        }
    }

    Ok(())
}
