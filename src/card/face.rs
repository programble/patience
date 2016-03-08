use super::Card;

/// Face-down or face-up card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Face {
    Down(Card),
    Up(Card),
}

impl Face {
    /// Flips the card.
    pub fn flip(&mut self) {
        *self = match *self {
            Face::Down(card) => Face::Up(card),
            Face::Up(card) => Face::Down(card),
        };
    }
}

impl Into<char> for Face {
    fn into(self) -> char {
        match self {
            Face::Down(_) => '🂠',
            Face::Up(card) => card.into(),
        }
    }
}
