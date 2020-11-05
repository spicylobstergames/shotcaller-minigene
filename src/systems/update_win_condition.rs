use crate::*;

system!(
    WinConditionSystem,
    |core: ReadStorage<'a, Core>, team: ReadStorage<'a, Team>, winner: Write<'a, Winner>| {
        let mut me = false;
        let mut you = false;
        for (_, t) in (&core, &team).join() {
            match *t {
                Team::Me => me = true,
                Team::Other => you = true,
            }
        }
        match (me, you) {
            (false, false) => *winner = Winner::None,
            (false, true) => *winner = Winner::Other,
            (true, false) => *winner = Winner::Me,
            (true, true) => *winner = Winner::Draw,
        }
    }
);
