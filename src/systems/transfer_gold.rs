use crate::*;

/// Transfer gold from an entity to another.
pub fn transfer_gold_system(
    stats: &mut Components<StatSet<Stats>>,
    events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut out_ev = Vec::new();
    for ev in events.iter() {
        if let GameEvent::TransferGold(to, from) = ev {
            let gold = stats
                .get(*from)
                .unwrap()
                .stats
                .get(&Stats::Gold)
                .unwrap()
                .value;
            let multiplied_gold = gold
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
                .value += multiplied_gold;

            out_ev.push(GameEvent::TransferedGold(*to, multiplied_gold));
        }
    }
    for ev in out_ev {
        events.push(ev);
    }
    Ok(())
}
