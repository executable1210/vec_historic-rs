#![doc = include_str!("../README.md")]

mod public;
mod private;
mod defines;
mod vec_historic;
pub mod macros;
pub mod factory;
pub mod defines_impl;
pub mod traits_impl;
pub use defines::*;
pub use vec_historic::VecHistoric;