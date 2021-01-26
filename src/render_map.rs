use crate::*;

/// Renders the background map on the terminal.
pub fn render<'a>(ctx: &mut BTerm) {
    let mut i = 0;
    for s in MAP {
        ctx.print(0, i, s);
        i = i + 1;
    }
}

/// Creates the map background 2d sprite entities.
pub fn create_map_bg<'a>(world: &mut World) {
    let mut i = 0;
    for s in MAP {
        let mut j = 0;
        for c in s.chars() {
            if c == '#' {
                centity!(world, SpriteIndex(55), Point::new(j, i),);
            }
            j = j + 1;
        }
        i = i + 1;
    }
}

/// Renders the user interface on the screen.
pub fn render_ui(_world: &mut World, _ctx: &mut BTerm) {
    #[cfg(not(feature = "headless"))]
    {
        _ctx.draw_box(
            PLAY_WIDTH,
            0,
            SCREEN_WIDTH - PLAY_WIDTH - 1,
            SCREEN_HEIGHT - 1,
            WHITE,
            BLACK,
        );
        _ctx.print(PLAY_WIDTH + 1, 1, "Leaders");
        _ctx.print(PLAY_WIDTH + 1, 3, "My Team");

        let selected = _world.get::<SelectedLeader>().unwrap().0;

        for (i, key) in _world.get::<TeamLeaders>().unwrap().me.iter().enumerate() {
            let name = _world
                .get::<LeaderDefinitions>()
                .unwrap()
                .defs
                .get(key)
                .unwrap()
                .name
                .clone();
            _ctx.print(PLAY_WIDTH + 1, i + 4, format!(" {}", name));
        }
        _ctx.print(PLAY_WIDTH + 1, 10, "Enemy Team");
        for (i, key) in _world.get::<TeamLeaders>().unwrap().me.iter().enumerate() {
            let name = _world
                .get::<LeaderDefinitions>()
                .unwrap()
                .defs
                .get(key)
                .unwrap()
                .name
                .clone();
            _ctx.print(PLAY_WIDTH + 1, i + 11, format!(" Leader {}", name));
        }

        _ctx.print(PLAY_WIDTH + 1, selected + 4, ">");

        _ctx.print(PLAY_WIDTH + 1, 17, "Keybinds");

        let hm = _world.get::<HashMap<char, InputEvent>>().unwrap();
        let mut keybinds = hm.iter().collect::<Vec<_>>();
        keybinds.sort_by(|t1, t2| format!("{:?}", t1.1).cmp(&format!("{:?}", t2.1)));
        for (idx, (k, v)) in keybinds.iter().enumerate() {
            if **k as u32 == 13 {
                _ctx.print(PLAY_WIDTH + 1, 18 + idx, format!("Enter:{:?}", v));
            } else if **k as u32 == 27 {
                _ctx.print(PLAY_WIDTH + 1, 18 + idx, format!("Esc:{:?}", v));
            } else {
                _ctx.print(PLAY_WIDTH + 1, 18 + idx, format!("{}:{:?}", k, v));
            }
        }

        let game_stats = _world.get::<GameStats>().unwrap();
        _ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 7, "Total Damage");
        _ctx.print(
            PLAY_WIDTH + 1,
            SCREEN_HEIGHT - 6,
            format!("{:.2}", game_stats.damage_dealt),
        );
        _ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 5, "Kills");
        _ctx.print(
            PLAY_WIDTH + 1,
            SCREEN_HEIGHT - 4,
            format!("{}", game_stats.kill_count),
        );
        _ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 3, "Earned Gold");
        _ctx.print(
            PLAY_WIDTH + 1,
            SCREEN_HEIGHT - 2,
            format!("{}", game_stats.earned_gold),
        );
    }
}
