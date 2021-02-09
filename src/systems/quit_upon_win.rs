use crate::*;
use std::process::exit;

/// Quits the application and potentially records data when the game is won
#[allow(unused_variables)]
pub fn quit_upon_win_system(
    winner: &Winner,
) -> SystemResult {
    match winner {
        Winner::Me => exit(0),
        Winner::Other => exit(0),
        _ => (),
    }
    Ok(())
}