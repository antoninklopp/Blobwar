use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;
use strategy::alphabeta::alpha_beta;
// use super::Strategy::alphabeta::{alphabeta};
use strategy::random::random;
use strategy::deep_play::deep_play;

pub fn mix_and_twist_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    let mut d = 0;
    for depth in 1..100 {
        d += 1;
        let chosen_movement = MixAndTwist(depth, depth + 4, 1.0).compute_next_move(state);
        movement.store(chosen_movement);
    }
    println!("deniere depth {:?}", d);
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct MixAndTwist(pub u8, pub u8, pub f32);

impl fmt::Display for MixAndTwist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mix_and_Twist (max level: {})", self.0)
    }
}

impl Strategy for MixAndTwist {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let depth_alphabeta: u8 = self.0;
        let depth_random = self.1;
        let percentage = self.2;
        println!(
            "alpha_beta depth = {}, random_depth = {}",
            depth_alphabeta, depth_random
        );
        // println!("MixAndTwist depth = {}", depth_alphabeta);

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

        let mut random_mov: Option<(Option<Movement>, i8)> = Some((None, -100));
        if depth_alphabeta >= 4 {
            random_mov = random(depth_random, state, tour_random, -100, 100, percentage);
        }
        // if depth_alphabeta == 5 {
        //     let var: u16 = 1000;
        //     random_mov = deep_play(var, state);
        // }
        let alphabeta_mov: Option<(Option<Movement>, i8)> =
            alpha_beta(depth_alphabeta, state, tour_alphabeta, -100, 100);

        let v = vec![alphabeta_mov, random_mov];
        println!("alpha & random {:?}", v);
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
        println!("Best move = {:?}", best_move);
        best_move
    }
}
