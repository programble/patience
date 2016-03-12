mod is_valid {
    use card::{Face, Pile};
    use game::Game;
    use game::klondike::{Klondike, Draw, Play, Foundation, Tableau};

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
    fn valid_reveal() {
        let mut game = Klondike::new(Draw::One);
        game.stock.deal_to(&mut game.tableau[0], 1, false);
        assert!(game.is_valid(&Play::Reveal(Tableau::One)));
    }

    #[test]
    fn invalid_reveal_up() {
        let mut game = Klondike::new(Draw::One);
        game.stock.deal_to(&mut game.tableau[0], 1, true);
        assert!(!game.is_valid(&Play::Reveal(Tableau::One)));
    }

    #[test]
    fn invalid_reveal_empty() {
        let game = Klondike::new(Draw::One);
        assert!(!game.is_valid(&Play::Reveal(Tableau::One)));
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
}

mod play {
    use card::Face;
    use game::Game;
    use game::klondike::{Klondike, Draw, Play, Tableau, Foundation};

    #[test]
    fn draw_one() {
        let mut game = Klondike::new(Draw::One);
        let top = game.stock.top();
        game.play(&Play::Draw);
        assert_eq!(top.map(Face::flipped), game.waste.top());
    }

    #[test]
    fn draw_three() {
        let mut game = Klondike::new(Draw::Three);
        let (a, b, c) = (game.stock.get_back(1), game.stock.get_back(2), game.stock.get_back(3));
        game.play(&Play::Draw);
        assert_eq!(a.map(Face::flipped), game.waste.get_back(3));
        assert_eq!(b.map(Face::flipped), game.waste.get_back(2));
        assert_eq!(c.map(Face::flipped), game.waste.get_back(1));
    }

    #[test]
    fn redeal() {
        let mut game = Klondike::new(Draw::Three);
        let stock = game.stock.clone();
        while !game.stock.is_empty() {
            game.play(&Play::Draw);
        }
        game.play(&Play::Redeal);
        assert_eq!(stock, game.stock);
    }

    #[test]
    fn reveal() {
        let mut game = Klondike::new(Draw::One);
        game.stock.deal_to(&mut game.tableau[0], 1, false);
        game.play(&Play::Reveal(Tableau::One));
        assert!(game.tableau[0].top().unwrap().is_up());
    }

    #[test]
    fn waste_tableau() {
        let mut game = Klondike::new(Draw::One);
        game.play(&Play::Draw);
        let card = game.waste.top();
        game.play(&Play::WasteTableau(Tableau::One));
        assert_eq!(card, game.tableau[0].top());
    }

    #[test]
    fn waste_foundation() {
        let mut game = Klondike::new(Draw::One);
        game.play(&Play::Draw);
        let card = game.waste.top();
        game.play(&Play::WasteFoundation(Foundation::One));
        assert_eq!(card, game.foundations[0].top());
    }

    #[test]
    fn tableau_foundation() {
        let mut game = Klondike::new(Draw::One);
        game.deal();
        let card = game.tableau[1].top();
        game.play(&Play::TableauFoundation(Tableau::Two, Foundation::One));
        assert_eq!(card, game.foundations[0].top());
    }

    #[test]
    fn foundation_tableau() {
        let mut game = Klondike::new(Draw::One);
        game.play(&Play::Draw);
        game.play(&Play::WasteFoundation(Foundation::One));
        let card = game.foundations[0].top();
        game.play(&Play::FoundationTableau(Foundation::One, Tableau::One));
        assert_eq!(card, game.tableau[0].top());
    }

    #[test]
    fn tableau_tableau() {
        let mut game = Klondike::new(Draw::One);
        game.deal();
        game.play(&Play::Draw);
        game.play(&Play::WasteTableau(Tableau::Two));
        let (a, b) = (game.tableau[1].get_back(2), game.tableau[1].get_back(1));
        game.play(&Play::TableauTableau(Tableau::Two, 2, Tableau::One));
        assert_eq!(a, game.tableau[0].get_back(2));
        assert_eq!(b, game.tableau[0].get_back(1));
    }
}
