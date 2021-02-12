use crate::*;

/// Periodically generates events to spawn pawns.
pub fn pawn_spawner_system(
    positions: &mut Components<Point>,
    spawners: &mut Components<PawnSpawner>,
    teams: &mut Components<Team>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    for (pos, mut spawner, team) in join!(&positions && &mut spawners && &teams) {
        let spawner = spawner.as_mut().unwrap();
        if spawner.0 == 0 {
            spawner.0 = spawner.1;
            // spawn
            game_events.push(GameEvent::SpawnPawn(
                pos.unwrap().clone(),
                team.unwrap().clone(),
            ));
        }
        spawner.0 -= 1;
    }
    Ok(())
}
