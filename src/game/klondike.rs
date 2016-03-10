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

#[cfg(test)]
mod tests {
    use card::{Suit, Rank, Card, Face, Pile};
    use game::Game;
    use super::{Klondike, Draw, Play, Foundation, Tableau};

    #[test]
    fn valid_draw_full_stock() {
        let game = Klondike::new(Draw::One);
        assert!(game.is_valid(&Play::Draw));
    }

    #[test]
    fn invalid_draw_empty_stock() {
        let mut game = Klondike::new(Draw::One);
        game.stock = Pile::new();
        assert!(!game.is_valid(&Play::Draw));
    }

    #[test]
    fn valid_redeal_empty_stock() {
        let mut game = Klondike::new(Draw::One);
        game.stock.deal_to(&mut game.waste, 52, true);
        assert!(game.is_valid(&Play::Redeal));
    }

    #[test]
    fn invalid_redeal_empty_waste() {
        let game = Klondike::new(Draw::One);
        assert!(!game.is_valid(&Play::Redeal));
    }

    #[test]
    fn invalid_redeal_non_empty_stock() {
        let mut game = Klondike::new(Draw::One);
        game.stock.deal_to(&mut game.waste, 1, true);
        assert!(!game.is_valid(&Play::Redeal));
    }

    #[test]
    fn invalid_redeal_empty_stock_empty_waste() {
        let mut game = Klondike::new(Draw::One);
        game.stock = Pile::new();
        assert!(!game.is_valid(&Play::Redeal));
    }

    #[test]
    fn valid_waste_tableau_king() {
        let mut game = Klondike::new(Draw::One);
        game.waste.push(Face::Up(Card::new(Suit::Club, Rank::King)));
        assert!(game.is_valid(&Play::WasteTableau(Tableau::One)));
    }

    #[test]
    fn valid_waste_tableau_pred_color() {
        let mut game = Klondike::new(Draw::One);
        game.waste.push(Face::Up(Card::new(Suit::Heart, Rank::Queen)));
        game.tableau[0].push(Face::Up(Card::new(Suit::Club, Rank::King)));
        assert!(game.is_valid(&Play::WasteTableau(Tableau::One)));
    }

    #[test]
    fn invalid_waste_tableau_empty_waste() {
        let game = Klondike::new(Draw::One);
        assert!(!game.is_valid(&Play::WasteTableau(Tableau::One)));
    }

    #[test]
    fn invalid_waste_tableau_non_king() {
        let mut game = Klondike::new(Draw::One);
        game.waste.push(Face::Up(Card::new(Suit::Heart, Rank::Queen)));
        assert!(!game.is_valid(&Play::WasteTableau(Tableau::One)));
    }

    #[test]
    fn invalid_waste_tableau_non_color() {
        let mut game = Klondike::new(Draw::One);
        game.waste.push(Face::Up(Card::new(Suit::Heart, Rank::Queen)));
        game.tableau[0].push(Face::Up(Card::new(Suit::Diamond, Rank::King)));
        assert!(!game.is_valid(&Play::WasteTableau(Tableau::One)));
    }

    #[test]
    fn invalid_waste_tableau_non_pred() {
        let mut game = Klondike::new(Draw::One);
        game.waste.push(Face::Up(Card::new(Suit::Heart, Rank::Jack)));
        game.tableau[0].push(Face::Up(Card::new(Suit::Club, Rank::King)));
        assert!(!game.is_valid(&Play::WasteTableau(Tableau::One)));
    }
}
