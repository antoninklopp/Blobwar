extern crate blobwar;
extern crate rand;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, DeepPlay, Greedy, MinMax, MixAndTwist, Random};
use blobwar::strategy::{IterativeDeepening, IterativeStrategy, NetworkPlayer};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    // game.battle(
    //     Greedy(),
    //     IterativeDeepening::new(IterativeStrategy::AlphaBeta),
    // );
    //game.battle(DeepPlay(10000), Greedy());
    game.battle(
        IterativeDeepening::new(IterativeStrategy::MixAndTwist),
        DeepPlay(10000),
    );
    // game.battle(Greedy(), Random(10, 1.0));
}
