//! Klondike solitaire.

use card::{Set, Rank, Card, Face, Pile};
use super::Game;

/// One-card or three-card draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Draw {
    One,
    Three,
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

/// Klondike play.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Play {
    /// Draw from stock into waste.
    Draw,

    /// Move waste back to stock.
    Redeal,

    /// Move top card of waste to tableau.
    WasteTableau(Tableau),

    /// Move top card of waste to foundation.
    WasteFoundation(Foundation),

    /// Move top card of tableau to foundation.
    TableauFoundation(Tableau, Foundation),

    /// Move top card of foundation to tableau.
    FoundationTableau(Foundation, Tableau),

    /// Move cards from tableau to tableau.
    TableauTableau(Tableau, usize, Tableau),
}

/// Klondike solitaire game.
#[derive(Debug, PartialEq, Eq)]
pub struct Klondike {
    draw: Draw,
    stock: Pile,
    waste: Pile,
    foundations: [Pile; 4],
    tableau: [Pile; 7],
}

impl Game for Klondike {
    type Rules = Draw;
    type Play = Play;

    fn new(draw: Draw) -> Self {
        Klondike {
            draw: draw,
            stock: Set::new().map(Face::Down).collect(),
            waste: Pile::new(),
            foundations: [Pile::new(), Pile::new(), Pile::new(), Pile::new()],
            tableau: [
                Pile::new(),
                Pile::new(),
                Pile::new(),
                Pile::new(),
                Pile::new(),
                Pile::new(),
                Pile::new(),
            ],
        }
    }

    fn deal(&mut self) {
        self.stock.shuffle();
        for (i, pile) in self.tableau.iter_mut().enumerate() {
            self.stock.deal_to(pile, i + 1, false);
            pile.flip_top();
        }
    }

    fn is_won(&self) -> bool {
        self.stock.is_empty()
            && self.waste.is_empty()
            && self.tableau.iter().all(Pile::is_empty)
    }

    fn is_valid(&self, play: &Play) -> bool {
        match *play {
            Play::Draw => {
                !self.stock.is_empty()
            },

            Play::Redeal => {
                self.stock.is_empty() && !self.waste.is_empty()
            },

            Play::WasteTableau(_tableau) => {
                unimplemented!()
            },

            Play::WasteFoundation(foundation) => {
                self.waste.top().map_or(false, |face| {
                    self.is_valid_foundation(foundation, face.card())
                })
            },

            Play::TableauFoundation(tableau, foundation) => {
                self.tableau[tableau as usize].top().map_or(false, |face| {
                    self.is_valid_foundation(foundation, face.card())
                })
            },

            Play::FoundationTableau(_foundation, _tableau) => {
                unimplemented!()
            },

            Play::TableauTableau(_src, _count, _dest) => {
                unimplemented!()
            },
        }
    }
}

impl Klondike {
    fn is_valid_foundation(&self, foundation: Foundation, card: Card) -> bool {
        let foundation = &self.foundations[foundation as usize];
        if foundation.is_empty() {
            card.rank == Rank::Ace
        } else {
            let top = foundation.top().unwrap().card();
            card.suit == top.suit && card.rank.pred() == Some(top.rank)
        }
    }
}
