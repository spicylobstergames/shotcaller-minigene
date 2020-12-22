use crate::*;

system!(
    CreepSpawnerSystem,
    |entities: Entities<'a>,
     positions: WriteStorage<'a, Point>,
     spawners: WriteStorage<'a, CreepSpawner>,
     creeps: WriteStorage<'a, Creep>,
     simple_movements: WriteStorage<'a, SimpleMovement>,
     ai_destinations: WriteStorage<'a, AiDestination>,
     proximity_attacks: WriteStorage<'a, ProximityAttack>,
     stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
     stat_def: ReadExpect<'a, StatDefinitions<Stats>>,
     ai_paths: WriteStorage<'a, AiPath>,
     teams: WriteStorage<'a, Team>,
     sprites: WriteStorage<'a, Sprite>,
     sprite_indices: WriteStorage<'a, SpriteIndex>,
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
