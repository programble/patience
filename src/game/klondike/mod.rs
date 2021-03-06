//! Klondike solitaire.

use card::Pile;

/// Klondike solitaire game.
#[derive(Debug, PartialEq, Eq)]
pub struct Klondike {
    draw: Draw,
    stock: Pile,
    waste: Pile,
    foundations: [Pile; 4],
    tableau: [Pile; 7],
}

// Clone can't be derived for non-Copy array fields?
impl Clone for Klondike {
    fn clone(&self) -> Self {
        Klondike {
            draw: self.draw,
            stock: self.stock.clone(),
            waste: self.waste.clone(),
            foundations: [
                self.foundations[0].clone(),
                self.foundations[1].clone(),
                self.foundations[2].clone(),
                self.foundations[3].clone(),
            ],
            tableau: [
                self.tableau[0].clone(),
                self.tableau[1].clone(),
                self.tableau[2].clone(),
                self.tableau[3].clone(),
                self.tableau[4].clone(),
                self.tableau[5].clone(),
                self.tableau[6].clone(),
            ],
        }
    }
}

/// One-card or three-card draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Draw {
    One,
    Three,
}

/// Klondike play.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Play {
    /// Draw from stock into waste.
    Draw,

    /// Move waste back to stock.
    Redeal,

    /// Reveal the top card of a tableau pile.
    Reveal(Tableau),

    /// Move top card of waste to tableau.
    WasteTableau(Tableau),

    /// Move top card of waste to foundation.
    WasteFoundation(Foundation),

    /// Move top card of tableau to foundation.
    TableauFoundation(Tableau, Foundation),

    /// Move top card of foundation to tableau.
    FoundationTableau(Foundation, Tableau),

    /// Move cards from tableau to tableau.
    TableauTableau(Tableau, u8, Tableau),
}

/// Foundations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Foundation {
    One,
    Two,
    Three,
    Four,
}

/// Tableaux.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Tableau {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

mod game;

#[cfg(test)]
mod tests;
