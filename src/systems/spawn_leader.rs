use crate::*;


// TODO
pub fn 
    spawn_leader_system(
game_events: &Vec<GameEvent>,
     stat_def: &Option<StatDefinitions<Stats>>,
     entities: &mut Entities,
     positions: &mut Components<Point>,
     spawners: &mut Components<CreepSpawner>,
     creeps: &mut Components<Creep>,
     simple_movements: &mut Components<SimpleMovement>,
     ai_destinations: &mut Components<AiDestination>,
     proximity_attacks: &mut Components<ProximityAttack>,
     stats: &mut Components<StatSet<Stats>>,
     ai_paths: &mut Components<AiPath>,
     teams: &mut Components<Team>,
     sprites: &mut Components<Sprite>,
     sprite_indices: &mut Components<SpriteIndex>,
     ) -> SystemResult{
        for ev in game_events.iter() {
            if let GameEvent::SpawnCreep(pos, team) = ev {
                let creep = entities.create();
                positions.insert(creep, pos.clone()).unwrap();
                creeps.insert(creep, Creep).unwrap();
                simple_movements.insert(creep, SimpleMovement).unwrap();
                ai_paths
                    .insert(creep, AiPath::new(NavigationPath::new()))
                    .unwrap();
                teams.insert(creep, *team).unwrap();
                stats.insert(creep, stat_def.unwrap().to_statset()).unwrap();
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
        Ok(())
     }
