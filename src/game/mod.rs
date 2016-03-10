//! Games.

/// Patience card game.
pub trait Game {
    /// Game variation rules.
    type Rules;

    /// Game play.
    type Play;

    /// Creates a game.
    fn new(rules: Self::Rules) -> Self;

    /// Deals the game.
    fn deal(&mut self);

    /// Returns true if the game is won.
    fn is_won(&self) -> bool;
}

pub mod klondike;
