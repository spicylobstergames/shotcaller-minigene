use crate::*;

event_reader_res!(DamageEntityRes, GameEvent);
system!(DamageEntitySystem, |res: WriteExpect<'a, DamageEntityRes>,
        stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
        positions: ReadStorage<'a, Point>,
        teams: ReadStorage<'a, Team>,
        entities: Entities<'a>,
        events: Read<'a, EventChannel<SkillTriggerEvent<Skills>>>,
        game_events: Write<'a, EventChannel<GameEvent>>| {
    let mut out_ev = vec![];
    for ev in game_events.read(&mut res.0) {
        if let GameEvent::DamageEntity(e, dmg) = ev {
            if let Some(stat) = stats.get_mut(*e) {
                damage(&mut stat.0, *dmg);
                if stat.0.stats.get(&Stats::Health).unwrap().value <= 0.0 {
                    out_ev.push(GameEvent::KillEntity(*e));
                }
            }
        }
    }
    for ev in out_ev {
        game_events.single_write(ev);
    }
});

