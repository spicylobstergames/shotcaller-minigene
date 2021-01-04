use crate::*;

pub fn reset_stat_effectors_system(stats: &mut Components<StatSet<Stats>>) -> SystemResult {
         for stat in stats.iter_mut() {
            for mut s in stat.stats.values_mut() {
                s.value_with_effectors = s.value;
            }
         }
    Ok(())
}
