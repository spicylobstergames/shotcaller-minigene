use crate::*;

/// Moves the entity to next point on AIPath, provided we have enough action points to do so.
pub fn movement_system(
    entities: &Entities,
    global_map: &Option<CollisionResource>,
    positions: &mut Components<Point>,
    stats: &mut Components<StatSet<Stats>>,
    paths: &mut Components<AiPath>,
) -> SystemResult {
    'query: for (e, pos, path) in join!(&entities && &mut positions && &mut paths) {
        if stats
            .get(e.unwrap())
            .unwrap()
            .stats
            .get(&Stats::ActionPoints)
            .unwrap()
            .value
            < ACTION_POINT_MOVE_COST
        {
            continue 'query;
        }
        let pos = pos.unwrap();
        let path = path.unwrap();
        if path.path.success && path.path.steps.len() > 1 {
            let dest = path.path.steps.remove(1);
            let (x, y) = global_map.as_ref().unwrap().map.position_of(dest as u32);
            *pos = Point::new(
                x as i32 + global_map.as_ref().unwrap().position.x,
                y as i32 + global_map.as_ref().unwrap().position.y,
            );

            // Update action points usage after unit has moved
            stats
                .get_mut(e.unwrap())
                .unwrap()
                .stats
                .get_mut(&Stats::ActionPoints)
                .unwrap()
                .value -= ACTION_POINT_MOVE_COST;
        }
    }

    Ok(())
}
