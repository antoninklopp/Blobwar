//! Dumb greedy algorithm.
use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use rayon::prelude::*;

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // Iterer sur tous les pions et sur toutes les positions possibles
        // Regarder le max de pions récupérés

        // Variable du mouvement du joueur
        let tmp_best: Vec<Movement> = state.movements().collect();
        let (config, _) = tmp_best
            .into_par_iter()
            .map(|m| (m, state.play(&m).value()))
            .max_by_key(|&(_, val)| val)
            .unwrap();

        // Valeur du mouvement de retour.
        Some(config)
    }
}
