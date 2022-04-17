use crate::gamestrategy::Feedback;
use crate::contestant::{ContestantPair, Player};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct GameMaster {
    iterations: i32,
    num_matched: i32,
    matches: Vec<ContestantPair>,
    max_iterations: i32,
}

impl GameMaster {
    pub fn initialize_game(num_contestants: i32, max_iterations: i32) -> GameMaster {
        // Create n number of contestants, randomly pair off into perfect .matches
        let mut pairs: Vec<ContestantPair> = vec![];

        // Each contestant is assigned an ID
        for i in (0..num_contestants - 1).step_by(2) {
            pairs.push(ContestantPair::new(Player::new(i), Player::new(i + 1)));
        }

        return GameMaster {
            iterations: 0,
            matches: pairs,
            num_matched: 0,
            max_iterations
        };
    }
    pub fn contestants(&self) -> Vec<&Player> {
        let mut players: Vec<&Player> = vec![];

        for pair in self.matches.iter() {
            players.push(&pair.get_a());
            players.push(&pair.get_b());
        }
        let mut rng = thread_rng();
        players.shuffle(&mut rng);
        return players;
    }

    fn is_match(&self, pair: &ContestantPair) -> bool {
        for matched_pair in self.matches.iter() {
            if matched_pair == pair {
                return true;
            }
        }
        return false;
    }

    pub fn truth_booth(&mut self, guess: Option<ContestantPair>) -> Feedback {
        println!("ROUND {}-------", self.iterations);
        if let Some(pair) = guess {
            println!("Attempted match {}", pair);

            self.iterations += 1;
            if self.is_match(&pair) {
                println!("Guessed correctly!\n");
                self.num_matched += 1;
                return Feedback::Correct(pair.clone());
            } else {
                println!("Wrong guess!\n");
                return Feedback::Wrong;
            }
            
        } else {
            println!("Abstained from the truth booth!");
            return Feedback::Wrong;
        }

    }

    pub fn ceremony(&self, pairs: Vec<ContestantPair>) -> usize {
        let mut num_matches = 0;
        for p in pairs.iter() {
            if self.is_match(p) {
                num_matches += 1;
            }
        }
        println!("{} perfect matches are contained!", num_matches);
        return num_matches;
    }

    pub fn output_stats(&self) {
        println!("{} perfect matches were found successfully after {} iterations",
            self.contestants().len(),
            self.iterations
        );
    }

    pub fn get_iterations(&self) -> i32 {
        return self.iterations;
    }

    pub fn in_progress(&self) -> bool {
        return (self.iterations < self.max_iterations) && (self.num_matched != i32::try_from(self.matches.len()).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::gamestrategy::Feedback;
        use crate::contestant::{ContestantPair, Player};
    use crate::gamemaster::GameMaster;

    #[test]
    fn test_game_initialized() {
        let game = GameMaster::initialize_game(12, 10);
        assert_eq!(game.matches.len(), 6);
        assert_eq!(game.contestants().len(), 12);
    }

    #[test]
    fn test_get_contestants() {
        let num_players = 12;
        let game = GameMaster::initialize_game(num_players, 10);
        // Check that the randomized contestant list is not the same as the one derived from matched pair order
        let randomized: Vec<&Player> = game.contestants();
        let mut num_equal = 0;
        for i in 0..randomized.len() {
            if randomized[i].id as usize == i {
                num_equal += 1;
            }
        }
        assert_ne!(num_equal, randomized.len());

        // Check that the matched pairs are still in order after shuffles
        for i in 0..game.matches.len() {
            assert_eq!(game.matches[i].get_a().id as usize, 2 * i);
            assert_eq!(game.matches[i].get_b().id as usize, 2 * i + 1);
        }
    }

    #[test]
    fn test_is_perfect_match() {
        let p1 = Player::new(1);
        let p2 = Player::new(2);
        let matched = ContestantPair::new(p1, p2);
        let game_master = GameMaster {
            iterations: 0,
            matches: vec![matched],
            num_matched: 0,
            max_iterations: 10
        };

        assert_eq!(
            game_master.is_match(&ContestantPair::new(Player::new(1), Player::new(2))),
            true
        );
        assert_eq!(
            game_master.is_match(&ContestantPair::new(Player::new(2), Player::new(1))),
            true
        );
        assert_eq!(
            game_master.is_match(&ContestantPair::new(Player::new(3), Player::new(1))),
            false
        );
    }

    #[test]
    fn test_in_progress_found_matches() {
        let mut game = GameMaster::initialize_game(12, 50);
        assert_eq!(game.in_progress(), true);
        for pair in game.matches.clone().iter() {
            assert_eq!(game.truth_booth(Some(pair.clone())), Feedback::Correct(pair.clone()));
        }
        assert_eq!(game.in_progress(), false);
    }

    #[test]
    fn test_in_progress_exceeds_limit() {
        let mut game = GameMaster::initialize_game(12, 5);
        assert_eq!(game.in_progress(), true);

        let cloned = game.matches.to_owned();
        for i in 0..6 {
            assert_eq!(game.truth_booth(Some(cloned.get(0).unwrap().clone())), Feedback::Correct(cloned.get(0).unwrap().clone()));
        }
        
        assert_eq!(game.in_progress(), false);
    }
}
