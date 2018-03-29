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
        let (best_move, best_value) = alpha_beta(depth, state, -1, -100, 100).unwrap();
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
    let mut nouveau_joueur: i8 = 1;
    if joueur == 1 {
        nouveau_joueur = -1;
    }

    let mut best: Option<(Option<Movement>, i8)>;
    if depth == 0 {
        best = state
            .movements()
            .map(|m| (Some(m), state.play(&m).value()))
            .filter(|&(mov, _)| !mov.is_none()) // On vérifie que la valeur n'est pas nulle.
            .max_by_key(|&(_, val)| joueur * val);
        best
    } else {
        // On met le pire mouvement.
        best = Some((None, -100));

        if state.movements().count() == 0 {
            // On retourne le pire move
            best = Some((None, -joueur * 100));
        // println!("depth {}", depth);
        } else {
            print!("Je suis ici\n");
            let mut recupere2 = state
                .movements()
                .map( |m| match alpha_beta(depth - 1, &state.play(&m).clone(), nouveau_joueur, -beta, -alpha) {
                    Some((Some(_), y)) => (Some(m), y),
                    _ => (None, -joueur * 100) // Trouver autre chose ici
                })
                .filter(|&(mov, _)| !mov.is_none()) // On vérifie que la valeur n'est pas nulle.
                .peekable() // TODO : Solution? 
                .scan((-100, 100, (None, -100)), |state, (mov, value)|{ // Dans l'ordre, alpha, beta, meilleurScore
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

            let taille = recupere2.by_ref().count(); //TODO: PROBLEME ICI : On consomme le vecteur

            print!("taille{:?}\n", taille);

            let recupere = recupere2
                .enumerate()
                .inspect(|&(i, _)| println!("{:?} {:?}", taille, i))
                .find(|&(i, ((_, _), trouve))| (trouve || (i == taille - 2))) // On prend le premier à true ou le dernier
                .map(|(_, ((mov, value), _))| (mov, value));

            best = recupere;

            if !best.is_none() {
                let (_, value) = best.unwrap();
                print!("Meilleur move{:?}\n", value);
            } else {
                println!("Je suis null");
            }

            // let mut recupere_clone = recupere.cloned();
            //
            // let result_find = recupere_clone.find(|&((_, _), trouve)| trouve==true);
            //
            // .filter(|&(mov, _)| !mov.is_none()).next(); // On s'arrete si alpha >= beta.

            // best = match recupere {
            //     Some((mov, value)) => (mov, value),
            //     _ => (None, 0),
            // };
        }
        best
    }
}
