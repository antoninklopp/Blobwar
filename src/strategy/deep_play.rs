use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

/// Deep Play algorithme with the number of game play for each play at time t
pub struct DeepPlay(pub u16);

impl fmt::Display for DeepPlay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deep - Play (max level: {})", self.0)
    }
}

impl Strategy for DeepPlay {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // nombre de partie jouées pour hcaque position
        let nb_game = self.0;
        // On vérifie qu'il y ait au moins un mouvement à jouer
        let mov = deep_play(nb_game, state);
        // println!("{}", best_value);
        mov.unwrap().0
    }
}

fn play_randomly(state: &Configuration, mov: Movement, nb_game: u16) -> i32 {
    let mut value: i32 = 0;
    // play nb_game games
    for _ in 1..nb_game {
        let mut new_state: &Configuration = &state.play(&mov).clone();

        if result_partie(new_state) {
            // println!("value {:?}", value);
            value += 1;
        }
    }
    println!("Nombre gagnées {:?} jouées {:?}", value, nb_game);
    value
}

// Fonction qui retourne un boolean :
// true si gagné
// false si perdu
// TODO : WARNING : Il faut que le joueur 1 joue en premier
fn result_partie(state: &Configuration) -> bool {
    let mut valeur_retour = true;
    if state.game_over() {
        valeur_retour = state.winner();
    // println!("gameover {:?}", valeur_retour);
    } else {
        let nb_mov_possibles = state.movements().count();
        // create a random index generation
        let mut selected_index = 0; // Impossible de mettre -1, on met une trop grande valeur?
        let mut index_ok = true;
        let mut rng = thread_rng();
        if rng.gen() {
            // random bool
            // random generation from 0 to nb_mov_possibles-1
            // println!("mouvements possibles {:?}", nb_mov_possibles);
            if nb_mov_possibles == 0 {
                index_ok = false;
            } else {
                selected_index = rng.gen_range(0, nb_mov_possibles);
            }
        }
        if index_ok {
            // selected the move randomly
            let mut selected_mov_tmp = state.movements().nth(selected_index);
            let mut i = 0;
            while selected_mov_tmp.is_none() && i < nb_mov_possibles {
                i += 1;
                selected_mov_tmp = state
                    .movements()
                    .nth((selected_index + i) % nb_mov_possibles);
            }

            if selected_mov_tmp.is_none() {
                // On ne peut pas aller plus bas, on dir qu'on s'arrete ici
                valeur_retour = false;
            } else {
                let selected_mov = selected_mov_tmp.unwrap();
                // Normalement pas de probleme de panic à l'unwrap
                // println!("Mouvement choisi {:?}", selected_mov);
                valeur_retour = result_partie(&state.play(&selected_mov).clone());
            }
        } else {
            // valeur_retour = state.winner();
            valeur_retour = false;
        }
    }
    valeur_retour
}

pub fn deep_play(nb_game: u16, state: &Configuration) -> Option<(Option<Movement>, i8)> {
    let best: Option<(Option<Movement>, i8)>;
    let best_tmp: Vec<Movement>;
    let result: Option<(Option<Movement>, i8)>;
    best_tmp = state.movements().collect();
    best = best_tmp
        .into_par_iter()
        .map(|mov| (Some(mov), play_randomly(state, mov, nb_game)))
        .max_by_key(|&(_, val)| val)
        .map(|(mov, val)| (mov, val as i8));

    if best.is_none() {
        result = None;
    } else {
        result = best;
    }
    result
}
