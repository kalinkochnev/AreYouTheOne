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
    possibilities: HashMap<Player, HashSet<Player>>,
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

    fn choose_ceremony(&self) {
        todo!();

        // If we have any rounds available, choose the one that is the most likely and take half of the possibile pairs for additional checking
        let ceremony_guess = Vec::new();

        if self.round_manager.should_use_round(&self.possibilities) {
            let best_round = self.round_manager.most_eff().unwrap();
            let num_pairs = self.contestants.len() / 2;
            ceremony_guess.push(best_round.pick_from_round(num_pairs / 2));
        }

        // We need to keep choosing until we find a set of pairs that does not contradict itself
        let mut num_contradictions = 0;
        
        while ceremony_guess.len() < self.contestants.len() / 2 {
            
        }

        
    }

    fn handle_correct_match(&self, pair: &ContestantPair) {
        todo!();
    }

    fn highest_prob_rounds(&self) -> Vec<SavedRound> {
        // self.rounds.sort_by(|a, b| a.get_probability().cmp(b.get_probability()));
        todo!();
    }

    fn add_round(&mut self, guess: Vec<ContestantPair>, num_correct: usize) {
        self.round_manager.add_round(guess, num_correct);
    }

    fn poss_left(&self) -> usize {
        let mut poss_matches_left = 0;
        for (player, poss_matches) in self.possibilities.iter() {
            poss_matches_left += poss_matches.len();
        }
        // Divide by two so you aren't double counting
        return (poss_matches_left / 2) as usize
    }

}

impl GameStrategy for BruteForce {
    fn ceremony_pairs(&mut self) -> Vec<ContestantPair> {
        todo!();
    }

    fn ceremony_feedback(&mut self, num_right: usize, guess: Vec<ContestantPair>) {
        todo!();
    }

    fn send_to_booth(&mut self) -> Option<ContestantPair> {
        todo!();
    }

    fn booth_feedback(&mut self, feedback: Feedback) {
        match feedback {
            Feedback::Correct(pair) => self.handle_correct_match(&pair),
            Feedback::Wrong => {
                // update the rounds that contain this pair
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn test_get_no_contradiction_ceremony() {
        let c = gen_contestants(4);
        let mut strategy = BruteForce::initialize(c.iter().collect());

        strategy.possibilities = HashMap::from([
            (c[0].clone(), vec![      c[1].clone(), c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[1].clone(), vec![c[0].clone(),       c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[2].clone(), vec![c[0].clone()  /*c[1],   c[3]*/].into_iter().collect()), // We create a contradiction by letting only c[2] be paired w/ c[0], but our off limits makes it so there's no one to pair with
            (c[3].clone(), vec![c[0].clone(), c[1].clone(), c[2].clone(),     ].into_iter().collect()),
        ]);
        
        // Only combo that works is [(c[2], c[0]), (c[1], c[3])], however we can create a contradiction
        // by disallowing c[2] from being paired with c[0]
        let off_limits = HashSet::from_iter(vec![c[0].clone(), c[3].clone()]);
        assert_eq!(PairPlayerResult::Contradiction, strategy.pair_player(&c[2], &off_limits));

        let expected_pair = ContestantPair::new(c[2].clone(), c[0].clone());
        assert_eq!(&expected_pair, strategy.ceremony_pairs().get(0).unwrap());
    }


    #[test]
    fn test_handle_correct_match() {
        let contestants = gen_contestants(12);
        let strategy = BruteForce::initialize(contestants.iter().collect());

        // remove players in couple from possible keys
        // also remove each player from couple as possible options for the remaining players
        let (c1, c2) = (contestants.get(0).unwrap(), contestants.get(1).unwrap());

        assert_eq!(strategy.already_guessed(c1, c2), false);
        strategy.handle_correct_match(&ContestantPair::new(c1.clone(), c2.clone()));
        assert_eq!(strategy.already_guessed(c1, c2), true);

    }

    #[test]
    fn test_handle_corr_in_round() {
        todo!();
        let contestants = gen_contestants(12);
        let strategy = BruteForce::initialize(contestants.iter().collect());

        for i in 0..3 {
            // TODO
        }
    }

    

    #[test]
    fn test_ceremony_feedback() {
          todo!();
        let c = gen_contestants(12);
        let mut strategy = BruteForce::initialize(c.iter().collect());
        let guess = contestants_to_pairs(&c);

        // Test is not guessed yet
        for pair in guess.iter() {
            assert_eq!(strategy.already_guessed(pair.get_a(), pair.get_b()), false);
        }

        // If there are no correct matches, it should remove all those possible guesses
        strategy.ceremony_feedback(0, guess);

        // Should be guessed 
        for pair in guess.iter() {
            assert_eq!(strategy.already_guessed(pair.get_a(), pair.get_b()), true);
        }

        // If there is at least one match, 
    }

    #[test]
    fn test_get_ceremony_pairs() {
        // If there are no rounds in play yet, just pick randomly
        
        // If there are are any rounds in play, take the most likely one reduce num possibilities by half
        // By including half of the remaining options in the next round 

    }

}