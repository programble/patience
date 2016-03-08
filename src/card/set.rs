use super::{Card, Suit, Rank};

/// Iterator of the standard 52-card set.
#[derive(Debug, Clone, Copy)]
pub struct Set {
    card: Option<Card>,
}

impl Set {
    /// Creates a set.
    pub fn new() -> Self {
        Set { card: None }
    }
}

impl Iterator for Set {
    type Item = Card;

    fn next(&mut self) -> Option<Card> {
        self.card = match self.card {
            None => Some(Card { suit: Suit::Club, rank: Rank::Ace }),

            Some(Card { suit: Suit::Club, rank: Rank::King }) => {
                Some(Card { suit: Suit::Diamond, rank: Rank::Ace })
            },
            Some(Card { suit: Suit::Diamond, rank: Rank::King }) => {
                Some(Card { suit: Suit::Heart, rank: Rank::Ace })
            },
            Some(Card { suit: Suit::Heart, rank: Rank::King }) => {
                Some(Card { suit: Suit::Spade, rank: Rank::Ace })
            },
            Some(Card { suit: Suit::Spade, rank: Rank::King }) => None,

            Some(Card { suit, rank }) => {
                Some(Card { suit: suit, rank: rank.succ().unwrap() })
            },
        };
        self.card
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (52, Some(52))
    }

}

impl ExactSizeIterator for Set { }
