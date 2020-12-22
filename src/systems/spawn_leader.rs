use crate::*;


// TODO
event_reader_res!(SpawnLeaderRes, GameEvent);
system!(
    SpawnLeaderSystem,
    |res: WriteExpect<'a, SpawnLeaderRes>,
     entities: Entities<'a>,
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
     game_events: Read<'a, EventChannel<GameEvent>>| {
        for ev in game_events.read(&mut res.0) {
            if let GameEvent::SpawnCreep(pos, team) = ev {
                let creep = entities.create();
                positions.insert(creep, pos.clone()).unwrap();
                creeps.insert(creep, Creep).unwrap();
                simple_movements.insert(creep, SimpleMovement).unwrap();
                ai_paths
                    .insert(creep, AiPath::new(NavigationPath::new()))
                    .unwrap();
                teams.insert(creep, *team).unwrap();
                stats.insert(creep, Comp(stat_def.to_statset())).unwrap();
                proximity_attacks
                    .insert(creep, ProximityAttack::new(CREEP_ATTACK_RADIUS))
                    .unwrap();
                let bg = if *team == Team::Me {
                    RGBA::named(GREEN)
                } else {
                    RGBA::named(RED)
                };
                sprites
                    .insert(
                        creep,
                        Sprite {
                            glyph: to_cp437('c'),
                            fg: RGBA::named(YELLOW),
                            bg,
                        },
                    )
                    .unwrap();
                sprite_indices.insert(creep, SpriteIndex(9)).unwrap();
        }
    }
     }
);
