use crate::*;

/// Respawns a leader that has died.
pub fn respawn_leader_driver(events: &mut Vec<GameEvent>) -> SystemResult {
    let mut o = vec![];
    for ev in events.iter() {
        if let GameEvent::LeaderDied(id) = ev {
            let pos = if *id < 5 {
                Point::new(LEADER_SPAWN_POINT_ME.0, LEADER_SPAWN_POINT_ME.1)
            } else {
                Point::new(LEADER_SPAWN_POINT_OTHER.0, LEADER_SPAWN_POINT_OTHER.1)
            };
            o.push(GameEvent::SpawnLeader(pos, *id));
        }
    }
    for ev in o {
        events.push(ev);
    }
    Ok(())
}
