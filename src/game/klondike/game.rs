use std::mem;

use card::{Rank, Card, Face, Set, Pile};
use game::Game;
use super::{Klondike, Draw, Play, Foundation, Tableau};

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

            Play::Reveal(tableau) => {
                self.tableau[tableau as usize].top().as_ref().map_or(false, Face::is_down)
            },

            Play::WasteTableau(tableau) => {
                self.waste.top().map_or(false, |face| {
                    self.is_valid_tableau(tableau, face.card())
                })
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

            Play::FoundationTableau(foundation, tableau) => {
                self.foundations[foundation as usize].top().map_or(false, |face| {
                    self.is_valid_tableau(tableau, face.card())
                })
            },

            Play::TableauTableau(src, count, dest) => {
                self.tableau[src as usize].get_back(count as usize).map_or(false, |face| {
                    face.is_up() && self.is_valid_tableau(dest, face.card())
                })
            },
        }
    }

    fn play(&mut self, play: &Self::Play) {
        match *play {
            Play::Draw => match self.draw {
                Draw::One => self.stock.deal_to(&mut self.waste, 1, true),
                Draw::Three => self.stock.deal_to(&mut self.waste, 3, true),
            },

            Play::Redeal => {
                mem::swap(&mut self.waste, &mut self.stock);
                self.stock.flip();
            },

            Play::Reveal(tableau) => {
                self.tableau[tableau as usize].flip_top();
            },

            Play::WasteTableau(tableau) => {
                self.waste.move_to(&mut self.tableau[tableau as usize], 1);
            },

            Play::WasteFoundation(foundation) => {
                self.waste.move_to(&mut self.foundations[foundation as usize], 1);
            },

            Play::TableauFoundation(tableau, foundation) => {
                self.tableau[tableau as usize].move_to(
                    &mut self.foundations[foundation as usize],
                    1,
                );
            },

            Play::FoundationTableau(foundation, tableau) => {
                self.foundations[foundation as usize].move_to(
                    &mut self.tableau[tableau as usize],
                    1,
                );
            },

            Play::TableauTableau(src, count, dest) => {
                if src < dest {
                    let (left, right) = self.tableau.split_at_mut(dest as usize);
                    left[src as usize].move_to(&mut right[0], count as usize);
                } else {
                    let (left, right) = self.tableau.split_at_mut(src as usize);
                    right[0].move_to(&mut left[dest as usize], count as usize);
                }
            },
        }
    }

    fn undo(&mut self, play: &Play) {
        match *play {
            Play::Draw => match self.draw {
                Draw::One => self.waste.deal_to(&mut self.stock, 1, true),
                Draw::Three => self.waste.deal_to(&mut self.stock, 3, true),
            },

            Play::Redeal => {
                mem::swap(&mut self.stock, &mut self.waste);
                self.waste.flip();
            },

            Play::Reveal(_) => self.play(&play),

            Play::WasteTableau(tableau) => {
                self.tableau[tableau as usize].move_to(&mut self.waste, 1);
            },

            Play::WasteFoundation(foundation) => {
                self.foundations[foundation as usize].move_to(&mut self.waste, 1);
            },

            Play::TableauFoundation(tableau, foundation) => {
                self.play(&Play::FoundationTableau(foundation, tableau));
            },

            Play::FoundationTableau(foundation, tableau) => {
                self.play(&Play::TableauFoundation(tableau, foundation));
            },

            Play::TableauTableau(src, count, dest) => {
                self.play(&Play::TableauTableau(dest, count, src));
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

    fn is_valid_tableau(&self, tableau: Tableau, card: Card) -> bool {
        let tableau = &self.tableau[tableau as usize];
        if tableau.is_empty() {
            card.rank == Rank::King
        } else {
            let top = tableau.top().unwrap().card();
            card.suit.color() != top.suit.color() && card.rank.succ() == Some(top.rank)
        }
    }
}
