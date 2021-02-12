use crate::*;

/// Spawns a pawn using the provided event.
pub fn spawn_pawn_system(
    game_events: &Vec<GameEvent>,
    stat_def: &StatDefinitions<Stats>,
    entities: &mut Entities,
    positions: &mut Components<Point>,
    pawns: &mut Components<Pawn>,
    simple_movements: &mut Components<MovementSystems>,
    proximity_attacks: &mut Components<ProximityAttackSystems>,
    stats: &mut Components<StatSet<Stats>>,
    teams: &mut Components<Team>,
    sprites: &mut Components<Sprite>,
    sprite_indices: &mut Components<SpriteIndex>,
    sights: &mut Components<LineOfSight>,
) -> SystemResult {
    for ev in game_events.iter() {
        if let GameEvent::SpawnPawn(pos, team) = ev {
            let pawn = entities.create();
            positions.insert(pawn, pos.clone());
            pawns.insert(pawn, Pawn);
            simple_movements.insert(pawn, MovementSystems::SimpleMovement);
            teams.insert(pawn, *team);
            stats.insert(pawn, stat_def.to_statset());
            proximity_attacks.insert(
                pawn,
                ProximityAttackSystems::SimpleProximityAttack(PAWN_ATTACK_RADIUS),
            );
            let bg = if *team == Team::Me {
                sights.insert(pawn, LineOfSight::new(5));
                RGBA::named(GREEN)
            } else {
                RGBA::named(RED)
            };
            sprites.insert(
                pawn,
                Sprite {
                    glyph: to_cp437('c'),
                    fg: RGBA::named(YELLOW),
                    bg,
                },
            );
            sprite_indices.insert(pawn, SpriteIndex(9));
        }
    }
    Ok(())
}
