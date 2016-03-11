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
            Some(card!(C K)) => Some(card!(D K)),
            Some(card!(D A)) => Some(card!(S K)),
            Some(card!(S A)) => None,

            Some(Card { suit: suit!(H), rank }) => Some(Card::new(suit!(H), rank.succ().unwrap())),
            Some(Card { suit: suit!(C), rank }) => Some(Card::new(suit!(C), rank.succ().unwrap())),
            Some(Card { suit: suit!(D), rank }) => Some(Card::new(suit!(D), rank.pred().unwrap())),
            Some(Card { suit: suit!(S), rank }) => Some(Card::new(suit!(S), rank.pred().unwrap())),
        };
        self.card
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (52, Some(52))
    }

}

impl ExactSizeIterator for Set { }
