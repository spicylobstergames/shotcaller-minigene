use crate::*;

/// Refills the action points of entities using their `ActionPointRefillRate` stat.
pub fn handle_action_points_system(stats: &mut Components<StatSet<Stats>>) -> SystemResult {
    for stat in stats.iter_mut() {
        let refill = stat
            .stats
            .get(&Stats::ActionPointRefillRate)
            .unwrap()
            .value_with_effectors;
        let stat = stat.stats.get_mut(&Stats::ActionPoints).unwrap();
        stat.value += refill;
        if stat.value >= 100.0 {
            stat.value = 100.0;
        }
    }
    Ok(())
}
