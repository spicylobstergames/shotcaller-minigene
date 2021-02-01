use crate::*;

/// Applies the area of effect damages to entities around the specified location.
pub fn aoe_damage_system(
    positions: &Components<Point>,
    teams: &Components<Team>,
    entities: &Entities,
    events: &Vec<SkillTriggerEvent<Skills>>,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::AOE {
            // Damage around
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                for (e, _, _) in entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= AOE_RADIUS,
                ) {
                    game_events.push(GameEvent::DamageEntity(ev.0, e, AOE_DAMAGE));
                }
            }
        } else if ev.1 == Skills::SlowAOE {
            // Damage around and apply effector
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                for (e, _, _) in entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= SLOW_AOE_RADIUS,
                ) {
                    game_events.push(GameEvent::DamageEntity(ev.0, e, SLOW_AOE_DAMAGE));

                    let slow_effector = effector_defs
                        .defs
                        .get(&Effectors::HalfMovementSpeed)
                        .expect("Unknown effector key.");

                    if effectors.get(e).is_none() {
                        effectors.insert(e, EffectorSet::default());
                    }

                    effectors
                        .get_mut(e)
                        .unwrap()
                        .effectors
                        .push(EffectorInstance::new(
                            Effectors::HalfMovementSpeed,
                            slow_effector.duration,
                        ))
                }
            }
        } else if ev.1 == Skills::ReturnAOE {
            // Damage around
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                for (e, _, _) in entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= RETURN_AOE_RADIUS,
                ) {
                    game_events.push(GameEvent::DamageEntity(ev.0, e, RETURN_AOE_DAMAGE));
                }
            }
        } else if ev.1 == Skills::Telekinesis {
            // Apply effector at the location of the second-closest enemy
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                // Find the second-closest enemy
                let enemies_around = entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= RANGED_LEADER_ATTACK_RADIUS,
                );

                let target_position = positions.get(enemies_around.get(1).unwrap().0).unwrap();

                // Apply effector
                for (e, _, _) in entities_in_radius(
                    target_position,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= STUN_AOE_RADIUS,
                ) {
                    let stun_effector = effector_defs
                        .defs
                        .get(&Effectors::Stun)
                        .expect("Unknown effector key.");

                    if effectors.get(e).is_none() {
                        effectors.insert(e, EffectorSet::default());
                    }

                    effectors
                        .get_mut(e)
                        .unwrap()
                        .effectors
                        .push(EffectorInstance::new(
                            Effectors::Stun,
                            stun_effector.duration,
                        ))
                }
            }
        }
    }
    Ok(())
}
