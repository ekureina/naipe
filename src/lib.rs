#![warn(nonstandard_style)]
#![warn(deprecated_in_future)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

//! A crate implementing basic card game concepts, along with some games
//!
//! Common structs are in the [`crate::common`] module

pub mod common;
#[cfg(any(feature = "war"))]
pub mod games;
