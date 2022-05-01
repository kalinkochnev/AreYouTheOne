pub mod contestant;
pub mod gamemaster;
pub mod gamestrategy;
pub mod bruteforce;
pub mod utils;
pub mod round;
pub mod roundmanager;

use std::time;
use crate::utils::pretty_print_poss;
use crate::contestant::ContestantPairs;
use crate::contestant::Players;
use crate::gamemaster::GameMaster;
use crate::gamestrategy::GameStrategy;
use crate::bruteforce::BruteForce;



/*Plan-----------
First round strategy-- 
1. Generate completely random combos of players
2. Get feedback, adjust based on the strategy
3. Pick the most likely pair (if they are all just as likely, pick any pair from the set of possibilities)

Strategy 1 -- Brute force
1. 

Strategy 2 -- Probabilistic + some brute force
Assuming the game has N contestants and N/2 pairs (P),
Picking the optimal pair:
-   If we have some W # of correct guesses, there is a 
        Likelihood=(# correct)/(# pairs in ceremony)
    chance that you would randomly pick a correct guess from the ceremony.

-   We can then "distribute" that percent chance across the number of pairs in the ceremony.
    We increase the probability of a pair being right by an additional 
        Likelihood/(# of remaining pairs)

-   Likewise, if we found W correct contestants, the complement 
        Likelihood not in ceremony = (# pairs in ceremony - W)/(# pairs in the ceremony) 
    is the probability that you won't find the correct pair in the ceremony. That means it must be
    in the set of remaining pairs. You can then distribute that probability across the # of pairs
    that are left, so 
        (Likelihood not in ceremony)/(# of remaining pairs)

Strategy 3 -- Smart/Improved/Adaptive/Bayes probabilistic
The goal is to re-expose pairs consistently and adapt probabilities that a pair is correct,
given that a pair re-exposed and is eliminated in a future round.


P(pair is perfect match | in round with n perfect matched)
= 
P(pair is perfect match | in x rounds where perfect matches were contained)
 = P(in x rounds where perfect matches were contained | pair is a perfect match) P(A is a perfect match)
 / P(in x rounds where perfect matches were contained)


 P(in x rounds where perfect matches were contained) =  / Rounds exposed 
 (# of rounds NOT contained) 
  = 100% * Likelihood / )



Basically, the Likelihood/(# of remaining pairs) is dependent on the # of players at the time
of the ceremony. However, given the fact that if a pair has managed to survive multiple rounds
Instead of increasing probabilities, we instead keep track of the total # of correct guesses this 
pair was exposed 2, as well as the # of rounds the correct guess was believed to be outside of the
ceremony set. 

For example. If pair A was in 3 rounds that had feedbacks of 1 correct, 2 correct, 0 correct (total 3),
and then pair A spent 2 rounds outside of the ceremony where subsequent rounds had said there were
2 correct, 3 correct, 
we recalculate the probability based on the # of remaining pairs.

*/


/*
Goals:
1. Use a particular strategy to play the game
2. Plot the average number of rounds
*/


fn main() {
    let mut game = GameMaster::initialize_game(12, 500);
    let mut strategy = BruteForce::initialize(game.contestants());
    println!("{}", ContestantPairs(&game.matches));
    let ten_millis = time::Duration::from_secs(3);

    std::thread::sleep(ten_millis);

    while game.in_progress() {
        println!("\\/\\/\\/\\/\\/\\/\\/\\/ROUND {}\\/\\/\\/\\/\\/\\/\\/\\/", game.get_iterations());

        let guess = strategy.ceremony_pairs();
        println!("Ceremony guess -----------------------------------------------");
        let num_correct = game.ceremony(&guess);
        if num_correct == game.contestants().len() /2 {
            break;
        }
        strategy.ceremony_feedback(num_correct, guess);
        pretty_print_poss(&strategy.possibilities);
        println!("Poss remaining: {}, Num correct: {}", strategy.poss_left(), game.num_matched);
        let booth_result = game.truth_booth(strategy.send_to_booth());
        strategy.booth_feedback(booth_result);
    }
    game.output_stats();
}
