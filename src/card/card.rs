use std::cmp::Ordering;

use super::{Suit, Rank};

/// Card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.suit == other.suit {
            Some(self.rank.cmp(&other.rank))
        } else {
            None
        }
    }
}

impl Into<String> for Card {
    fn into(self) -> String {
        let mut s = String::new();
        s.push(self.suit.into());
        s.push(self.rank.into());
        s
    }
}
