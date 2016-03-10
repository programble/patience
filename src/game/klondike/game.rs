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
                self.foundations[src as usize].get_back(count).map_or(false, |face| {
                    face.is_up() && self.is_valid_tableau(dest, face.card())
                })
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
