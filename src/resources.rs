use crate::*;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Winner {
    None,
    Me,
    Other,
    Draw,
}

impl Default for Winner {
    fn default() -> Self {
        Winner::None
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct QuitGame(pub bool);

#[derive(Clone, Default, new)]
pub struct TeamHeroes {
    pub me: Vec<Heroes>,
    pub other: Vec<Heroes>,
}

#[derive(Clone, Default, new)]
pub struct SelectedHero(pub u8);

#[derive(Clone, Default)]
pub struct GameStats {
    pub kill_count: u32,
    pub damage_dealt: f64,
}
