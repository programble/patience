use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Error as FmtError};

use super::{Suit, Rank};

/// Card.
#[derive(Clone, Copy, PartialEq, Eq)]
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

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}{}", Into::<char>::into(self.suit), Into::<char>::into(self.rank))
    }
}
