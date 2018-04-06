extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, DeepPlay, Greedy, MinMax, MixAndTwist, Random};
use blobwar::strategy::{IterativeDeepening, IterativeStrategy, NetworkPlayer};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle(DeepPlay(2), Random(6, 1.0));
    // game.battle(Greedy(), Random(12, 1.0));
}
