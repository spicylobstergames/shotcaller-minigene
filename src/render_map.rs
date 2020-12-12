use crate::*;

pub fn render<'a>(ctx: &mut BTerm) {
    let mut i = 0;
    for s in MAP {
        ctx.print(0, i, s);
        i = i + 1;
    }
}

pub fn create_map_bg<'a>(world: &mut World) {
    let mut i = 0;
    for s in MAP {
        let mut j = 0;
        for c in s.chars() {
            if c == '#' {
                world.create_entity()
                    .with(SpriteIndex(55))
                    .with(Point::new(j, i))
                    .build();
            }
            j = j + 1;
        }
        i = i + 1;
    }
}

pub fn render_ui(world: &mut World, ctx: &mut BTerm) {
    ctx.draw_box(PLAY_WIDTH, 0, SCREEN_WIDTH-PLAY_WIDTH-1, SCREEN_HEIGHT-1, WHITE, BLACK);
    ctx.print(PLAY_WIDTH+1, 1, "Leaders");
    ctx.print(PLAY_WIDTH+1, 3, "My Team");

    // TODO tmp
    let selected = 3;

    for i in 0..5 {
        ctx.print(PLAY_WIDTH+1, i+4, format!(" Leader {}", i));
    }
    ctx.print(PLAY_WIDTH+1, 10, "Enemy Team");
    for i in 0..5 {
        ctx.print(PLAY_WIDTH+1, i+11, format!(" Leader {}", i));
    }

    ctx.print(PLAY_WIDTH+1, selected+4, ">");

    ctx.print(PLAY_WIDTH+1, 17, "Keybinds");

    let hm = world.fetch::<HashMap<VirtualKeyCode, InputEvent>>();
    let mut keybinds = hm.iter().collect::<Vec<_>>();
    keybinds.sort_by(|t1, t2| format!("{:?}", t1.1).cmp(&format!("{:?}", t2.1)));
    for (idx, (k, v)) in keybinds.iter().enumerate(){
        ctx.print(PLAY_WIDTH+1, 18+idx, format!("{:?}:{:?}", k, v));
    }

    ctx.print(PLAY_WIDTH+1, SCREEN_HEIGHT-2, "12345678901234567890");
}
