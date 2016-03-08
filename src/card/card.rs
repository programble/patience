use std::char;

use super::{Suit, Rank};

/// Card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Into<char> for Card {
    fn into(self) -> char {
        // "Knight" appears between "Jack" and "Queen" in Unicode.
        let base = match self.rank {
            Rank::Queen => '🂭',
            Rank::King => '🂭',
            r => char::from_u32('🂠' as u32 + r as u32).unwrap(),
        };

        let offset = match self.suit {
            Suit::Spade => 0,
            Suit::Heart => 16,
            Suit::Diamond => 32,
            Suit::Club => 48,
        };

        char::from_u32(base as u32 + offset).unwrap()
    }
}
