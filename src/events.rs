use crate::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum InputEvent {
    Quit,
    MenuNorth,
    MenuWest,
    MenuEast,
    MenuSouth,
    MenuSelect,
    MenuCancel,
    SpeedToggle,
    ZoomToggle,
    Teleport(u8),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum GameEvent {
    GameWon(Team),
}
