use crate::*;

/// Reduce the defense to half of all enemy units in AOE_RADIUS.
pub fn air_corrosion_system(
    positions: &Components<Point>,
    teams: &Components<Team>,
    entities: &Entities,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    events: &mut Vec<SkillTriggerEvent<Skills>>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::AirCorrosion {
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                let enemies_around = entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= AOE_RADIUS,
                );

                let half_def_effector = effector_defs
                    .defs
                    .get(&Effectors::HalfDefense)
                    .expect("Unknown effector key.");

                for e in enemies_around {
                    if effectors.get(e.0).is_none() {
                        effectors.insert(e.0, EffectorSet::default());
                    }

                    effectors
                        .get_mut(e.0)
                        .unwrap()
                        .effectors
                        .push(EffectorInstance::new(
                            Effectors::HalfDefense,
                            half_def_effector.duration,
                        ))
                }
            }
        }
    }

    Ok(())
}
