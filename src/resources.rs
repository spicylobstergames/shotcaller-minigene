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
    /// The total gold earned
    pub earned_gold: f64,
}

/// The mouse data
#[derive(Clone, Default, new)]
pub struct Mouse {
    /// The mouse position x, y
    pub pos: (i32, i32),
    /// The left button was clicked
    pub left_click: bool,
}

/// Current gamemode. Used to disable/enable specific systems
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum GameMode {
    /// Default gamemode
    Shotcaller,
    /// Gamemode where player directly controls individual units (aka. traditional RTS controls).
    MircoInput
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Shotcaller
    }
}