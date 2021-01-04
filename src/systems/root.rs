use crate::*;

/// Stops the movement of the closest enemy in AOE_RADIUS for 5s.
pub fn root_system(
    positions: &Components<Point>,
    teams: &Components<Team>,
    entities: &Entities,
    effector_defs: &EffectorDefinitions<Stats, Effectors>,
    effectors: &mut Components<EffectorSet<Effectors>>,
    events: &mut Vec<SkillTriggerEvent<Skills>>,
) -> SystemResult {
    for ev in events.iter() {
        if ev.1 == Skills::Root {
            if let (Some(from), Some(team)) = (positions.get(ev.0), teams.get(ev.0)) {
                let enemies_around = entities_in_radius(
                    from,
                    &*entities,
                    &positions,
                    |e, _| teams.get(e).map(|t| t != team).unwrap_or(false),
                    |_, _, d| d <= AOE_RADIUS,
                );
                let closest_enemy = enemies_around.first().unwrap().0;
                let root_effector = effector_defs
                    .defs
                    .get(&Effectors::Root)
                    .expect("Unknown effector key.");

                if effectors.get(closest_enemy).is_none() {
                    effectors.insert(closest_enemy, EffectorSet::default());
                }

                effectors
                    .get_mut(enemies_around.first().unwrap().0)
                    .unwrap()
                    .effectors
                    .push(EffectorInstance::new(
                        Effectors::Root,
                        root_effector.duration,
                    ))
            }
        }
    }

    Ok(())
}
