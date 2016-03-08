use std::iter::FromIterator;
use std::slice::Iter;

use rand::{self, Rng};

use super::Face;

/// Pile of face-down or face-up cards.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Returns true if pile is empty.
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// Returns the top card.
    pub fn last(&self) -> Option<Face> {
        self.vec.last().cloned()
    }

    /// Pushes a card onto the pile.
    pub fn push(&mut self, face: Face) {
        self.vec.push(face);
    }

    /// Pops a card from the pile.
    pub fn pop(&mut self) -> Option<Face> {
        self.vec.pop()
    }

    /// Shuffles the pile.
    pub fn shuffle(&mut self) {
        rand::thread_rng().shuffle(&mut self.vec);
    }

    /// Flips each card in the pile.
    pub fn flip(&mut self) {
        for face in &mut self.vec {
            face.flip();
        }
    }

    /// Flips the top card.
    pub fn flip_last(&mut self) {
        let _ = self.vec.last_mut().map(Face::flip);
    }

    /// Moves at most `count` cards from the top of this pile to the top of another pile.
    pub fn move_to(&mut self, dest: &mut Pile, count: usize, flip: bool) {
        let index = self.vec.len().saturating_sub(count);
        for face in self.vec.drain(index..) {
            dest.push(if flip { face.flipped() } else { face });
        }
    }

    /// Deals at most`count` cards from the top of this pile to the top of another pile, in reverse
    /// order.
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
