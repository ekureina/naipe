//! An implementation of a hand to store cards



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
