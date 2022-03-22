use crate::contestant::{Player, ContestantPair, Players};
use rand::seq::SliceRandom;
use rand::thread_rng;


pub struct GameMaster {
    iterations: i32,
    pairs: Vec<ContestantPair>,
}

impl GameMaster {
    pub fn initialize_game(num_contestants: i32) -> GameMaster {
        // Create n number of contestants, randomly pair off into perfect matches
        let mut pairs: Vec<ContestantPair> = vec![];

        // Each contestant is assigned an ID
        for i in (0..num_contestants - 1).step_by(2) {
            pairs.push(ContestantPair::new(Player::new(i), Player::new(i+1)));
        }

        return GameMaster {iterations: 0, pairs};
    }
    pub fn contestants(&self) -> Vec<&Player> {
        let mut players: Vec<&Player> = vec![];

        for pair in self.pairs.iter() {
            players.push(&pair.getA());
            players.push(&pair.getB());
        }
        let mut rng = thread_rng();
        players.shuffle(&mut rng);
        return players;
    }

    fn isPerfectMatch(&self, other_pair: &ContestantPair) -> bool {
        for matched_pair in self.pairs.iter() {
            if matched_pair == other_pair {
                return true;
            }
        }
        return false;
    }

}


#[cfg(test)]
mod tests {
    use crate::gamemaster::GameMaster;
    use crate::contestant::{Player, ContestantPair, ContestantPairs};


    #[test]
    fn test_game_initialized() {
        let game = GameMaster::initialize_game(12);
        assert_eq!(game.pairs.len(), 6);
        assert_eq!(game.contestants().len(), 12);
    }

    #[test]
    fn test_get_contestants() {
        let num_players = 12;
        let game = GameMaster::initialize_game(num_players);
        
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
        for i in 0..game.pairs.len() {
            assert_eq!(game.pairs[i].getA().id as usize, 2*i);
            assert_eq!(game.pairs[i].getB().id as usize, 2*i+1);
        }
    }
    #[test]
    fn test_is_perfect_match() {
        let p1 = Player::new(1);
        let p2 = Player::new(2);
        let matched = ContestantPair::new(p1, p2);
        let game_master = GameMaster {iterations: 0, pairs: vec![matched]};

        assert_eq!(game_master.isPerfectMatch(&ContestantPair::new(Player::new(1), Player::new(2))), true);
        assert_eq!(game_master.isPerfectMatch(&ContestantPair::new(Player::new(2), Player::new(1))), true);
        assert_eq!(game_master.isPerfectMatch(&ContestantPair::new(Player::new(3), Player::new(1))), false);
    }

}
