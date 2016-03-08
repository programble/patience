use std::cmp::Ordering;

use super::Card;

/// Face-down or face-up card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Face {
    Down(Card),
    Up(Card),
}

impl Face {
    /// Returns true if the card is face-down.
    pub fn is_down(&self) -> bool {
        match *self {
            Face::Down(_) => true,
            _ => false,
        }
    }

    /// Returns true if the card is face-up.
    pub fn is_up(&self) -> bool {
        match *self {
            Face::Up(_) => true,
            _ => false,
        }
    }

    /// Returns the card.
    pub fn card(&self) -> Card {
        match *self {
            Face::Down(card) => card,
            Face::Up(card) => card,
        }
    }

    /// Returns the card, flipped.
    pub fn flipped(self) -> Self {
        match self {
            Face::Down(card) => Face::Up(card),
            Face::Up(card) => Face::Down(card),
        }
    }

    /// Flips the card in place.
    pub fn flip(&mut self) {
        *self = self.flipped();
    }
}

impl PartialOrd for Face {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.card().partial_cmp(&other.card())
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
