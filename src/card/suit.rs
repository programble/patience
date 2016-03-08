//! Card suits.

/// Card color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Color {
    Black,
    Red,
}

/// Card suit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl Suit {
    /// Returns the color of the suit.
    pub fn color(&self) -> Color {
        match *self {
            Suit::Club | Suit::Spade => Color::Black,
            Suit::Diamond | Suit::Heart => Color::Red,
        }
    }
}

impl Into<char> for Suit {
    fn into(self) -> char {
        match self {
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Heart => '♥',
            Suit::Spade => '♠',
        }
    }
}
