use crate::*;

/// Spawn a bear companion at a specific location
pub fn bear_spawner_system(
    events: &Vec<SkillTriggerEvent<Skills>>,
    stat_def: &StatDefinitions<Stats>,
    teams: &mut Components<Team>,
    proximity_attacks: &mut Components<ProximityAttack>,
    simple_movements: &mut Components<SimpleMovement>,
    stats: &mut Components<StatSet<Stats>>,
    positions: &mut Components<Point>,
    entities: &mut Entities,
    companions: &mut Components<Companion>,
    sprites: &mut Components<Sprite>,
    sprite_indices: &mut Components<SpriteIndex>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::BearSummon {
            let pos = positions.get(ev.0).unwrap().clone();
            let team = teams.get_mut(ev.0).unwrap().clone();

            let bear = entities.create();
            positions.insert(bear, pos.clone());
            companions.insert(ev.0, Companion(Unit::Bear(bear)));
            simple_movements.insert(bear, SimpleMovement);
            teams.insert(bear, team);
            stats.insert(bear, stat_def.to_statset());
            proximity_attacks.insert(bear, ProximityAttack::new(CREEP_ATTACK_RADIUS));
            let bg = if team == Team::Me {
                RGBA::named(GREEN)
            } else {
                RGBA::named(RED)
            };
            sprites.insert(
                bear,
                Sprite {
                    glyph: to_cp437('c'),
                    fg: RGBA::named(YELLOW),
                    bg,
                },
            );
            sprite_indices.insert(bear, SpriteIndex(15));
        }
    }
    Ok(())
}
