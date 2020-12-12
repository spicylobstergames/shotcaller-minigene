use crate::*;

pub struct SelectHeroRes {
    pub reader: ReaderId<InputEvent>,
}

system!(
    SelectHeroSystem,
    |events: Read<'a, EventChannel<InputEvent>>,
     res: WriteExpect<'a, HeroTeleportRes>,
     selected_hero: Write<'a, SelectedHero>| {
        for k in events.read(&mut res.reader) {
            match *k {
                InputEvent::MenuNorth => {
                    if selected_hero.0 > 0 {
                        selected_hero.0 -= 1;
                    }
                }
                InputEvent::MenuSouth => {
                    if selected_hero.0 < 4 {
                        selected_hero.0 += 1;
                    }
                }
                _ => {}
            }
        }
    }
);
