//! Klondike solitaire.

use card::{Set, Face, Pile};
use super::Game;

/// One-card or three-card draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Draw {
    One,
    Three,
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
            pile.flip_last();
        }
    }

    fn is_won(&self) -> bool {
        self.stock.is_empty()
            && self.waste.is_empty()
            && self.tableau.iter().all(Pile::is_empty)
    }

    fn is_lost(&self) -> bool {
        unimplemented!()
    }
}
