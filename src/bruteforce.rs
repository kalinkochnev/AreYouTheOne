use crate::Players;
use crate::contestant::ContestantPairs;
use crate::utils::pairs_to_contestants;
use crate::roundmanager::RoundManager;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::Iterator;
use crate::gamestrategy::{GameStrategy, Feedback};
use crate::round::SavedRound;
use crate::contestant::{Player, ContestantPair};

pub struct BruteForce {
    contestants: HashSet<Player>,
    right_matches: HashSet<ContestantPair>,
    pub possibilities: HashMap<Player, HashSet<Player>>,
    round_manager: RoundManager,
}

#[derive(Debug, PartialEq)]
pub enum PairPlayerResult {
    Ok(ContestantPair),
    Contradiction
}

impl BruteForce {
    /* 
    This algorithm attempts to match people who have never been matched before until
    there is at least 1 success. It then keeps going to the truth booth until that 
    the perfect match is identified. It then fixes those matches in place and attempts
    to keep finding people who have not been found. 

    */

    pub fn initialize(contestants: Vec<&Player>) -> Self {
        let cloned_contestants: HashSet<Player> = HashSet::from_iter(contestants.into_iter().cloned());
        let mut possibilities = HashMap::new(); 

        // Initializes which contestants haven't been checked yet
        // Remove player from key-value pair so that they can't be 
        // matched with themselves
        for player in cloned_contestants.iter() {
            let mut unpaired_contestants = HashSet::from_iter(cloned_contestants.clone());
            unpaired_contestants.remove(player);

            possibilities.insert(player.clone(), unpaired_contestants);
        }

        Self { 
            contestants: cloned_contestants, 
            right_matches: HashSet::new(),
            round_manager: RoundManager::new(),
            possibilities,
        }
    }

    fn already_guessed(&self, p1: &Player, p2: &Player) -> bool {
        if !self.contestants.contains(p1) || !self.contestants.contains(p2) {
            panic!("Player checked is not a part of the game!");
        }
        // If it does not contain either player in the set of keys, it has been found already!
        if !(self.possibilities.contains_key(p1) && self.possibilities.contains_key(p2)) {
            return true;
        }
        
        let p1_poss = self.possibilities.get(p1).expect("Player 1 should be key in possibilties but is nonexistant");
        let p2_poss = self.possibilities.get(p2).expect("Player 2 hould be key in possibilties but is nonexistant");

        return !p2_poss.contains(p1) && !p1_poss.contains(p2); // if they are both not contained, then it has already been guessed
    }

    /* This method should remove the possible pair (A, B) from the set of possibilities.
        This should only be used when eliminating a single possibility and not when a
        correct pair is found. 
    */
    fn remove_guess(&mut self, pair: &ContestantPair) {
        let p1 = pair.get_a();
        let p2 = pair.get_b();
        if !self.already_guessed(p1, p2) {
            // Throw error is somehow a hashset contains no options left
            if self.possibilities[p1].len() == 0 || self.possibilities[p2].len() == 0{
                panic!("Book keeping error for {}. Not possible to reach point when only <= 1 possibility left and it needs to be removed", p1)
            } 
            self.possibilities.get_mut(p1).expect("Attempted to remove p2 from p1 guesses").remove(p2);
            self.possibilities.get_mut(p2).expect("Attempted to remove p1 from p2 guesses").remove(p1);

            // eliminate guesses from rounds
            self.round_manager.eliminate_guesses(vec![pair.clone()]);
        }
    }


    fn pair_player(&self, player: &Player, off_limits: &HashSet<Player>) -> PairPlayerResult {
        // Given a player, try to get a player who isn't already picked. 
        let player_poss = self.possibilities.get(player).expect("Player not in possibilities");

        let mut can_pair_with = player_poss.difference(off_limits);

        // If that is not possible, return a contradiction
        return match can_pair_with.next() {
            Some(other_player) => PairPlayerResult::Ok(ContestantPair::new(player.clone(), other_player.clone())),
            None => PairPlayerResult::Contradiction,
        };
    }

