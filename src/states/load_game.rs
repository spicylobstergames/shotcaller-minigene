use crate::*;

/// The default state of the game. Where the gameplay happens.
pub struct LoadGameState;

#[allow(unused_variables)]
impl minigene::State for LoadGameState {
    fn on_start(&mut self, world: &mut World, dispatcher: &mut Dispatcher, ctx: &mut BTerm) {
    }
    fn update(&mut self, world: &mut World, dispatcher: &mut Dispatcher, ctx: &mut BTerm) -> Trans {
        Trans::None
    }
}
