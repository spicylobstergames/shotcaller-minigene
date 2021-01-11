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
pub fn render_ui(world: &mut World, ctx: &mut BTerm) {
    ctx.draw_box(
        PLAY_WIDTH,
        0,
        SCREEN_WIDTH - PLAY_WIDTH - 1,
        SCREEN_HEIGHT - 1,
        WHITE,
        BLACK,
    );
    ctx.print(PLAY_WIDTH + 1, 1, "Leaders");
    ctx.print(PLAY_WIDTH + 1, 3, "My Team");

    let selected = world.get::<SelectedLeader>().unwrap().0;

    for (i, key) in world.get::<TeamLeaders>().unwrap().me.iter().enumerate() {
        let name = world
            .get::<LeaderDefinitions>()
            .unwrap()
            .defs
            .get(key)
            .unwrap()
            .name
            .clone();
        ctx.print(PLAY_WIDTH + 1, i + 4, format!(" {}", name));
    }
    ctx.print(PLAY_WIDTH + 1, 10, "Enemy Team");
    for (i, key) in world.get::<TeamLeaders>().unwrap().me.iter().enumerate() {
        let name = world
            .get::<LeaderDefinitions>()
            .unwrap()
            .defs
            .get(key)
            .unwrap()
            .name
            .clone();
        ctx.print(PLAY_WIDTH + 1, i + 11, format!(" Leader {}", name));
    }

    ctx.print(PLAY_WIDTH + 1, selected + 4, ">");

    ctx.print(PLAY_WIDTH + 1, 17, "Keybinds");

    let hm = world.get::<HashMap<char, InputEvent>>().unwrap();
    let mut keybinds = hm.iter().collect::<Vec<_>>();
    keybinds.sort_by(|t1, t2| format!("{:?}", t1.1).cmp(&format!("{:?}", t2.1)));
    for (idx, (k, v)) in keybinds.iter().enumerate() {
        if **k as u32 == 13 {
            ctx.print(PLAY_WIDTH + 1, 18 + idx, format!("Enter:{:?}", v));
        } else if **k as u32 == 27 {
            ctx.print(PLAY_WIDTH + 1, 18 + idx, format!("Esc:{:?}", v));
        } else {
            ctx.print(PLAY_WIDTH + 1, 18 + idx, format!("{}:{:?}", k, v));
        }
    }

    let game_stats = world.get::<GameStats>().unwrap();
    ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 5, "Total Damage");
    ctx.print(
        PLAY_WIDTH + 1,
        SCREEN_HEIGHT - 4,
        format!("{:.2}", game_stats.damage_dealt),
    );
    ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 3, "Kills");
    ctx.print(
        PLAY_WIDTH + 1,
        SCREEN_HEIGHT - 2,
        format!("{}", game_stats.kill_count),
    );
}
