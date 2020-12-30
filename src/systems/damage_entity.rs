use crate::*;

pub fn damage_entity_system(
        positions: &Components<Point>,
        teams: &Components<Team>,
        entities: &Entities,
        events: &Vec<SkillTriggerEvent<Skills>>,
        stats: &mut Components<StatSet<Stats>>,
        game_events: &mut Vec<GameEvent>) -> SystemResult {
    let mut out_ev = vec![];
    for ev in game_events.iter() {
        if let GameEvent::DamageEntity(e, dmg) = ev {
            if let Some(mut stat) = stats.get_mut(*e) {
                damage(&mut stat, *dmg);
                if stat.stats.get(&Stats::Health).unwrap().value <= 0.0 {
                    out_ev.push(GameEvent::KillEntity(*e));
                }
            }
        }
    }
    for ev in out_ev {
        game_events.push(ev);
    }
    Ok(())
}

