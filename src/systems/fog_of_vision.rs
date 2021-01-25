use crate::*;

/// Sets which tiles are visible.
pub fn fog_of_vision_system(
    collision_res: &Option<CollisionResource>,
    viewsheds: &mut Components<Viewshed>,
    positions: &mut Components<Point>,
) -> SystemResult{
    for (v, p) in join!(&mut viewsheds && &positions).map(|(v, p)| (v.unwrap(), p.unwrap())) {
        if let Some(col_res) = collision_res {
            let map = &col_res.map;
            v.visible_tiles.clear();
            v.visible_tiles = field_of_view(Point::new(p.x, p.y), v.range, map);
            v.visible_tiles.retain(|p| p.x >= 0 && p.x < (map.size().0 as i32) && p.y >= 0 && p.y < (map.size().1 as i32));
        }
    }
    Ok(())
}
