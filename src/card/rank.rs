//! Card ranks.

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
}