    /*This finds a player in the current set of possibilities that has the fewest
    number of possibilities left.*/
    fn highest_prob_player(&self, filtered: &HashSet<Player>) -> Option<Player> {
        let mut best_player: Option<(&Player, usize)> = None;
        for (player_key, poss_pairs) in self.possibilities.iter() {
            if filtered.contains(player_key) {
                continue;
            }

            match best_player {
                Some(best) => {
                    if poss_pairs.len() < best.1 {
                        best_player = Some((player_key, poss_pairs.len()))
                    }
                }
                None => {
                    best_player = Some((player_key, poss_pairs.len()))
                }
            }
        }

        match best_player {
            Some((player, count)) => {
                println!("Highest prob player: {}  len {}", &player, count);
                return Some(player.clone())
            },
            None => return None,
        }
    }

    fn filter_possibilities(possible_pairs: &HashSet<Player>, off_limits: &HashSet<Player>) -> HashSet<Player> {
        let difference: HashSet<&Player> = possible_pairs.difference(&off_limits).collect();
        return difference.into_iter().map(|item| item.to_owned()).collect();
    }

    /* This does an initial depth first search for a pair, and if it is 
    not possible to form a pair with the given combination, then it
    works backwards from the contraditing pair to find said pair */
    fn possible_pairing(&self) -> Vec<ContestantPair>{
        // Poss stack keeps track of players and their possibilities (which are filtered as you go down the stack)
        // Current pairing is the actual pairing 
        let mut poss_stack: Vec<(Player, HashSet<Player>)> = Vec::new();
        let mut current_pairing: Vec<ContestantPair> = Vec::new();

        if self.right_matches.len() > 0 {
            current_pairing.extend(self.right_matches.clone());
        }

        { // Start with the highest likelihood pair
            let initial_player = self.highest_prob_player(&HashSet::new()).unwrap();
            poss_stack.push((
                initial_player.clone(), 
                self.possibilities.get(&initial_player).unwrap().clone()
            ));
        }

        // keep going until you have found a full set of current_pairs
        while current_pairing.len() < self.contestants.len() / 2 {
            println!("\n\nPAIRING ATTEMPT/////////////////\n");

            // from the top of the stack, add the next stack of possibilities
            let (player_a, player_a_poss) = poss_stack.last().unwrap().clone();
            let player_a_match = player_a_poss.iter().next();
            
            if let Some(player_b) = player_a_match {
                // add to the current pairing
                poss_stack.last_mut().unwrap().1.remove(&player_b);
                current_pairing.push(ContestantPair::new(player_a.clone(), player_b.clone()));
                println!("Correct pairs: \n{} ", ContestantPairs(&current_pairing[..self.right_matches.len()].to_vec()));

                println!("Current pairing: \n{} ", ContestantPairs(&current_pairing[self.right_matches.len()..].to_vec()));

                // create the next stack of possibilities
                let off_limits: HashSet<Player> = pairs_to_contestants(&current_pairing).into_iter().collect();
                // println!("Off limits: {:?}", &off_limits.iter().map(|player| player.name.clone()).collect::<Vec<String>>());

                match self.highest_prob_player(&off_limits) {
                    // If there is a player with options left, add it to the stack
                    Some(new_player) => {
                        let mut player_possibilities = self.possibilities.get(&new_player).unwrap().clone();
                        
                        player_possibilities = BruteForce::filter_possibilities(&player_possibilities, &off_limits);
                        println!("New: {} - Poss [\n{}]", &new_player, Players(&Vec::from_iter(player_possibilities.iter())));

                        // add the new player and it's possible pairings to the stack
                        poss_stack.push((new_player, player_possibilities));
                    },

                    // Otherwise, do nothing. The next iteration of the loop will either exit
                    // bc all matches were found or will try different combinations
                    None => {
                        println!("combo found");
                    }
                };

            
            } else {
                // If there is no match, that means there are no possibilities left, means there is a contradition
                // Keep deleting stacks as long as they are empty (and the current pairing array to remove bad pairs)
                println!("Moving back up the stack......");
                // println!("stack: {:#?}", &poss_stack);

                let mut new_stack = poss_stack.last_mut().unwrap();
                while new_stack.1.len() == 0 { // new_stack.1 is the possibilities
                    // println!("stack: {:#?}", &poss_stack);
                
                    poss_stack.pop().unwrap();
                    println!("Popped pair: {:?}", current_pairing.pop());
                    // println!("new stack: {:#?}", &poss_stack);

                    new_stack = match poss_stack.last_mut(){
                        Some(item) => {
                            item
                        }
                        None => {
                            panic!("It should not be possible to have no combinations left when the game is still going");
                        }

                    }
                }
            }
        }
        return current_pairing;
    }

