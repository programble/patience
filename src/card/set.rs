use super::Card;

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
            None => Some(card!(H A)),

            Some(card!(H K)) => Some(card!(C A)),
            Some(card!(C K)) => Some(card!(D A)),
            Some(card!(D K)) => Some(card!(S A)),
            Some(card!(S K)) => None,

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
