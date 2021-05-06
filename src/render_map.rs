use crate::*;

/// Renders the background map on the terminal.
pub fn render<'a>(ctx: &mut BTerm) {
    let mut i = 0;
    for s in MAP_STRING.iter() {
        for (j, c) in s.chars().enumerate() {
            if c == '#' {
                ctx.print(j, i, "#");
            } else {
                ctx.print(j, i, "0");
            }
        }
        i = i + 1;
    }
}

/// Creates the map background 2d sprite entities.
pub fn create_map_bg<'a>(world: &mut World) {
    let mut i = 0;
    for s in MAP_STRING.iter() {
        let mut j = 0;
        for c in s.chars() {
            if c == '#' {
                centity!(world, SpriteIndex(TileMapping::Forest.into()), Point::new(j, i),);
            }
            j = j + 1;
        }
        i = i + 1;
    }
}

/// Renders the user interface on the screen.
#[allow(unused_variables)]
pub fn render_ui(world: &mut World, ctx: &mut BTerm) {
    #[cfg(not(feature = "headless"))]
    {
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
        let mut offset = 0;
        for (idx, (k, v)) in keybinds.iter().enumerate() {
            match v {
                InputEvent::AutoSelect(..) => {
                    offset += 1;
                    continue;
                }
                _ => {}
            }
            if **k as u32 == 13 {
                ctx.print(PLAY_WIDTH + 1, 18 + idx - offset, format!("Enter:{:?}", v));
            } else if **k as u32 == 27 {
                ctx.print(PLAY_WIDTH + 1, 18 + idx - offset, format!("Esc:{:?}", v));
            } else {
                ctx.print(PLAY_WIDTH + 1, 18 + idx - offset, format!("{}:{:?}", k, v));
            }
        }

        let selected_units = world.get::<SelectedUnits>().unwrap();
        ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 9, "Selected Units");
        ctx.print(
            PLAY_WIDTH + 1,
            SCREEN_HEIGHT - 8,
            format!("{:.2}", selected_units.units.len()),
        );

        let input_state = world.get::<InputState>().unwrap();
        let is_txt = match *input_state {
            InputState::Default => "Default",
            InputState::MMove => "Move",
            InputState::AMove => "Attack",
        };
        ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 11, "InputState: ");
        ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 10, format!("{:.2}", is_txt));

        let game_stats = world.get::<GameStats>().unwrap();
        ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 7, "Total Damage");
        ctx.print(
            PLAY_WIDTH + 1,
            SCREEN_HEIGHT - 6,
            format!("{:.2}", game_stats.damage_dealt),
        );
        ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 5, "Kills");
        ctx.print(
            PLAY_WIDTH + 1,
            SCREEN_HEIGHT - 4,
            format!("{}", game_stats.kill_count),
        );
        ctx.print(PLAY_WIDTH + 1, SCREEN_HEIGHT - 3, "Earned Gold");
        ctx.print(
            PLAY_WIDTH + 1,
            SCREEN_HEIGHT - 2,
            format!("{}", game_stats.earned_gold),
        );
    }
}
/// Renders a cursor at mouse position
pub fn render_cursor(world: &mut World, ctx: &mut BTerm) {
    let mouse = world.get::<Mouse>().unwrap();
    ctx.print(mouse.pos.0, mouse.pos.1, format!(">"));
}
