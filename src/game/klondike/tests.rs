use card::{Face, Pile};
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
    game.waste.push(Face::Up(card!(C K)));
    assert!(game.is_valid(&Play::WasteTableau(Tableau::One)));
}

#[test]
fn valid_waste_tableau_pred_color() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(card!(H Q)));
    game.tableau[0].push(Face::Up(card!(C K)));
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
    game.waste.push(Face::Up(card!(H Q)));
    assert!(!game.is_valid(&Play::WasteTableau(Tableau::One)));
}

#[test]
fn invalid_waste_tableau_non_color() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(card!(H Q)));
    game.tableau[0].push(Face::Up(card!(D K)));
    assert!(!game.is_valid(&Play::WasteTableau(Tableau::One)));
}

#[test]
fn invalid_waste_tableau_non_pred() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(card!(H J)));
    game.tableau[0].push(Face::Up(card!(C K)));
    assert!(!game.is_valid(&Play::WasteTableau(Tableau::One)));
}

#[test]
fn valid_waste_foundation_ace() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(card!(S A)));
    assert!(game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn valid_waste_foundation_suit_succ() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(card!(S 2)));
    game.foundations[0].push(Face::Up(card!(S A)));
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
    game.waste.push(Face::Up(card!(S 2)));
    assert!(!game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn invalid_waste_foundation_non_succ() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(card!(S 3)));
    game.foundations[0].push(Face::Up(card!(S A)));
    assert!(!game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn invalid_waste_foundation_non_suit() {
    let mut game = Klondike::new(Draw::One);
    game.waste.push(Face::Up(card!(C 2)));
    game.foundations[0].push(Face::Up(card!(S A)));
    assert!(!game.is_valid(&Play::WasteFoundation(Foundation::One)));
}

#[test]
fn valid_tableau_foundation_ace() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S A)));
    assert!(game.is_valid(&Play::TableauFoundation(Tableau::One, Foundation::One)));
}

#[test]
fn valid_tableau_foundation_suit_succ() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S 2)));
    game.foundations[0].push(Face::Up(card!(S A)));
    assert!(game.is_valid(&Play::TableauFoundation(Tableau::One, Foundation::One)));
}

#[test]
fn invalid_tableau_foundation_empty_tableau() {
    let game = Klondike::new(Draw::One);
    assert!(!game.is_valid(&Play::TableauFoundation(Tableau::One, Foundation::One)));
}

#[test]
fn invalid_tableau_foundation_non_ace() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S 2)));
    assert!(!game.is_valid(&Play::TableauFoundation(Tableau::One, Foundation::One)));
}

#[test]
fn invalid_tableau_foundation_non_succ() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S 3)));
    game.foundations[0].push(Face::Up(card!(S A)));
    assert!(!game.is_valid(&Play::TableauFoundation(Tableau::One, Foundation::One)));
}

#[test]
fn invalid_tableau_foundation_non_suit() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(C 2)));
    game.foundations[0].push(Face::Up(card!(S A)));
    assert!(!game.is_valid(&Play::TableauFoundation(Tableau::One, Foundation::One)));
}

#[test]
fn valid_foundation_tableau_king() {
    let mut game = Klondike::new(Draw::One);
    game.foundations[0].push(Face::Up(card!(C K)));
    assert!(game.is_valid(&Play::FoundationTableau(Foundation::One, Tableau::One)));
}

#[test]
fn valid_foundation_tableau_pred_color() {
    let mut game = Klondike::new(Draw::One);
    game.foundations[0].push(Face::Up(card!(H Q)));
    game.tableau[0].push(Face::Up(card!(C K)));
    assert!(game.is_valid(&Play::FoundationTableau(Foundation::One, Tableau::One)));
}

#[test]
fn invalid_foundation_tableau_empty_foundation() {
    let game = Klondike::new(Draw::One);
    assert!(!game.is_valid(&Play::FoundationTableau(Foundation::One, Tableau::One)));
}

#[test]
fn invalid_foundation_tableau_non_king() {
    let mut game = Klondike::new(Draw::One);
    game.foundations[0].push(Face::Up(card!(H Q)));
    assert!(!game.is_valid(&Play::FoundationTableau(Foundation::One, Tableau::One)));
}

#[test]
fn invalid_foundation_tableau_non_color() {
    let mut game = Klondike::new(Draw::One);
    game.foundations[0].push(Face::Up(card!(H Q)));
    game.tableau[0].push(Face::Up(card!(D K)));
    assert!(!game.is_valid(&Play::FoundationTableau(Foundation::One, Tableau::One)));
}

#[test]
fn invalid_foundation_tableau_non_pred() {
    let mut game = Klondike::new(Draw::One);
    game.foundations[0].push(Face::Up(card!(H J)));
    game.tableau[0].push(Face::Up(card!(C K)));
    assert!(!game.is_valid(&Play::FoundationTableau(Foundation::One, Tableau::One)));
}

#[test]
fn valid_tableau_tableau_one_king() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(C K)));
    assert!(game.is_valid(&Play::TableauTableau(Tableau::One, 1, Tableau::Two)));
}

#[test]
fn valid_tableau_tableau_two_king() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(C K)));
    game.tableau[0].push(Face::Up(card!(H Q)));
    assert!(game.is_valid(&Play::TableauTableau(Tableau::One, 2, Tableau::Two)));
}

#[test]
fn valid_tableau_tableau_one_pred_color() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(C A)));
    game.tableau[1].push(Face::Up(card!(H 2)));
    assert!(game.is_valid(&Play::TableauTableau(Tableau::One, 1, Tableau::Two)));
}

#[test]
fn valid_tableau_tableau_two_pred_color() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(C Q)));
    game.tableau[0].push(Face::Up(card!(H J)));
    game.tableau[1].push(Face::Up(card!(D K)));
    assert!(game.is_valid(&Play::TableauTableau(Tableau::One, 2, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_empty_src() {
    let game = Klondike::new(Draw::One);
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 1, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_face_down() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Down(card!(H K)));
    game.tableau[0].push(Face::Up(card!(S Q)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 2, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_count() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S K)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 2, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_zero() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S K)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 0, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_same() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(H K)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 1, Tableau::One)));
}

#[test]
fn invalid_tableau_tableau_one_non_king() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S Q)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 1, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_two_non_king() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S Q)));
    game.tableau[0].push(Face::Up(card!(H J)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 2, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_one_non_color() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S Q)));
    game.tableau[1].push(Face::Up(card!(C K)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 1, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_two_non_color() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S Q)));
    game.tableau[0].push(Face::Up(card!(H J)));
    game.tableau[1].push(Face::Up(card!(C K)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 2, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_one_non_pred() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S J)));
    game.tableau[1].push(Face::Up(card!(H K)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 1, Tableau::Two)));
}

#[test]
fn invalid_tableau_tableau_two_non_pred() {
    let mut game = Klondike::new(Draw::One);
    game.tableau[0].push(Face::Up(card!(S J)));
    game.tableau[0].push(Face::Up(card!(D 10)));
    game.tableau[1].push(Face::Up(card!(H K)));
    assert!(!game.is_valid(&Play::TableauTableau(Tableau::One, 2, Tableau::Two)));
}
