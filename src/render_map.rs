use crate::*;

pub fn render<'a>(ctx: &mut BTerm) {
    ctx.cls();
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
