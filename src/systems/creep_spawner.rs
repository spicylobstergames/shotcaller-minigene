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
     sprite_indices: WriteStorage<'a, SpriteIndex>| {
        let mut v = vec![];
        for (pos, mut spawner, team) in (&positions, &mut spawners, &teams).join() {
            if spawner.0 == 0 {
                spawner.0 = spawner.1;
                // spawn
                v.push((pos.clone(), team.clone()));
            }
            spawner.0 -= 1;
        }
        v.into_iter().for_each(|(pos, team)| {
            let creep = entities.create();
            positions.insert(creep, pos.clone()).unwrap();
            creeps.insert(creep, Creep).unwrap();
            simple_movements.insert(creep, SimpleMovement).unwrap();
            ai_paths
                .insert(creep, AiPath::new(NavigationPath::new()))
                .unwrap();
            teams.insert(creep, team).unwrap();
            stats.insert(creep, Comp(stat_def.to_statset())).unwrap();
            proximity_attacks
                .insert(creep, ProximityAttack::new(CREEP_ATTACK_RADIUS))
                .unwrap();
            let bg = if team == Team::Me {
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
        });
    }
);
