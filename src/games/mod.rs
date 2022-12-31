//! A collection of basic games included with naipe

#[cfg(feature = "war")]
pub mod war;

pub trait Game {
    #[must_use]
    fn tick(self) -> Self;
}