    fn add_round(&mut self, guess: Vec<ContestantPair>, num_correct: usize) {
        self.round_manager.add_round(guess, num_correct);
    }

    pub fn poss_left(&self) -> usize {
        let mut poss_matches_left = 0;
        for (player, poss_matches) in self.possibilities.iter() {
            poss_matches_left += poss_matches.len();
        }
        // Divide by two so you aren't double counting
        return (poss_matches_left / 2) as usize
    }

    fn add_perfect_match(&mut self, pair: ContestantPair) {
        // Remove players in pair from possibilities
        for (player, player_poss) in self.possibilities.iter_mut() {
            player_poss.remove(pair.get_a());
            player_poss.remove(pair.get_b());
        }
        // Remove player keys entirely
        self.possibilities.remove(pair.get_a());
        self.possibilities.remove(pair.get_b());

        // Add to found perfect matches
        self.right_matches.insert(pair.clone());

        // eliminate guesses from rounds
        self.round_manager.perfect_match_found(&pair);
    }

}

impl GameStrategy for BruteForce {
    fn ceremony_pairs(&mut self) -> Vec<ContestantPair> {
        return self.possible_pairing();
    }

    fn ceremony_feedback(&mut self, num_right: usize, guess: Vec<ContestantPair>) {
        let num_new_correct = num_right - self.right_matches.len(); // only care about number correct of ones we don't care about
        
        
        if num_new_correct == 0 {
            for pair in guess.iter() {
                self.remove_guess(pair);
            }
        } else {
            self.round_manager.add_round(guess, num_new_correct);
        }
        
    }

    fn send_to_booth(&mut self) -> ContestantPair {
        if self.round_manager.should_use_round(&self.possibilities) {
            println!("*******using round best guess*******");
            return self.round_manager.best_guess().unwrap();
        } else {
            println!("*******using possibilities best guess*******");

            let player = self.highest_prob_player(&HashSet::new()).unwrap();
            // println!("truth booth:  player {} possibilities --- {:#?}", &player, &self.possibilities);

            let to_pair = self.possibilities.get(&player).unwrap().iter().next().unwrap();
            return ContestantPair::new(player, to_pair.clone());
        }
    }

