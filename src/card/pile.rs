use std::fmt::{Debug, Formatter, Error as FmtError};
use std::iter::FromIterator;
use std::slice::Iter;

use rand::{self, Rng};

use super::Face;

/// Pile of face-down or face-up cards.
#[derive(Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct Pile {
    pub vec: Vec<Face>,
}

impl Pile {
    /// Creates an empty pile.
    pub fn new() -> Self {
        Pile { vec: Vec::new() }
    }

    /// Returns the number of cards in the pile.
    pub fn count(&self) -> usize {
        self.vec.len()
    }

    /// Returns true if the pile is empty.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Gets the card at the index from the bottom.
    pub fn get(&self, index: usize) -> Option<Face> {
        self.vec.get(index).cloned()
    }

    /// Gets the card at the index from the top.
    pub fn get_back(&self, index: usize) -> Option<Face> {
        self.vec.get(self.vec.len() - index).cloned()
    }

    /// Returns the top card.
    pub fn top(&self) -> Option<Face> {
        self.vec.last().cloned()
    }

    /// Adds a card to the top of the pile.
    pub fn push(&mut self, face: Face) {
        self.vec.push(face);
    }

    /// Removes a card from the top of the pile.
    pub fn pop(&mut self) -> Option<Face> {
        self.vec.pop()
    }

    /// Shuffles the pile.
    pub fn shuffle(&mut self) {
        rand::thread_rng().shuffle(&mut self.vec);
    }

    /// Flips the whole pile over, reversing the order and flipping each card.
    pub fn flip(&mut self) {
        self.vec.reverse();
        for face in &mut self.vec {
            face.flip();
        }
    }

    /// Flips the top card.
    pub fn flip_top(&mut self) {
        let _ = self.vec.last_mut().map(Face::flip);
    }

    /// Moves at most `count` cards from the top of this pile to the top of another pile.
    pub fn move_to(&mut self, dest: &mut Pile, count: usize) {
        let index = self.vec.len().saturating_sub(count);
        for face in self.vec.drain(index..) {
            dest.push(face);
        }
    }

    /// Deals at most `count` cards from the top of this pile onto the top of another pile.
    pub fn deal_to(&mut self, dest: &mut Pile, count: usize, flip: bool) {
        let index = self.vec.len().saturating_sub(count);
        for face in self.vec.drain(index..).rev() {
            dest.push(if flip { face.flipped() } else { face });
        }
    }
}

impl FromIterator<Face> for Pile {
    fn from_iter<T>(iterator: T) -> Self where T: IntoIterator<Item=Face> {
        Pile { vec: Vec::from_iter(iterator) }
    }
}

impl<'a> IntoIterator for &'a Pile {
    type Item = &'a Face;
    type IntoIter = Iter<'a, Face>;

    fn into_iter(self) -> Iter<'a, Face> {
        self.vec[..].into_iter()
    }
}

impl Debug for Pile {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.debug_list()
            .entries(&self.vec)
            .finish()
    }
}
