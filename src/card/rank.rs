/// Card rank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    /// Returns true if `self` is a face card, i.e. `Jack`, `Queen`, `King`.
    pub fn is_face(&self) -> bool {
        match *self {
            Rank::Jack | Rank::Queen | Rank::King => true,
            _ => false,
        }
    }

    /// Returns the successive rank.
    pub fn succ(self) -> Option<Self> {
        match self {
            Rank::Ace => Some(Rank::Two),
            Rank::Two => Some(Rank::Three),
            Rank::Three => Some(Rank::Four),
            Rank::Four => Some(Rank::Five),
            Rank::Five => Some(Rank::Six),
            Rank::Six => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Nine),
            Rank::Nine => Some(Rank::Ten),
            Rank::Ten => Some(Rank::Jack),
            Rank::Jack => Some(Rank::Queen),
            Rank::Queen => Some(Rank::King),
            Rank::King => None,
        }
    }

    /// Returns the predecessive rank.
    pub fn pred(self) -> Option<Self> {
        match self {
            Rank::King => Some(Rank::Queen),
            Rank::Queen => Some(Rank::Jack),
            Rank::Jack => Some(Rank::Ten),
            Rank::Ten => Some(Rank::Nine),
            Rank::Nine => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Six),
            Rank::Six => Some(Rank::Five),
            Rank::Five => Some(Rank::Four),
            Rank::Four => Some(Rank::Three),
            Rank::Three => Some(Rank::Two),
            Rank::Two => Some(Rank::Ace),
            Rank::Ace => None,
        }
    }
}
