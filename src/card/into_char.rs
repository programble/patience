use std::char;

use super::{Suit, Rank, Card, Face};

impl Into<char> for Suit {
    fn into(self) -> char {
        match self {
            Suit::Club => '♣',
            Suit::Diamond => '♦',
            Suit::Heart => '♥',
            Suit::Spade => '♠',
        }
    }
}

impl Into<char> for Rank {
    fn into(self) -> char {
        match self {
            Rank::Ace => 'A',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
        }
    }
}

impl Into<char> for Card {
    fn into(self) -> char {
        // "Knight" appears between "Jack" and "Queen" in Unicode.
        let base = match self.rank {
            Rank::Queen => '🂭',
            Rank::King => '🂮',
            r => char::from_u32('🂠' as u32 + r as u32).unwrap(),
        };

        let offset = match self.suit {
            Suit::Spade => 0,
            Suit::Heart => 16,
            Suit::Diamond => 32,
            Suit::Club => 48,
        };

        char::from_u32(base as u32 + offset).unwrap()
    }
}

impl Into<[char; 2]> for Card {
    fn into(self) -> [char; 2] {
        [self.suit.into(), self.rank.into()]
    }
}

impl Into<char> for Face {
    fn into(self) -> char {
        match self {
            Face::Down(_) => '🂠',
            Face::Up(card) => card.into(),
        }
    }
}

impl Into<[char; 2]> for Face {
    fn into(self) -> [char; 2] {
        match self {
            Face::Down(_) => ['▮', '▮'],
            Face::Up(card) => card.into(),
        }
    }
}
