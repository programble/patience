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

    /// Determines if the play is currently valid.
    fn is_valid(&self, play: &Self::Play) -> bool;

    /// Performs a play.
    fn play(&mut self, play: &Self::Play);

    /// Undoes a play.
    fn undo(&mut self, play: &Self::Play);
}

pub mod klondike;
