use crate::*;

/// Transfer gold from an entity to another.
pub fn transfer_gold_system(
    effectors: &mut Components<EffectorSet<Effectors>>,
    stats: &mut Components<StatSet<Stats>>,
    events: &mut Vec<GameEvent>,
) -> SystemResult {
    for ev in events.iter() {
        if let GameEvent::TransferGold(to, from, gold) = ev {
            let mut earned_gold = *gold;
            if let Some(effectors) = effectors.get(*to) {
                for e in &effectors.effectors {
                    if e.effector_key == Effectors::DoubleGold {
                        earned_gold *= 2.0;
                    }
                }

            }
            stats.get_mut(*from).unwrap().stats.get_mut(&Stats::Gold).unwrap().value -= gold;
            stats.get_mut(*to).unwrap().stats.get_mut(&Stats::Gold).unwrap().value += earned_gold;
        }
    }
    Ok(())
}
