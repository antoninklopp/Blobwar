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
        let tour: i8;
        if (depth as u16 as i8) % 2 == 1 {
            tour = -1;
        } else {
            tour = 1;
        }
        // On vérifie qu'il y ait au moins un mouvement à jouer
        let (best_move, best_value) = match alpha_beta(depth, state, tour, -100, 100) {
            Some((mov, y)) => (mov, y),
            _ => (None, 0),
        };
        print!("{:?}", best_value);
        // println!("{}", best_value);
        best_move
    }
}

// Alpha beta
fn alpha_beta(
    depth: u8,
    state: &Configuration,
    joueur: i8,
    alpha: i8,
    beta: i8,
) -> Option<(Option<Movement>, i8)> {
    let best: Option<(Option<Movement>, i8)>;

    if depth == 0 {
        best = state
            .movements()
            .map(|m| (Some(m), state.play(&m).value()))
            .filter(|&(mov, _)| !mov.is_none()) // On vérifie que la valeur n'est pas nulle.
            .max_by_key(|&(_, val)| joueur * val);
        best
    } else {
        // On met le pire mouvement.

        if state.movements().count() == 0 {
            // On retourne le pire move
            best = Some((None, -joueur * 100));
        // println!("depth {}", depth);
        } else {
            let mut recupere2 = state
                .movements()
                .map( |m| match alpha_beta(depth - 1, &state.play(&m).clone(), -joueur, -beta, -alpha) {
                    Some((Some(_), y)) => (Some(m), y),
                    _ => (None, -joueur * 100) // Trouver autre chose ici
                })
                .filter(|&(mov, _)| !mov.is_none()) // On vérifie que la valeur n'est pas nulle.
                .peekable() // TODO : Solution?
                .scan((alpha, beta, (None, -100)), |state, (mov, value)|{ // Dans l'ordre, alpha, beta, meilleurScore
                    let mut trouve:bool = false;
                    let best_value = state.2;
                    if value > best_value.1 {
                        state.2 = (mov, value);
                        if value > state.0 {
                            state.0 = value;
                            if state.0 >= state.1 {
                                trouve = true;
                                // return
                            }
                        }
                    }
                    Some((state.2, trouve)) // Retourne une opion sur la valeur
                });

            let iter_debut = recupere2
                .by_ref()
                .take_while(|&((_, _), trouve)| !trouve)
                .last()
                .map(|((mov, value), _)| (mov, value));

            let premier_true = recupere2.next();
            if premier_true.is_none() {
                best = iter_debut;
            } else {
                best = premier_true.map(|((mov, value), _)| (mov, value));
            }
        }
        best
    }
}
