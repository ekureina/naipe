//! A collection of basic games included with naipe

#[cfg(feature = "war")]
pub mod war;

pub trait Game {
    fn tick(self) -> Self;
}
