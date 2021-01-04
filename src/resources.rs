use crate::*;

/// The winner of this game.
#[allow(missing_docs)]
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

/// Whether we requested to quit the game.
#[derive(PartialEq, Eq, Clone, Default)]
pub struct QuitGame(pub bool);

/// The selected leaders in each team.
#[derive(Clone, Default, new)]
pub struct TeamLeaders {
    /// The leaders in my team.
    pub me: Vec<Leaders>,
    /// The leaders in the opponent's team.
    pub other: Vec<Leaders>,
}

/// The selected leader in the user interface.
#[derive(Clone, Default, new)]
pub struct SelectedLeader(pub u8);

/// The statistics of this game.
#[derive(Clone, Default)]
pub struct GameStats {
    /// The number of killed entities.
    pub kill_count: u32,
    /// The total damage dealt to all entities during this game.
    pub damage_dealt: f64,
}
