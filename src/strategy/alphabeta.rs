use std::fmt;

use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
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
        // On let joueur à -1 comme ça, inversion directe
        let (best_move, best_value) = alpha_beta(depth, state, -1, -100, 100);
        // println!("{}", best_value);
        best_move
    }
}

fn alpha_beta(
    depth: u8,
    state: &Configuration,
    joueur: i8,
    alpha: i8,
    beta: i8,
) -> (Option<Movement>, i8) {
    let mut nouveau_joueur: i8 = 1;
    if joueur == 1 {
        nouveau_joueur = -1;
    }

    let best;
    if depth == 0 {
        let recupere: Vec<(Movement, i8)> = state
            .movements()
            .map(|m| (m, state.play(&m).value()))
            .collect();
        // On véifie que l'iterateur ne soit pas vide
        if recupere.clone().into_iter().count() == 0 {
            best = (Movement::Duplicate(0), -joueur * 100);
        } else {
            best = recupere.clone().into_iter().max_by_key(|&(_, val)| joueur * val) // Si joueur == 1, on cherche un max, sinon on cherche un min
            .unwrap();
        }
    } else {
        // On met le pire mouvement.
        best = (Some(Movement::Duplicate(0), -100));

        if state.movements().count() == 0 {
            // On retourne le pire move
            best = (Movement::Duplicate(0), -joueur * 100);
        // println!("depth {}", depth);
        } else {
            let meilleurScore = -100;
            let recupere: Vec<(Movement, i8)> = state
                .movements()
                .map(|m| {
                    match alpha_beta(
                        depth - 1,
                        &state.play(&m).clone(),
                        nouveau_joueur,
                        -beta,
                        -alpha,
                    ) {
                        // -alphabeta dans l'algorithme
                        (Some(_), y) => (m, -y),
                        _ => (Movement::Duplicate(0), -joueur * 100), // Trouver autre chose ici
                    }
                })
                .map(|(mov, value)| {
                    if (value > meilleurScore) {
                        meilleurScore = value;
                        if meilleurScore > alpha {
                            alpha = meilleurScore;
                            if alpha >= beta {
                                // Ici on doit retourner le meilleur Score.
                                // Faire un for?
                                alpha = meilleurScore
                            }
                        }
                    }
                })
                .collect();
            // On véifie que l'iterateur ne soit pas vide
            if recupere.clone().into_iter().count() == 0 {
                best = (Movement::Duplicate(0), -joueur * 100);
            } else {
                best = recupere.clone().into_iter().max_by_key(|&(_, val)| joueur * val) // Si joueur == 1, on cherche un max, sinon on cherche un min
                .unwrap();
            }
        }
    }

    (Some(Movement::Duplicate(0)), 0)
}
