use crate::*;

/// The AI for the tower attack.
// TODO separate  creation of the projectile entities from the tower's decision to attack.
pub fn tower_ai_system(
    stat_def: &StatDefinitions<Stats>,
    towers: &Components<Tower>,
    entities: &mut Entities,
    teams: &mut Components<Team>,
    tower_projectiles: &mut Components<TowerProjectile>,
    sprites: &mut Components<Sprite>,
    sprite_indices: &mut Components<SpriteIndex>,
    stats: &mut Components<StatSet<Stats>>,
    goto_positions: &mut Components<GotoStraight>,
    positions: &mut Components<Point>,
) -> SystemResult {
    let mut v = vec![];
    for (_, team, pos) in join!(&towers && &teams && &positions) {
        // find closest in other team
        // TODO: optimize
        let closest = find_closest_in_other_team(
            team.unwrap(),
            pos.unwrap(),
            &teams,
            &positions,
            &stats,
            &entities,
        );
        if let Some((_, c)) = closest {
            if dist(&c, pos.unwrap()) <= TOWER_RANGE {
                v.push((pos.unwrap().clone(), *team.unwrap(), c.clone()))
            }
        }
    }
    for (source, team, target) in v.into_iter() {
        let n = entities.create();
        positions.insert(n, source).unwrap();
        tower_projectiles.insert(n, TowerProjectile).unwrap();
        teams.insert(n, team).unwrap();
        stats.insert(n, stat_def.to_statset()).unwrap();
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
        sprite_indices.insert(n, SpriteIndex(TileMapping::Fireball.into())).unwrap();
        goto_positions
            .insert(n, GotoStraight::new(target.clone(), 1.0))
            .unwrap();
    }
    Ok(())
}
