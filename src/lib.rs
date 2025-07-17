#![doc = include_str!("../README.md")]

pub use gapbuf::gap_buffer;

mod public;
mod private;
mod defines;
mod macros;
mod vec_historic;
pub mod factory;
pub mod defines_impl;
pub mod traits_impl;

// pub use traits_impl::*;
// pub use defines_impl::*;
// pub use public::*;
// pub use macros::*;
pub use defines::*;
pub use vec_historic::VecHistoric;