use crate::*;

/// Kill the entity specified by the event.
/// TODO If it happens to be a leader, set its respawn time and save its inventory.
pub fn kill_entity_system(
    leaders: &Components<Leader>,
    uuids: &Components<Uuid>,
    entities: &mut Entities,
    events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut out_ev = vec![];
    for ev in events.iter() {
        if let GameEvent::KillEntity(e) = ev {
            if let Some(Leader(leader_id)) = leaders.get(*e) {
                let uuid_opt = uuids.get(*e).cloned();
                // We killed a leader, create a new event.
                out_ev.push(GameEvent::LeaderDied(*leader_id, uuid_opt));
            }
            entities.kill(*e);
        }
    }
    for ev in out_ev {
        events.push(ev);
    }
    Ok(())
}
