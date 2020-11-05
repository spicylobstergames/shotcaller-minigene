pub struct HeroTeleportRes {
    pub reader: ReaderId<InputEvent>,
    pub selected_hero: Option<u8>,
}

system!(
    HeroTeleportSystem,
    |events: Read<'a, EventChannel<InputEvent>>,
     res: WriteExpect<'a, HeroTeleportRes>,
     positions: WriteStorage<'a, Point>,
     leaders: ReadStorage<'a, Leader>| {
        for k in events.read(&mut res.reader) {
            if let &InputEvent::Teleport(n) = k {
                if let Some(hero) = res.selected_hero {
                    if n >= 1 && n <= 3 {
                        for (mut pos, leader) in (&mut positions, &leaders).join() {
                            if leader.0 == hero {
                                // teleport to n
                                let x = PLAY_WIDTH as i32 / 2 + PLAY_WIDTH as i32 / 7 * (n as i32 - 2);
                                let y = PLAY_HEIGHT as i32 - 1 - PLAY_HEIGHT as i32 / 8;
                                pos.x = x;
                                pos.y = y;
                            }
                        }
                        res.selected_hero = None;
                    }
                } else {
                    if n >= 1 && n <= 5 {
                        res.selected_hero = Some(n);
                    }
                }
            }
        }
    }
);
