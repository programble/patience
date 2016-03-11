//! Patience.

#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

extern crate rand;

/// Suit shorthand.
#[macro_export]
macro_rules! suit {
    (H) => { $crate::card::Suit::Heart };
    (C) => { $crate::card::Suit::Club };
    (D) => { $crate::card::Suit::Diamond };
    (S) => { $crate::card::Suit::Spade };
}

/// Rank shorthand.
#[macro_export]
macro_rules! rank {
    (A) => { $crate::card::Rank::Ace };
    (2) => { $crate::card::Rank::Two };
    (3) => { $crate::card::Rank::Three };
    (4) => { $crate::card::Rank::Four };
    (5) => { $crate::card::Rank::Five };
    (6) => { $crate::card::Rank::Six };
    (7) => { $crate::card::Rank::Seven };
    (8) => { $crate::card::Rank::Eight };
    (9) => { $crate::card::Rank::Nine };
    (10) => { $crate::card::Rank::Ten };
    (J) => { $crate::card::Rank::Jack };
    (Q) => { $crate::card::Rank::Queen };
    (K) => { $crate::card::Rank::King };
}

/// Card shorthand.
#[macro_export]
macro_rules! card {
    ($suit:tt $rank:tt) => { $crate::card::Card { suit: suit!($suit), rank: rank!($rank) } };
}

pub mod card;
pub mod game;
