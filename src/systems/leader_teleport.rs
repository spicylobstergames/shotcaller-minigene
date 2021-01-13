use crate::*;

/// Teleports the selected leader to the location specified in the event.
pub fn leader_teleport_system(
    events: &Vec<InputEvent>,
    selected_leader: &SelectedLeader,
    leaders: &Components<Leader>,
    positions: &mut Components<Point>,
) -> SystemResult {
    for k in events.iter() {
        if let &InputEvent::Teleport(n) = k {
            let selected_leader = selected_leader.0;
            for (mut pos, leader) in join!(&mut positions && &leaders) {
                let pos = pos.as_mut().unwrap();
                if leader.unwrap().0 == selected_leader {
                    // teleport to n
                    let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 7 * (n as i32 - 2);
                    let y = PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 / 8;
                    pos.x = x;
                    pos.y = y;
                }
            }
        }
    }
    Ok(())
}
