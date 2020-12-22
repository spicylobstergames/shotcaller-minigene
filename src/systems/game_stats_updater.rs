use crate::*;

event_reader_res!(GameStatsUpdaterRes, GameEvent);
system!(GameStatsUpdaterSystem, |res: WriteExpect<'a, GameStatsUpdaterRes>,
        game_stats: Write<'a, GameStats>,
        game_events: Read<'a, EventChannel<GameEvent>>| {
    for ev in game_events.read(&mut res.0) {
        match ev {
            GameEvent::DamageEntity(_, dmg) => game_stats.damage_dealt += dmg,
            GameEvent::KillEntity(_) => game_stats.kill_count += 1,
            _ => {},
        }
    }
});