    fn booth_feedback(&mut self, feedback: Feedback) {
        match feedback {
            Feedback::Correct(pair) => {
                self.add_perfect_match(pair);
            },
            Feedback::Wrong(pair) => {
                self.remove_guess(&pair);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bruteforce::Feedback;
    use crate::utils::get_matches;
    use crate::utils::gen_contestants;
    use crate::utils::contestants_to_pairs;
    use std::collections::HashMap;
    use crate::bruteforce::{PairPlayerResult, BruteForce};
    use std::collections::HashSet;
    use crate::gamestrategy::GameStrategy;
    use crate::contestant::{Player, ContestantPair};
    
    #[test]
    fn test_initialization() {
        let contestants = gen_contestants(12);
        let strategy = BruteForce::initialize(contestants.iter().collect());

        assert_eq!(strategy.contestants, HashSet::from_iter(contestants.clone()));

        // Test that the right number of contestants are placed into the possibilities, and that the total number remaining is correct
        let n = contestants.len();
        assert_eq!(strategy.possibilities.keys().len(), n); //len of possibilities and number of possibilities left are not the same
        
        // Check that possibilities set for each player is right length and does not contain itself
        for (player, poss_matches) in strategy.possibilities.iter() {
            assert_eq!(poss_matches.len(), n - 1);
            assert_eq!(poss_matches.contains(player), false);
        }

        assert_eq!(strategy.poss_left(), (n * n - n) / 2);
    }


    #[test]
    fn test_already_guessed() {
        let c = gen_contestants(12);
        let mut strategy = BruteForce::initialize(c.iter().collect());
        
        assert_eq!(strategy.already_guessed(&c[0], &c[1]), false);

        // A pair is guessed if it is not contained within either player's possible guesses
        strategy.possibilities.get_mut(&c[0]).unwrap().remove(&c[1]);
        strategy.possibilities.get_mut(&c[1]).unwrap().remove(&c[0]);

        assert_eq!(strategy.already_guessed(&c[0], &c[1]), true);


        // It is also already guessed if the player key is not in the set of possible keys (and is actually a contestant)
        assert_eq!(strategy.already_guessed(&c[2], &c[3]), false);
        strategy.possibilities.remove(&c[2]);
        strategy.possibilities.remove(&c[3]);

        assert_eq!(strategy.already_guessed(&c[2], &c[3]), true);

    }

    #[test]
    fn test_remove_guess() {
        let c = gen_contestants(12);
        let mut strategy = BruteForce::initialize(c.iter().collect());

        // remove the pair from the key
        assert_eq!(strategy.already_guessed(&c[0], &c[1]), false);
        strategy.remove_guess(&ContestantPair::new(c[0].clone(), c[1].clone()));
        assert_eq!(strategy.already_guessed(&c[0], &c[1]), true);

        // TODO test should panic if you try to remove a pair with only one item left
    }

    #[test]
    fn test_pair_playerd_ok() {
        let c = gen_contestants(4);
        let mut strategy = BruteForce::initialize(c.iter().collect());

        strategy.possibilities = HashMap::from([
            (c[0].clone(), vec![              c[1].clone(), c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[1].clone(), vec![c[0].clone(),               c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[2].clone(), vec![c[0].clone(), c[1].clone(),               c[3].clone()].into_iter().collect()),
            (c[3].clone(), vec![c[0].clone(), c[1].clone(), c[2].clone(),     ].into_iter().collect()),
        ]);
        
        // Test all possible pairs are generated
        let valid_pairs = [
            (ContestantPair::new(c[0].clone(), c[1].clone()), ContestantPair::new(c[2].clone(), c[3].clone())),
            (ContestantPair::new(c[0].clone(), c[2].clone()), ContestantPair::new(c[1].clone(), c[3].clone())),
            (ContestantPair::new(c[0].clone(), c[3].clone()), ContestantPair::new(c[1].clone(), c[2].clone())),
        ];
        
        for (already_taken_pair, expected_pair) in valid_pairs.iter() {
            let off_limits: HashSet<Player> = HashSet::from_iter(vec![already_taken_pair.get_a().clone(), already_taken_pair.get_b().clone()]);
            let paired_with = strategy.pair_player(&expected_pair.get_a(), &off_limits); // This should output the other only pair possible
            
            assert_eq!(PairPlayerResult::Ok(expected_pair.clone()), paired_with);
        }
    }

    #[test]
    fn test_pair_player_contradiction() {
        let c = gen_contestants(4);
        let mut strategy = BruteForce::initialize(c.iter().collect());

        strategy.possibilities = HashMap::from([
            (c[0].clone(), vec![             c[1].clone(), c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[1].clone(), vec![c[0].clone(),              c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[2].clone(), vec![c[0].clone()  /*c[1],   c[3]*/].into_iter().collect()), // We create a contradiction by letting only c[2] be paired w/ c[0], but our off limits makes it so there's no one to pair with
            (c[3].clone(), vec![c[0].clone(),c[1].clone(), c[2].clone(),             ].into_iter().collect()),
        ]);
        
        // Only combo that works is [(c[2], c[0]), (c[1], c[3])], however we can create a contradiction
        // by disallowing c[2] from being paired with c[0]
        let off_limits =  HashSet::from_iter(vec![c[0].clone(), c[3].clone()]);
        assert_eq!(PairPlayerResult::Contradiction, strategy.pair_player(&c[2], &off_limits));

        let off_limits = HashSet::from_iter(vec![c[1].clone(), c[3].clone()]);
        let expected_pair = ContestantPair::new(c[2].clone(), c[0].clone());
        assert_eq!(PairPlayerResult::Ok(expected_pair), strategy.pair_player(&c[2], &off_limits));
    }

    #[test]
    fn test_highest_prob_player() {
        let c = gen_contestants(4);
        let mut strategy = BruteForce::initialize(c.iter().collect());

        strategy.possibilities = HashMap::from([
            (c[0].clone(), vec![              c[1].clone(), c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[1].clone(), vec![c[0].clone(),               c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[2].clone(), vec![c[0].clone()  /*c[1],   c[3]*/                        ].into_iter().collect()), // We create a contradiction by letting only c[2] be paired w/ c[0], but our off limits makes it so there's no one to pair with
            (c[3].clone(), vec![c[0].clone(), c[1].clone(), c[2].clone(),             ].into_iter().collect()),
        ]);
            
        assert_eq!(strategy.highest_prob_player(&HashSet::new()).unwrap(), c[2].clone());
    }

    #[test]
    fn test_get_no_contradiction_ceremony() {
        let c = gen_contestants(4);
        let mut strategy = BruteForce::initialize(c.iter().collect());

        strategy.possibilities = HashMap::from([
            (c[0].clone(), vec![              c[1].clone(), c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[1].clone(), vec![c[0].clone(),               c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[2].clone(), vec![c[0].clone()  /*c[1],   c[3]*/                        ].into_iter().collect()), // We create a contradiction by letting only c[2] be paired w/ c[0], but our off limits makes it so there's no one to pair with
            (c[3].clone(), vec![c[0].clone(), c[1].clone(), c[2].clone(),             ].into_iter().collect()),
        ]);
        
        // Only combo that works is [(c[2], c[0]), (c[1], c[3])], however we can create a contradiction
        // by disallowing c[2] from being paired with c[0]
        let off_limits = HashSet::from_iter(vec![c[0].clone(), c[3].clone()]);
        assert_eq!(PairPlayerResult::Contradiction, strategy.pair_player(&c[2], &off_limits));

        let expected_pair = ContestantPair::new(c[2].clone(), c[0].clone());
        assert_eq!(&expected_pair, strategy.possible_pairing().get(0).unwrap());
    }

    #[test]
    fn test_ceremony_feedback() {
        let contestants = gen_contestants(12);
        let perfect_matches = contestants_to_pairs(&contestants);

        let mut strategy = BruteForce::initialize(contestants.iter().collect());
        let guess = get_matches(&perfect_matches, 2, 4);

        // test round is created with 6 guesses left
        strategy.ceremony_feedback(2, guess.clone());
        let latest_round = strategy.round_manager.latest().unwrap();

        assert_eq!(latest_round.guesses_left(), 6);

        // test when 0 matches found
        strategy.right_matches.insert(perfect_matches.get(0).unwrap().clone());

        let guess2 = get_matches(&perfect_matches, 0, 6);
        strategy.ceremony_feedback(1, guess2.clone());

        for g in guess2.iter() {
            assert_eq!(strategy.already_guessed(g.get_a(), g.get_b()), true);
        }
    }

    #[test]
    fn test_handle_correct_match() {
        let contestants = gen_contestants(12);
        let mut strategy = BruteForce::initialize(contestants.iter().collect());
        
        let perfect_matches = contestants_to_pairs(&contestants);

        // remove players in couple from possible keys
        // also remove each player from couple as possible options for the remaining players
        let pm_match = perfect_matches.get(0).unwrap();
        assert_eq!(strategy.already_guessed(pm_match.get_a(), pm_match.get_b()), false);

        // Test when there are rounds that contain the perfect match
        strategy.add_round(get_matches(&perfect_matches, 2, 4), 2);
        strategy.add_round(get_matches(&perfect_matches, 1, 5), 1);
        
        strategy.booth_feedback(Feedback::Correct(pm_match.clone()));
        
        // Test is not a possible guess anymore
        assert_eq!(strategy.already_guessed(pm_match.get_a(), pm_match.get_b()), true);

        // Test that the round probabilities chance since it contained both
        assert_eq!(strategy.round_manager.rounds.get(0).unwrap().probability(), 1.0/5.0); // now 1/5 chance of guessing from round
        assert_eq!(strategy.round_manager.rounds.len(), 1); // round with only one option left before should be removed 
    }

    #[test]
    fn test_handle_incorrect_match() {
        let contestants = gen_contestants(12);
        let mut strategy = BruteForce::initialize(contestants.iter().collect());
        
        let matches = contestants_to_pairs(&contestants);
        let wrong_guess = matches.get(0).unwrap().clone();

        assert_eq!(strategy.already_guessed(wrong_guess.get_a(), wrong_guess.get_b()), false);

        strategy.add_round(matches.clone(), 2);
        strategy.add_round(matches.clone(), 1);

        // test when the truth booth did not have the perfect match
        strategy.booth_feedback(Feedback::Wrong(wrong_guess.clone()));
        assert_eq!(strategy.already_guessed(wrong_guess.get_a(), wrong_guess.get_b()), true);

        // Test that the round probabilities change
        assert_eq!(strategy.round_manager.rounds.get(0).unwrap().probability(), 2.0/5.0); 
        assert_eq!(strategy.round_manager.rounds.get(1).unwrap().probability(), 1.0/5.0); 
    }

    #[test]
    fn test_get_ceremony_pairs() {
        // If there are no rounds in play yet, just pick randomly
        
        // If there are are any rounds in play, take the most likely one reduce num possibilities by half
        // By including half of the remaining options in the next round 

    }

}