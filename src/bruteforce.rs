use std::collections::HashSet;
use std::collections::HashMap;
use crate::contestant::{Player, ContestantPair};
use std::iter::Iterator;
use crate::gamestrategy::{GameStrategy, Feedback};

pub struct BruteForce {
    contestants: HashSet<Player>,
    right_matches: HashSet<ContestantPair>,
    unmatched_pairs: HashMap<Player, HashSet<Player>>,
    unique_ceremony_guesses: Vec<ContestantPair>,
    booth_queue: Vec<ContestantPair>
}

impl BruteForce {
    /* 
    This algorithm attempts to match people who have never been matched before until
    there is at least 1 success. It then keeps going to the truth booth until that 
    the perfect match is identified. It then fixes those matches in place and attempts
    to keep finding people who have not been found. 

    unique_ceremony_guesses: a list of pairs that we are interested in checking during
        the ceremony and don't know if they are correct

    booth_queue: a list of pairs we are interested in testing in the truth booth
    */

    pub fn initialize(contestants: Vec<&Player>) -> Self {
        let cloned_contestants: HashSet<Player> = HashSet::from_iter(contestants.into_iter().cloned());
        let mut unmatched_pairs = HashMap::new(); // We initialize 

        // Initializes which contestants haven't been checked yet
        // Remove player from key-value pair so that they can't be 
        // matched with themselves
        for player in cloned_contestants.iter() {
            let mut unpaired_contestants = HashSet::from_iter(cloned_contestants.clone());
            unpaired_contestants.remove(player);

            unmatched_pairs.insert(player.clone(), unpaired_contestants);
        }

        Self { 
            contestants: cloned_contestants, 
            unmatched_pairs,
            right_matches: HashSet::new(),
            unique_ceremony_guesses: vec![],
            booth_queue: vec![]
        }
    }

    fn already_guessed(&self, p1: &Player, p2: &Player) -> bool {
        match self.unmatched_pairs.get(p1) {
            Some(players_to_match) => return !players_to_match.contains(p2),
            None => panic!("p1 given is not an existing contestant")
        }
    }

    // fn pair_player(&self, player: &Player, off_limits: HashSet<Player>) -> ContestantPair {
    //     let unmatched_partners = self.unmatched_pairs.get(player).unwrap();
    //     let partner = unmatched_partners.iter().next().unwrap();

    //     ContestantPair::new(
    //         player.clone(),
    //         partner.clone()
    //     )
    // }

    fn correct_contestants(&self) -> HashSet<&Player> {
        let mut correct_contestants = HashSet::new();
        for p in self.right_matches.iter() {
            correct_contestants.insert(p.get_a());
            correct_contestants.insert(p.get_b());
        }
        return correct_contestants;
    }

    fn get_unmatched_pairs(&mut self) -> Vec<ContestantPair> {
        // let mut in_match_process: HashSet<&Player> = HashSet::new();
        let mut newly_matched: Vec<ContestantPair> = vec![];
        let off_limits = HashSet::new();

        // If a player is in the guess map, then that means they haven't been matched yet
        for player in self.unmatched_pairs.keys() {
            let poss_partners = self.unmatched_pairs.get(player).unwrap();
            let unmatched: HashSet<&Player> = poss_partners.difference(&off_limits).collect();

            // If there are no players that can be matched, the for loop skips that (since it breaks immediately)
            for partner in unmatched.into_iter() {
                let new_pair = ContestantPair::new(player.clone(), partner.clone());
                newly_matched.push(new_pair);
            }

        }

        // Remove the new guesses from the hashmap
        for new_match in newly_matched.iter() {
            self.remove_guess(&new_match);
        }
        newly_matched
    }

    fn remove_guess(&mut self, pair: &ContestantPair) {
        // We remove each couple from the guessing map (so they aren't included again)
        if let Some(players_not_matched) = self.unmatched_pairs.get_mut(pair.get_a()) {
            players_not_matched.remove(pair.get_b());
        };

        if let Some(players_not_matched) = self.unmatched_pairs.get_mut(pair.get_b()) {
            players_not_matched.remove(pair.get_a());
        };
    }

    fn handle_correct_match(&mut self, pair: ContestantPair) {
        self.unmatched_pairs.remove(pair.get_a());
        self.unmatched_pairs.remove(pair.get_b());

        // Filter the booth queue for any entries that include either player in the pair
        self.booth_queue = self.booth_queue.drain(..).filter(|guess_pair| 
            guess_pair.has_player(pair.get_a()) || guess_pair.has_player(pair.get_b())
        ).collect();

        // Add to right matches
        self.right_matches.insert(pair);
    }
}

impl GameStrategy for BruteForce {
    fn ceremony_pairs(&mut self) -> Vec<ContestantPair> {
        let mut ceremony_guess = self.get_unmatched_pairs();
        self.unique_ceremony_guesses = ceremony_guess.clone(); 

        // Add the already matched players to the guess 
        for matched in self.right_matches.clone().iter() {
            ceremony_guess.push(matched.clone());
        }
        // This should include already matched 
        ceremony_guess
    }

    fn ceremony_feedback(&mut self, num_right: usize) {
        // If we find multiple (or one) new potential matches compared to our last iteration
        if (num_right - self.right_matches.len()) >= 0 {
            // queue up every pair that hasn't already been guessed for the truth booth
            for pair in self.unique_ceremony_guesses.iter() {
                self.booth_queue.push(pair.clone());
            }
        }
    }

    fn send_to_booth(&mut self) -> Option<ContestantPair> {
        if self.booth_queue.len() == 0 {
            return None
        }
        return self.booth_queue.pop();
    }

    fn booth_feedback(&mut self, feedback: Feedback) {
        println!("{}", self.booth_queue.len());
        match feedback {
            Feedback::Correct(pair) => self.handle_correct_match(pair),
            Feedback::Wrong => {
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bruteforce::ContestantPair;
use std::collections::HashSet;
use crate::BruteForce;
    use crate::contestant::{Player};
    
    fn gen_contestants(num: usize) -> Vec<Player> {
        let mut players = vec![];
        for i in 0..num {
            players.push(Player::new(i as i32));
        }
        players
    }

    #[test]

    fn test_initialization() {
        let contestants = gen_contestants(12);
        let strategy = BruteForce::initialize(contestants.iter().collect());

        assert_eq!(strategy.contestants, HashSet::from_iter(contestants.clone()));
        assert_eq!(strategy.unmatched_pairs.len(), contestants.len());

        let player_to_check = contestants.get(0).unwrap();
        let not_paired_yet = strategy.unmatched_pairs.get(player_to_check).unwrap();
        assert_eq!(not_paired_yet.contains(player_to_check), false);
        assert_eq!(not_paired_yet.len(), contestants.len() - 1, "not paired array should be one less than number of contestants");
    }

    #[test]
    fn test_already_guessed() {
        let contestants = gen_contestants(12);
        let mut strategy = BruteForce::initialize(contestants.iter().collect());
        let player_to_check = contestants.get(0).unwrap();
        let other_player = contestants.get(1).unwrap();
        
        let unmatched = strategy.unmatched_pairs.clone();
        let not_paired_yet = unmatched.get(player_to_check).unwrap();
        
        assert_eq!(
            strategy.already_guessed(&player_to_check, not_paired_yet.get(other_player).unwrap()),
            false   
        );
        
        // Remove player from not yet guessed set
        strategy.remove_guess(&ContestantPair::new(player_to_check.clone(), other_player.clone()));
        assert_eq!(
            strategy.already_guessed(&player_to_check, not_paired_yet.get(other_player).unwrap()),
            true   
        );
    }
}