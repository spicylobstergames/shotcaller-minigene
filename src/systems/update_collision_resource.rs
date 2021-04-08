use crate::*;

// non portable
/// Updates the global `CollisionResource` using the background map data.
pub fn update_collision_resource_system(
    positions: &Components<Point>,
    players: &Components<Player>,
    global_map: &mut Option<CollisionResource>,
) -> SystemResult {
    let global_map = global_map.as_mut().unwrap();
    for j in 0..(PLAY_HEIGHT as usize) {
        MAP_STRING[j].char_indices().for_each(|(i, c)| {
            if c == '#' {
                global_map.map.set(i as u32, j as u32);
            } else {
                global_map.map.unset(i as u32, j as u32);
            }
        });
    }
    // TODO fix this
    for (pos, _) in join!(&positions && &players) {
        global_map.position.x = pos.unwrap().x - 40;
        global_map.position.y = pos.unwrap().y - 25;
    }
    Ok(())
}
