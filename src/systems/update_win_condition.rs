use crate::*;

pub fn 
    win_condition_system(core: &Components<Core>, team: &Components<Team>, winner: &mut Winner) -> SystemResult {
        let mut me = false;
        let mut you = false;
        for (_, t) in join!(&core && &team) {
            let t = t.unwrap();
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
        Ok(())
    }
