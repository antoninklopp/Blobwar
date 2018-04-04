//! Implementation of the min max algorithm.
use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // profondeur de l'algorithme
        let depth = self.0;
        // On let joueur à -1 comme ça, inversion directe
        let tour: i8;
        if (depth as u16 as i8) % 2 == 1 {
            tour = -1;
        } else {
            tour = 1;
        }
        let (best_move, _) = match compute_depth(depth, state, tour) {
            Some((mov, y)) => (mov, y),
            _ => (None, 0),
        };
        println!("{:?}", best_move);
        // println!("{}", best_value);
        best_move
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}

//renvoie le meilleur mouvement et sa valeur.
fn compute_depth(depth: u8, state: &Configuration, joueur: i8) -> Option<(Option<Movement>, i8)> {
    // joueur == 1 si c'est notre joueur, -1 si c'est le joueur adverse

    // println!("deth {}", depth);

    let best: Option<(Option<Movement>, i8)>;
    // Si on est arrivé au bout de la profondeur
    if depth == 0 {
        // Meme implementation que le greedy.
        best = state
            .movements()
            .map(|m| (Some(m), state.play(&m).value()))
            .max_by_key(|&(_, val)| joueur * val);
    } else {
        best = state
            .movements()
            .map(
                |m| match compute_depth(depth - 1, &state.play(&m).clone(), -joueur) {
                    Some((Some(_), y)) => (Some(m), y),
                    _ => (None, joueur * 100), // Trouver autre chose ici
                },
            )
            .max_by_key(|&(_, val)| joueur * val); // Si joueur == 1, on cherche un max, sinon on cherche un min
    }
    best
}
