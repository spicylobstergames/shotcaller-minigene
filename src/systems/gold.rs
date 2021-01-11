use crate::*;

/// Transfer gold from an entity to another.
pub fn gold_system(
    stats: &mut Components<StatSet<Stats>>,
    events: &mut Vec<GameEvent>,
) -> SystemResult {
    for ev in events.iter() {
        if let GameEvent::TransferGold(to, from) = ev {
            let total_earned = stats.get(*from).unwrap().stats.get(&Stats::Gold).unwrap().value;
            stats.get_mut(*from).unwrap().stats.get_mut(&Stats::Gold).unwrap().value = 0.0;
            stats.get_mut(*to).unwrap().stats.get_mut(&Stats::Gold).unwrap().value += total_earned;
        }
    }
    Ok(())
}
