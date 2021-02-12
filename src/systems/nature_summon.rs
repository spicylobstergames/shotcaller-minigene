use crate::*;

/// Spawn a treant at a specific location
pub fn nature_summon_system(
    positions: &Components<Point>,
    teams: &Components<Team>,
    events: &Vec<SkillTriggerEvent<Skills>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::NatureSummon {
            let pos = positions.get(ev.0).unwrap();
            let team = teams.get(ev.0).unwrap();

            //@TODO: spawn an actual treant pawn
            game_events.push(GameEvent::SpawnPawn(*pos, *team));
        }
    }
    Ok(())
}
