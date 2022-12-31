use std::fmt::{Display, Formatter, self};

/// An enum representing the rank of a card
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    /// Get all the ranks in the enum in a vector
    /// # Examples
    /// ```
    /// # use naipe::common::card::Rank;
    /// let ranks = Rank::all_ranks();
    /// assert_eq!(ranks.len(), 13);
    /// ```
    pub fn all_ranks() -> Vec<Rank> {
        vec![
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }

    /// Compares the two ranks, and determines if the rank is
    /// directly after this one
    /// 
    /// # Examples
    /// ```
    /// # use naipe::common::card::Rank;
    /// let ace = Rank::Ace;
    /// let two = Rank::Two;
    /// assert!(two.is_directly_after(ace.clone()));
    /// ```
    ///
    /// ```
    /// # use naipe::common::card::Rank;
    /// let ace = Rank::Ace;
    /// let king = Rank::King;
    /// assert!(!ace.is_directly_after(king.clone()));
    /// ```
    pub fn is_directly_after(&self, other_rank: Rank) -> bool {
        match self {
            Rank::Ace => false,
            Rank::Two => other_rank == Rank::Ace,
            Rank::Three => other_rank == Rank::Two,
            Rank::Four => other_rank == Rank::Three,
            Rank::Five => other_rank == Rank::Four,
            Rank::Six => other_rank == Rank::Five,
            Rank::Seven => other_rank == Rank::Six,
            Rank::Eight => other_rank == Rank::Seven,
            Rank::Nine => other_rank == Rank::Eight,
            Rank::Ten => other_rank == Rank::Nine,
            Rank::Jack => other_rank == Rank::Ten,
            Rank::Queen => other_rank == Rank::Jack,
            Rank::King => other_rank == Rank::Queen,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Rank::Ace => write!(f, "A"),
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "J"),
            Rank::Queen => write!(f, "Q"),
            Rank::King => write!(f, "K"),
        }
    }
}
