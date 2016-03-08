//! Playing cards.

use std::char;

pub use self::suit::{Color, Suit};
pub use self::rank::Rank;

mod suit;
mod rank;

/// Card orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Face {
    Down,
    Up,
}

/// Card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Card {
    pub face: Face,
    pub suit: Suit,
    pub rank: Rank,
}

impl Into<char> for Card {
    fn into(self) -> char {
        match self {
            Card { face: Face::Down, .. } => '🂠',
            Card { suit, rank, .. } => {
                // "Knight" appears between "Jack" and "Queen" in Unicode.
                let base = match rank {
                    Rank::Queen => '🂭',
                    Rank::King => '🂭',
                    r => char::from_u32('🂠' as u32 + r as u32).unwrap(),
                };

                let offset = match suit {
                    Suit::Spade => 0,
                    Suit::Heart => 16,
                    Suit::Diamond => 32,
                    Suit::Club => 48,
                };

                char::from_u32(base as u32 + offset).unwrap()
            },
        }
    }
}
