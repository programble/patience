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

#[test]
fn valid_waste_foundation_ace() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(Card::new(Suit::Spade, Rank::Ace)));
    assert!(game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn valid_waste_foundation_suit_succ() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(Card::new(Suit::Spade, Rank::Two)));
    game.foundations[0].push(Face::Up(Card::new(Suit::Spade, Rank::Ace)));
    assert!(game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn invalid_waste_foundation_empty_waste() {
    let game = Klondike::new(Draw::One);
    assert!(!game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn invalid_waste_foundation_non_ace() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(Card::new(Suit::Spade, Rank::Two)));
    assert!(!game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn invalid_waste_foundation_non_succ() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(Card::new(Suit::Spade, Rank::Three)));
    game.foundations[0].push(Face::Up(Card::new(Suit::Spade, Rank::Ace)));
    assert!(!game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn invalid_waste_foundation_non_suit() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(Card::new(Suit::Club, Rank::Two)));
    game.foundations[0].push(Face::Up(Card::new(Suit::Spade, Rank::Ace)));
    assert!(!game.is_valid(&Play::WasteFoundation(Foundation::One)));
}
