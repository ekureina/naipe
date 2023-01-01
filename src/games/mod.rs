//! A collection of basic games included with naipe

#[cfg(feature = "war")]
pub mod war;

pub trait Game {
    type TickOk;
    type TickError;

    /// Tick the game, advancing play
    #[allow(clippy::missing_errors_doc)]
    fn tick(&mut self) -> Result<Self::TickOk, Self::TickError>;
}
