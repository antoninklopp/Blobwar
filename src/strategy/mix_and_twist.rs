use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;
use strategy::alphabeta::alpha_beta;
// use super::Strategy::alphabeta::{alphabeta};
use strategy::random::random;

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct Mix_and_Twist(pub u8, pub u8, pub f32);

impl fmt::Display for Mix_and_Twist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mix_and_Twist (max level: {})", self.0)
    }
}

impl Strategy for Mix_and_Twist {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let depth_alphabeta: u8 = self.0;
        let depth_random = self.1;
        let percentage = self.2;

        let tour_alphabeta: i8;
        if (depth_alphabeta as u16 as i8) % 2 == 1 {
            tour_alphabeta = -1;
        } else {
            tour_alphabeta = 1; // 1
        }

        let tour_random: i8;
        if (depth_random as u16 as i8) % 2 == 1 {
            tour_random = -1;
        } else {
            tour_random = 1; // 1
        }

        let alphabeta_mov: Option<(Option<Movement>, i8)> =
            alpha_beta(depth_alphabeta, state, tour_alphabeta, -100, 100);
        let random_mov: Option<(Option<Movement>, i8)> =
            random(depth_random, state, tour_random, -100, 100, percentage);
        let v = vec![alphabeta_mov, random_mov];

        let best_move_tmp = v.into_iter()
            .map(|x| match x {
                Some((Some(mov), y)) => (Some(mov), y),
                _ => (None, 0),
            })
            .filter(|&(x, _)| !x.is_none())
            .max_by_key(|&(_, val)| val);

        let mut best_move = None;
        if !best_move_tmp.is_none() {
            best_move = best_move_tmp.unwrap().0;
        }
        best_move
    }
}
