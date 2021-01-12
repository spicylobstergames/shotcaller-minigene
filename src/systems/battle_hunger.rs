use crate::*;

/// Slows and damages the closest enemy unit for 5 seconds. Caster gains movement speed.
/// TODO: End duration if target kills another unit
pub fn battle_hunger_system(
    positions: &Components<Point>,
    teams: &Components<Team>,
    entities: &Entities,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    events: &mut Vec<SkillTriggerEvent<Skills>>,
    stats: &mut Components<StatSet<Stats>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    for ev in events.iter() {
        for e in join!(&entities) {
            let e = e.unwrap();
            if let Some(effectors) = effectors.get(e) {
                for effector in &effectors.effectors {
                    if effector.effector_key == Effectors::Enraged {
                        game_events.push(GameEvent::DamageEntity(e, 1.0));
                    }
                }
            }
        }
        if ev.1 == Skills::BattleHunger {
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                let enemies_around = entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= AOE_RADIUS,
                );
                let closest_enemy = enemies_around.first().unwrap().0;
                let enraged_effector = effector_defs
                    .defs
                    .get(&Effectors::Enraged)
                    .expect("Unknown effector key.");

                if effectors.get(closest_enemy).is_none() {
                    effectors.insert(closest_enemy, EffectorSet::default());
                }

                effectors
                    .get_mut(closest_enemy)
                    .unwrap()
                    .effectors
                    .push(EffectorInstance::new(
                        Effectors::Enraged,
                        enraged_effector.duration,
                    ))
            }
        }
    }
    Ok(())
}
