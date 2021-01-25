use crate::*;

/// Sets which tiles are visible.
pub fn fog_of_vision_system(
    collision_res: &Option<CollisionResource>,
    sights: &Components<LineOfSight>,
    viewshed: &mut Viewshed,
    positions: &mut Components<Point>,
) -> SystemResult{
    viewshed.visible_tiles.clear();
    for (sight, pos) in join!(&sights && &positions).map(|(s, p)| (s.unwrap(), p.unwrap())) {
        if let Some(col_res) = collision_res {
            let map = &col_res.map;
            viewshed.visible_tiles.extend(field_of_view(*pos, sight.range, map));
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < (map.size().0 as i32) && p.y >= 0 && p.y < (map.size().1 as i32));
        }
    }
    Ok(())
}
