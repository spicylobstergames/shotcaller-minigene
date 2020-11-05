system!(
    TowerAiSystem,
    |entities: Entities<'a>,
     stat_def: ReadExpect<'a, StatDefinitions<Stats>>,
     towers: ReadStorage<'a, Tower>,
     teams: WriteStorage<'a, Team>,
     tower_projectiles: WriteStorage<'a, TowerProjectile>,
     sprites: WriteStorage<'a, Sprite>,
     sprite_indices: WriteStorage<'a, SpriteIndex>,
     stats: WriteStorage<'a, Comp<StatSet<Stats>>>,
     goto_positions: WriteStorage<'a, GotoStraight>,
     positions: WriteStorage<'a, Point>| {
        let mut v = vec![];
        for (_, team, pos) in (&towers, &teams, &positions).join() {
            // find closest in other team
            // TODO: optimize
            let mut vec = (&teams, &positions)
                .join()
                .filter(|(t, _)| **t != *team)
                .map(|(_, p)| (dist(pos, p), p.clone()))
                .filter(|(d, _)| *d < TOWER_RANGE)
                .collect::<Vec<_>>();
            vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
            let closest = vec.into_iter().next().map(|(d, p)| p);
            if let Some(c) = closest {
                v.push((pos.clone(), *team, c.clone()))
            }
        }
        for (source, team, target) in v.into_iter() {
            let n = entities.create();
            positions.insert(n, source).unwrap();
            tower_projectiles.insert(n, TowerProjectile).unwrap();
            teams.insert(n, team).unwrap();
            stats.insert(n, Comp(stat_def.to_statset())).unwrap();
            sprites
                .insert(
                    n,
                    Sprite {
                        glyph: to_cp437('X'),
                        fg: RGBA::named(RED),
                        bg: RGBA::named(WHITE),
                    },
                )
                .unwrap();
            sprite_indices.insert(n, SpriteIndex(85)).unwrap();
            goto_positions
                .insert(n, GotoStraight::new(target.clone(), 1.0))
                .unwrap();
        }
    }
);
