
//! pusoy dos core crate

extern crate rand;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

///! generic(ish) card module
#[macro_use]
pub mod cards;
///! game specific module
pub mod game;

#[cfg(test)]
mod tests;


