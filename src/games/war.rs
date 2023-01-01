//! An implementation of the card game War

use std::cmp::Ordering;

use log::debug;

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
    pub fn player_1_won(&self) -> bool {
        self.player_2_hand.is_empty() && self.player_2_capture.is_empty()
    }
    /// A convenience function to specify if the game is won by player 2
    pub fn player_2_won(&self) -> bool {
        self.player_1_hand.is_empty() && self.player_1_capture.is_empty()
    }

    fn player_1_card_count(&self) -> usize {
        self.player_1_hand.len() + self.player_1_capture.len()
    }

    fn player_2_card_count(&self) -> usize {
        self.player_2_hand.len() + self.player_2_capture.len()
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
            debug!(
                "Finished! (Player 1 Count: {}, Player 2 Count: {})",
                self.player_1_card_count(),
                self.player_2_card_count()
            );
            return Ok(true);
        }

        if self.player_1_hand.is_empty() {
            self.player_1_capture.shuffle_with_default_rng();
            self.player_1_hand
                .extend(self.player_1_capture.deal_all_cards(1).unwrap()[0].clone());
            debug!(
                "Reshuffling player 1! Current card count: {} (Player 2 count: {})",
                self.player_1_card_count(),
                self.player_2_card_count()
            );
        }

        if self.player_2_hand.is_empty() {
            self.player_2_capture.shuffle_with_default_rng();
            self.player_2_hand
                .extend(self.player_2_capture.deal_all_cards(1).unwrap()[0].clone());
            debug!(
                "Reshuffling player 2!, Current card Count: {} (Player 1 count: {})",
                self.player_2_card_count(),
                self.player_1_card_count()
            );
        }

        let player_1_play = Suitless(self.player_1_hand.pop().unwrap());
        let player_2_play = Suitless(self.player_2_hand.pop().unwrap());
        debug!(
            "Player 1: {}, Player 2: {}",
            player_1_play.0, player_2_play.0
        );

        match player_1_play.cmp(&player_2_play) {
            Ordering::Equal => {
                debug!("{} == {}", player_1_play, player_2_play);
                self.player_1_capture.add(player_1_play.unwrap());
                self.player_2_capture.add(player_2_play.unwrap());
            }
            Ordering::Less => {
                debug!("{} < {}", player_1_play, player_2_play);
                self.player_2_capture.add(player_1_play.unwrap());
                self.player_2_capture.add(player_2_play.unwrap());
            }
            Ordering::Greater => {
                debug!("{} > {}", player_1_play, player_2_play);
                self.player_1_capture.add(player_1_play.unwrap());
                self.player_1_capture.add(player_2_play.unwrap());
            }
        }

        debug!(
            "Player 1 Count: {}, Player 2 Count: {}",
            self.player_1_card_count(),
            self.player_2_card_count()
        );

        Ok(false)
    }
}
