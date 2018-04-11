use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;
use rayon::prelude::*;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    let mut d = 0;
    for depth in 1..100 {
        d += 1;
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
    println!("deniere depth {:?}", d);
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // profondeur de l'algorithme
        let depth = self.0;
        println!("{}", depth);
        let tour: i8;
        if (depth as u16 as i8) % 2 == 1 {
            tour = -1;
        } else {
            tour = 1; // 1
        }
        // On vérifie qu'il y ait au moins un mouvement à jouer
        let mov = alpha_beta(depth, state, tour, -100, 100);
        let mut best_move = None;
        if !mov.is_none() {
            best_move = mov.unwrap().0;
        }
        // println!("{}", best_value);
        best_move
    }
}

// Alpha beta
pub fn alpha_beta(
    depth: u8,
    state: &Configuration,
    joueur: i8,
    mut alpha: i8,
    beta: i8,
) -> Option<(Option<Movement>, i8)> {
    let best: Option<(Option<Movement>, i8)>;
    let mut tmp_best: (Option<Movement>, i8) = (None, -100);

    if depth == 0 {
        let best_tmp: Vec<Movement>;
        best_tmp = state.movements().collect();
        best = best_tmp
            .into_par_iter()
            .map(|m| (Some(m), state.play(&m).value()))
            .max_by_key(|&(_, val)| joueur * val);
    } else {
        for m in state.movements() {
            let mov = match alpha_beta(depth - 1, &state.play(&m).clone(), -joueur, -beta, -alpha) {
                Some((Some(_), y)) => (Some(m), -y),
                _ => (None, -joueur * 100),
            };
            if mov.0.is_none() {
                continue;
            }
            if mov.1 > tmp_best.1 {
                tmp_best = mov;
                if tmp_best.1 > alpha {
                    alpha = tmp_best.1;
                    if alpha >= beta {
                        break;
                    };
                }
            }
        }

        if tmp_best.0.is_none() {
            best = None;
        } else {
            best = Some(tmp_best);
        }
    }
    best
}
