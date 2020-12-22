use crate::*;

event_reader_res!(KillEntityRes, GameEvent);

// Kill the entity specified by the event.
// If it happens to be a leader, set its respawn time and save its inventory.
system!(
    KillEntitySystem, |res: WriteExpect<'a, KillEntityRes>,
        stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
        positions: ReadStorage<'a, Point>,
        teams: ReadStorage<'a, Team>,
        leaders: ReadStorage<'a, Leader>,
        entities: Entities<'a>,
        events: Write<'a, EventChannel<GameEvent>>| {
    let mut out_ev = vec![];
    for ev in events.read(&mut res.0) {
        if let GameEvent::KillEntity(e) = ev {
            if let Some(Leader(leader_id)) = leaders.get(*e) {
                // We killed a leader, create a new event.
                out_ev.push(GameEvent::LeaderDied(*leader_id));
            }
            entities.delete(*e).expect("Failed to delete entity after damaging it.");
        }
    }
    for ev in out_ev {
        events.single_write(ev);
    }
});
