pub mod contestant;
pub mod gamemaster;
pub mod gamestrategy;
pub mod bruteforce;

use crate::gamemaster::GameMaster;
use crate::gamestrategy::GameStrategy;
use crate::bruteforce::BruteForce;


fn main() {
    let mut game = GameMaster::initialize_game(12, 500);
    let mut strategy: Box<dyn GameStrategy> = Box::new(BruteForce::initialize(game.contestants()));

    while game.in_progress() {
        let num_correct = game.ceremony(strategy.ceremony_pairs());
        strategy.ceremony_feedback(num_correct);
        let booth_result = game.truth_booth(strategy.send_to_booth());
        strategy.booth_feedback(booth_result);
    }
    game.output_stats();
}
