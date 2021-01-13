use crate::*;

/// Update the game statistics using the generated game events.
pub fn game_stats_updater_system(
    game_events: &Vec<GameEvent>,
    game_stats: &mut GameStats,
) -> SystemResult {
    for ev in game_events.iter() {
        match ev {
            GameEvent::DamageEntity(_, _, dmg) => game_stats.damage_dealt += dmg,
            GameEvent::KillEntity(_) => game_stats.kill_count += 1,
            GameEvent::TransferedGold(_, gold) => game_stats.earned_gold += gold,
            _ => {}
        }
    }
    Ok(())
}
