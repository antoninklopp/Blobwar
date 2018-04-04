extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, Greedy, MinMax, Random};
use blobwar::strategy::{IterativeDeepening, IterativeStrategy, NetworkPlayer};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle(Greedy(), Random(25, 0.5));
}
