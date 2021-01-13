use crate::*;

/// Transfer gold from an entity to another.
pub fn transfer_gold_system(
    stats: &mut Components<StatSet<Stats>>,
    events: &mut Vec<GameEvent>,
) -> SystemResult {
    for ev in events.iter() {
        if let GameEvent::TransferGold(to, from, gold) = ev {
            let earned_gold = gold
                * stats
                    .get(*to)
                    .unwrap()
                    .stats
                    .get(&Stats::GoldGainMultiplier)
                    .unwrap()
                    .value;
            stats
                .get_mut(*from)
                .unwrap()
                .stats
                .get_mut(&Stats::Gold)
                .unwrap()
                .value -= gold;
            stats
                .get_mut(*to)
                .unwrap()
                .stats
                .get_mut(&Stats::Gold)
                .unwrap()
                .value += earned_gold;
        }
    }
    Ok(())
}
