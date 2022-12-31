//! A module for handling decks
//!
//! Decks can have any integer number of full sets of cards

use std::num::NonZeroU16;

use rand::prelude::SliceRandom;
use thiserror::Error;

use super::card::Card;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Creates a new deck with the specified number of card sets
    /// # Examples
    /// ```
    /// # use naipe::common::deck::Deck;
    /// use std::num::NonZeroU16;
    /// let deck = Deck::new(NonZeroU16::new(2).unwrap());
    /// ```
    /// The above deck has 2 full sets of each card
    pub fn new(sets: NonZeroU16) -> Deck {
        let cards = (0..sets.into()).flat_map(|_| Card::all_cards()).collect();
        Deck { cards }
    }

    /// Shuffles the deck with the provided Rng
    /// Useful for seeded Rng
    pub fn shuffle<Rng: rand::Rng + ?Sized>(&mut self, rng: &mut Rng) {
        self.cards.shuffle(rng);
    }

    /// Shuffles the deck with the default [`rand::thread_rng`]
    pub fn shuffle_with_default_rng(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    /// Adds a card to this deck
    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Gets the number of cards currently in the deck
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Empties the deck fully
    pub fn empty(&mut self) {
        self.cards = vec![];
    }

    /// Finds if the deck is currently empty of cards
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Deals a set amount of cards from the top of the deck for the specified number of hands
    /// # Errors
    /// [`DeckDealError::NotEnoughCards`] if the deck does not have enough cards to fulfill the deal request
    pub fn deal_cards(
        &mut self,
        hand_count: usize,
        cards_per_hand: usize,
    ) -> Result<Vec<Vec<Card>>, DeckDealError> {
        if self.len() < hand_count * cards_per_hand {
            return Err(DeckDealError::NotEnoughCards);
        }

        let mut ret_hands = Vec::with_capacity(hand_count);
        for _ in 0..hand_count {
            ret_hands.push(Vec::with_capacity(cards_per_hand));
        }

        for _ in 0..cards_per_hand {
            for hand in &mut ret_hands {
                hand.push(self.cards.pop().ok_or(DeckDealError::NotEnoughCards)?);
            }
        }

        Ok(ret_hands)
    }
}

impl Default for Deck {
    /// Creates a deck with one set of cards
    ///
    /// Equivalent to
    /// ```
    /// # use naipe::common::deck::Deck;
    /// use std::num::NonZeroU16;
    /// let deck = Deck::new(NonZeroU16::new(1).unwrap());
    /// ```
    fn default() -> Deck {
        Deck::new(NonZeroU16::new(1).unwrap())
    }
}

impl Extend<Card> for Deck {
    fn extend<T: IntoIterator<Item = Card>>(&mut self, iter: T) {
        self.cards.extend(iter);
    }
}

/// Errors related to dealing from a deck
#[derive(Copy, Clone, Debug, Error)]
pub enum DeckDealError {
    #[error("Not enough cards in deck to deal")]
    NotEnoughCards,
}
