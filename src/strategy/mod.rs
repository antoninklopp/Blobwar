//! We provide here structs for all possible kinds of players and AI.
use std::fmt;
use configuration::{Configuration, Movement};

/// To be a strategy you need to be able to compute the next move.
pub trait Strategy: fmt::Display {
    /// Take current `Configuration` and return what to do next.
    /// None if no move is possible.
    fn compute_next_move(&mut self, configuration: &Configuration) -> Option<Movement>;
}

pub mod human;
pub use self::human::Human;
pub mod network;
pub use self::network::NetworkPlayer;
pub mod greedy;
pub use self::greedy::Greedy;
pub mod minmax;
pub use self::minmax::{min_max_anytime, MinMax};
pub mod alphabeta;
pub use self::alphabeta::{alpha_beta_anytime, AlphaBeta};
pub mod mix_and_twist;
pub use self::mix_and_twist::MixAndTwist;
pub mod random;
pub use self::random::{random_anytime, Random};
pub mod iterative;
pub use self::iterative::IterativeDeepening;
pub use self::iterative::IterativeStrategy;
