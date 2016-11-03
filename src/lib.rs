
//! pusoy dos core crate

extern crate rand;
extern crate rustc_serialize;

///! generic(ish) card module
#[macro_use]
pub mod cards;
///! game specific module
pub mod game;

#[cfg(test)]
mod tests;


