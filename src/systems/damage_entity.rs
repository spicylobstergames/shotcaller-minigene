use crate::*;

/// Applies damage events to entities.
/// Can emit events to kill the entity if it is out of health.
pub fn damage_entity_system(
    stats: &mut Components<StatSet<Stats>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut out_ev = vec![];
    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(a, e, dmg) = ev {
            if let Some(mut stat) = stats.get_mut(*e) {
                if stat.stats.get(&Stats::Health).unwrap().value > 0.0 {
                    damage(&mut stat, *dmg);
                    if stat.stats.get(&Stats::Health).unwrap().value <= 0.0 {
                        out_ev.push(GameEvent::TransferGold(*a, *e));
                        out_ev.push(GameEvent::KillEntity(*e));
                    }
                }
            }
        }
    }
    for ev in out_ev {
        game_events.push(ev);
    }
    Ok(())
}
