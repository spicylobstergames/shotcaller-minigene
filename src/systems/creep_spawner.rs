use crate::*;

system!(
    CreepSpawnerSystem, |
     positions: WriteStorage<'a, Point>,
     spawners: WriteStorage<'a, CreepSpawner>,
     teams: WriteStorage<'a, Team>,
     game_events: Write<'a, EventChannel<GameEvent>>| {
        for (pos, mut spawner, team) in (&positions, &mut spawners, &teams).join() {
            if spawner.0 == 0 {
                spawner.0 = spawner.1;
                // spawn
                game_events.single_write(GameEvent::SpawnCreep(pos.clone(), team.clone()));
            }
            spawner.0 -= 1;
        }
    }
);
