use crate::*;

/// Spawn an elephant companion at a specific location
pub fn elephant_spawner_system(
    events: &Vec<SkillTriggerEvent<Skills>>,
    stat_def: &StatDefinitions<Stats>,
    teams: &mut Components<Team>,
    proximity_attacks: &mut Components<ProximityAttack>,
    simple_movements: &mut Components<MovementSystems>,
    stats: &mut Components<StatSet<Stats>>,
    positions: &mut Components<Point>,
    entities: &mut Entities,
    companions: &mut Components<Companion>,
    sprites: &mut Components<Sprite>,
    sprite_indices: &mut Components<SpriteIndex>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::ElephantSummon {
            let pos = positions.get(ev.0).unwrap().clone();
            let team = teams.get_mut(ev.0).unwrap().clone();

            let elephant = entities.create();
            positions.insert(elephant, pos.clone());
            companions.insert(ev.0, Companion::Elephant(elephant));
            simple_movements.insert(elephant, MovementSystems::SimpleMovement);
            teams.insert(elephant, team);
            stats.insert(elephant, stat_def.to_statset());
            proximity_attacks.insert(elephant, ProximityAttack::new(CREEP_ATTACK_RADIUS));
            let bg = if team == Team::Me {
                RGBA::named(GREEN)
            } else {
                RGBA::named(RED)
            };
            sprites.insert(
                elephant,
                Sprite {
                    glyph: to_cp437('c'),
                    fg: RGBA::named(YELLOW),
                    bg,
                },
            );
            sprite_indices.insert(elephant, SpriteIndex(15));
        }
    }
    Ok(())
}
