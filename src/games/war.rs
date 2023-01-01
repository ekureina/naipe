//! An implementation of the card game War

use std::cmp::Ordering;

use crate::common::{card::Suitless, deck::Deck, hand::Hand};
use crate::games::Game;

/// Game state for the game of War
#[derive(Clone, Debug)]
pub struct WarGame {
    player_1_hand: Hand,
    player_2_hand: Hand,
    player_1_capture: Deck,
    player_2_capture: Deck,
}

impl WarGame {
    /// A convenience function to specify if the game is won by player 1
    fn player_1_won(&self) -> bool {
        self.player_2_hand.is_empty() && self.player_2_capture.is_empty()
    }
    /// A convenience function to specify if the game is won by player 2
    fn player_2_won(&self) -> bool {
        self.player_1_hand.is_empty() && self.player_2_capture.is_empty()
    }
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

impl Game for WarGame {
    type TickOk = bool;
    type TickError = ();

    /// Advances the game of war
    fn tick(&mut self) -> Result<bool, ()> {
        if self.player_1_won() || self.player_2_won() {
            return Ok(true);
        }

        if self.player_1_hand.is_empty() {
            self.player_1_capture.shuffle_with_default_rng();
            self.player_1_hand
                .extend(self.player_1_capture.deal_all_cards(1).unwrap()[0].clone());
        }

        if self.player_2_hand.is_empty() {
            self.player_2_capture.shuffle_with_default_rng();
            self.player_2_hand
                .extend(self.player_2_capture.deal_all_cards(1).unwrap()[0].clone());
        }

        let player_1_play = Suitless(self.player_1_hand.pop().unwrap());
        let player_2_play = Suitless(self.player_2_hand.pop().unwrap());

        match player_1_play.cmp(&player_2_play) {
            Ordering::Equal => {
                self.player_1_capture.add(player_1_play.unwrap());
                self.player_2_capture.add(player_2_play.unwrap());
            }
            Ordering::Less => {
                self.player_2_capture.add(player_1_play.unwrap());
                self.player_2_capture.add(player_2_play.unwrap());
            }
            Ordering::Greater => {
                self.player_1_capture.add(player_1_play.unwrap());
                self.player_1_capture.add(player_2_play.unwrap());
            }
        }

        Ok(false)
    }
}
