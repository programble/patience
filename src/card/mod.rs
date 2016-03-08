//! Playing cards.

pub use self::suit::{Color, Suit};
pub use self::rank::Rank;
pub use self::card::Card;
pub use self::set::Set;

mod suit;
mod rank;
mod card;
mod set;
