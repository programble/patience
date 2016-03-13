//! User interfaces.

use card::{Face, Pile};

/// Widths proportional to cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Width {
    /// Width of a card edge.
    Edge,

    /// Width of a card border.
    Border,

    /// Width of a card pip.
    Pip,

    /// Half width of a card.
    HalfCard,

    /// Width of a card.
    Card,
}

/// Heights proportional to cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Height {
    /// Height of a card edge.
    Edge,

    /// Height of a card border.
    Border,

    /// Height of a card pip.
    Pip,

    /// Half height of a card.
    HalfCard,

    /// Height of a card.
    Card,
}

/// Canvas on which cards can be drawn.
pub trait Canvas {
    /// Moves left.
    fn left(&mut self, width: Width);

    /// Moves right.
    fn right(&mut self, width: Width);

    /// Moves down.
    fn down(&mut self, height: Height);

    /// Moves up.
    fn up(&mut self, height: Height);

    /// Pushes the current position.
    fn push(&mut self);

    /// Pops the current position.
    fn pop(&mut self);

    /// Draws a card face or an empty cell.
    fn draw(&mut self, face: Option<Face>);
}
