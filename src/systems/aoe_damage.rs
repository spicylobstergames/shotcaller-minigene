use crate::*;

event_reader_res!(AoeDamageRes, SkillTriggerEvent<Skills>);
system!(AoeDamageSystem, |res: WriteExpect<'a, AoeDamageRes>,
        stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
        positions: ReadStorage<'a, Point>,
        teams: ReadStorage<'a, Team>,
        entities: Entities<'a>,
        events: Read<'a, EventChannel<SkillTriggerEvent<Skills>>>,
        game_events: Write<'a, EventChannel<GameEvent>>| {
    for ev in events.read(&mut res.0) {
        if ev.1 == Skills::AOE {
            // Damage around
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                for (e, _, _) in entities_in_radius(from, &*entities, &positions, 
                                            |e,_| teams.get(e).map(|t| t != team).unwrap_or(false), |_,_,d| d <= AOE_RADIUS) {
                    game_events.single_write(GameEvent::DamageEntity(e, AOE_DAMAGE));
                }
            }
        }
    }
});
