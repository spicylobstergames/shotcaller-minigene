use crate::*;

pub fn hero_teleport_system(
    events: &Vec<InputEvent>,
    selected_hero: &SelectedHero,
    leaders: &Components<Leader>,
    positions: &mut Components<Point>,
) -> SystemResult {
    for k in events.iter() {
        if let &InputEvent::Teleport(n) = k {
            let hero = selected_hero.0;
            for (mut pos, leader) in join!(&mut positions && &leaders) {
                let pos = pos.as_mut().unwrap();
                if leader.unwrap().0 == hero {
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
