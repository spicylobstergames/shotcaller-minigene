use crate::*;

pub struct HeroTeleportRes {
    pub reader: ReaderId<InputEvent>,
}

system!(
    HeroTeleportSystem,
    |events: Read<'a, EventChannel<InputEvent>>,
     res: WriteExpect<'a, HeroTeleportRes>,
     positions: WriteStorage<'a, Point>,
     selected_hero: Read<'a, SelectedHero>,
     leaders: ReadStorage<'a, Leader>| {
        for k in events.read(&mut res.reader) {
            if let &InputEvent::Teleport(n) = k {
                let hero = selected_hero.0;
                for (mut pos, leader) in (&mut positions, &leaders).join() {
                    if leader.0 == hero {
                        // teleport to n
                        let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 7 * (n as i32 - 2);
                        let y = PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 / 8;
                        pos.x = x;
                        pos.y = y;
                    }
                }
            }
        }
    }
);
