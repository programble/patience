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
            Some(Card { suit: Suit::Spade, rank: Rank::King }) => None,
            Some(card) => Some(match card {
                Card { suit, rank: Rank::Ace } => Card { suit: suit, rank: Rank::Two },
                Card { suit, rank: Rank::Two } => Card { suit: suit, rank: Rank::Three },
                Card { suit, rank: Rank::Three } => Card { suit: suit, rank: Rank::Four },
                Card { suit, rank: Rank::Four } => Card { suit: suit, rank: Rank::Five },
                Card { suit, rank: Rank::Five } => Card { suit: suit, rank: Rank::Six },
                Card { suit, rank: Rank::Six } => Card { suit: suit, rank: Rank::Seven },
                Card { suit, rank: Rank::Seven } => Card { suit: suit, rank: Rank::Eight },
                Card { suit, rank: Rank::Eight } => Card { suit: suit, rank: Rank::Nine },
                Card { suit, rank: Rank::Nine } => Card { suit: suit, rank: Rank::Ten },
                Card { suit, rank: Rank::Ten } => Card { suit: suit, rank: Rank::Jack },
                Card { suit, rank: Rank::Jack } => Card { suit: suit, rank: Rank::Queen },
                Card { suit, rank: Rank::Queen } => Card { suit: suit, rank: Rank::King },
                Card { suit: Suit::Club, .. } => Card { suit: Suit::Diamond, rank: Rank::Ace },
                Card { suit: Suit::Diamond, .. } => Card { suit: Suit::Heart, rank: Rank::Ace },
                Card { suit: Suit::Heart, .. } => Card { suit: Suit::Spade, rank: Rank::Ace },
                _ => unreachable!(),
            }),
        };
        self.card
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (52, Some(52))
    }

}

impl ExactSizeIterator for Set { }
