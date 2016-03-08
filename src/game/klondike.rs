use card::{Set, Face, Pile};

/// Klondike solitaire game.
#[derive(Debug, PartialEq, Eq)]
pub struct Klondike {
    stock: Pile,
    waste: Pile,
    foundations: [Pile; 4],
    tableau: [Pile; 7],
}

impl Klondike {
    /// Creates a game of Klondike.
    pub fn new() -> Self {
        let mut game = Klondike {
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
        };
        game.setup();
        game
    }

    fn setup(&mut self) {
        self.stock.shuffle();
        for (i, pile) in self.tableau.iter_mut().enumerate() {
            self.stock.deal_to(pile, i + 1, false);
            pile.flip_last();
        }
    }
}
