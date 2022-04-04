pub mod contestant;
pub mod gamemaster;
pub mod gamestrategy;

use crate::gamemaster::GameMaster;
use crate::gamestrategy::{GameStrategy, SimpleStrategy};

fn main() {
    let mut game = GameMaster::initialize_game(12, 100);
    let strategy: Box<dyn GameStrategy> = Box::new(SimpleStrategy::initialize(game.contestants()));

    while game.in_progress() {
        let was_correct = game.make_guess(&strategy.guess());
        strategy.handle_feedback(was_correct);
    }
    game.output_stats();
}
