//! An implementation of the card game War

use crate::common::{deck::Deck, hand::Hand};

/// Game state for the game of War
#[derive(Clone, Debug)]
pub struct WarGame {
    player_1_hand: Hand,
    player_2_hand: Hand,
    player_1_capture: Deck,
    player_2_capture: Deck,
}

impl Default for WarGame {
    fn default() -> WarGame {
        let mut players = vec![Hand::default(), Hand::default()];
        let mut deck = Deck::default();
        deck.deal_all_cards_to_hands(&mut players).unwrap();
        WarGame {
            player_1_hand: players[0].clone(),
            player_2_hand: players[1].clone(),
            player_1_capture: Deck::new_empty(),
            player_2_capture: Deck::new_empty(),
        }
    }
}
