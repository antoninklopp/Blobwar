use std::fmt;

use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;

impl<'a> Global<'a> {
    pub fn derniere_feuille(state: &Configuration, joueur: i8) -> (Movement, i8)
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
        best
    }
}
