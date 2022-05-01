pub mod contestant;
pub mod gamemaster;
pub mod gamestrategy;
pub mod bruteforce;
pub mod utils;
pub mod round;
pub mod roundmanager;

use std::time;
use env_logger;
use log::{debug, error, trace, info};
use crate::utils::pretty_string_poss;
use crate::contestant::ContestantPairs;
use crate::contestant::Players;
use crate::gamemaster::GameMaster;
use crate::gamestrategy::GameStrategy;
use crate::bruteforce::BruteForce;


fn main() {
    env_logger::init();

    let mut game = GameMaster::initialize_game(12, 500);
    let mut strategy = BruteForce::initialize(game.contestants());
    info!("-------------------------Correct pairing-------------------------\n{}", ContestantPairs(&game.matches));

    let time_sleep = time::Duration::from_secs(3);
    std::thread::sleep(time_sleep);

    while game.in_progress() {
        info!("\\/\\/\\/\\/\\/\\/\\/\\/ROUND {}\\/\\/\\/\\/\\/\\/\\/\\/", game.get_iterations());
        let guess = strategy.ceremony_pairs();
        info!("Ceremony guess -----------------------------------------------");

        let num_correct = game.ceremony(&guess);
        if num_correct == game.contestants().len() /2 {
            break;
        }

        strategy.ceremony_feedback(num_correct, guess);
        pretty_string_poss(&strategy.possibilities);
        debug!("Poss remaining: {}, Num correct: {}", strategy.poss_left(), game.num_matched);
        
        let booth_result = game.truth_booth(strategy.send_to_booth());
        strategy.booth_feedback(booth_result);
    }
    game.output_stats();
}
