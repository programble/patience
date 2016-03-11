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
    Heart,
    Club,
    Diamond,
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
