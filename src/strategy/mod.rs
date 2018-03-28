//! We provide here structs for all possible kinds of players and AI.
use std::fmt;
use configuration::{Configuration, Movement};

/// To be a strategy you need to be able to compute the next move.
pub trait Strategy: fmt::Display {
    /// Take current `Configuration` and return what to do next.
    /// None if no move is possible.
    fn compute_next_move(&mut self, configuration: &Configuration) -> Option<Movement>;
}
