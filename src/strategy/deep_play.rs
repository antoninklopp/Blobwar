use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;
use rand::{Rng, thread_rng};

/// Deep Play algorithme with the number of game play for each play at time t
pub struct DeepPlay(pub u8);

impl fmt::Display for DeepPlay{
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
        mov
    }
}

fn play_randomly(state: &Configuration, mov: Movement, nb_game: u8) -> i8 {

    let value = 0;
    // play nb_game games
    for _ in 1..nb_game {
        let new_state: &Configuration = &state.play(&mov).clone();
        let game_is_on = true;
        // play randomly until the game finished
        while game_is_on {
            // count the number of nomve possible
            let nb_mov_possibles = new_state.movements().count();
            // create a random index generation
            let selected_index = 0;
            let mut rng = thread_rng();
            if rng.gen() { // random bool
                // random generation from 0 to nb_mov_possibles-1
                selected_index = rng.gen_range(0, nb_mov_possibles);
            }
            // selected the move randomly
            let selected_mov = new_state.movements().nth(selected_index);


            // new_state.play(&selected_mov);
            // if movements_possible == None {
            //     game_is_on = false;
            }
        }
    }
    value
}

// Alpha beta
pub fn deep_play(
    nb_game: u8,
    state: &Configuration,
) -> Option<Movement> {

    let best: Option<(Option<Movement>, i8)>;
    let result: Option<Movement>;
    best = state.movements().map(|mov| (Some(mov), play_randomly(state, mov, nb_game))).max_by_key(|&(_, val)| val);

    if best.is_none() {
        result = None;
    } else {
        result = best.unwrap().0;
    }
    result
}
