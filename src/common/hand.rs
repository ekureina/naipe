//! An implementation of a hand to store cards

use std::fmt::{self, Display, Formatter};

use super::card::Card;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    /// Create a new, empty hand
    pub fn new() -> Hand {
        Hand::default()
    }

    /// Pops off the top card from the hand
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Finds if the hand is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Finds how many cards are currently in this hand
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

impl FromIterator<Card> for Hand {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Hand {
        let cards = Vec::from_iter(iter);
        Hand { cards }
    }
}

impl IntoIterator for Hand {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl<'a> IntoIterator for &'a Hand {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        // Need to split out into line to get the slice properly
        let cards = &self.cards;
        cards.iter()
    }
}

impl Extend<Card> for Hand {
    fn extend<T: IntoIterator<Item = Card>>(&mut self, iter: T) {
        self.cards.extend(iter);
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = self
            .into_iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "[{string}]")
    }
}
