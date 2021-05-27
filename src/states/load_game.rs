use crate::*;

/// The default state of the game. Where the gameplay happens.
pub struct LoadGameState;

#[allow(unused_variables)]
impl minigene::State<GameData> for LoadGameState {
    fn on_start(&mut self, data: &mut GameData) {}
    fn update(&mut self, data: &mut GameData) -> StateTransition<GameData> {
        StateTransition::None
    }
}
