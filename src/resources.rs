use crate::*;
use nanoserde::*;

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
#[derive(Clone, Default, new, SerBin, DeBin, Debug)]
pub struct TeamLeaders {
    /// The leaders in my team.
    pub me: Vec<Leaders>,
    /// The leaders in the opponent's team.
    pub other: Vec<Leaders>,
}

/// The selected leader in the user interface.
#[derive(Clone, Default, new)]
pub struct SelectedLeader(pub u8);

/// The selected item in the user interface.
#[derive(Clone, Default, Debug, new)]
pub struct SelectedItem(pub Option<ShelfItem>);

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
    // /// The right button was clicked
    // pub right_click: bool,
}

/// Current game mode. Used to disable/enable specific systems
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum GameMode {
    /// Default game mode
    Shotcaller,
    /// Game mode where player directly controls individual units (aka. traditional RTS controls).
    MicroInput,
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Shotcaller
    }
}

/// State of UI for micro-input
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum InputState {
    /// Default state
    Default,
    /// State when next input is to be considered M-Move target
    MMove,
    /// State when next input is to be considered A-Move target
    AMove,
}

impl Default for InputState {
    fn default() -> Self {
        InputState::Default
    }
}

/// Stores units that were selected by the player
#[derive(Clone, Debug, new)]
pub struct SelectedUnits {
    /// Vector of all selected Entities
    pub units: Vec<Entity>,
}

impl Default for SelectedUnits {
    fn default() -> Self {
        SelectedUnits { units: vec![] }
    }
}
