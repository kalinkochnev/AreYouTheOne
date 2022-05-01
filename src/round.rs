use crate::contestant::Player;
use crate::contestant::ContestantPair;
use log::{debug, error, trace, info};

#[derive(Debug, PartialEq, Clone)]
pub struct SavedRound {
    guesses: Vec<ContestantPair>,
    in_consideration: Vec<usize>,
    pub num_correct: usize,
    pub round_id: u32,
}

impl SavedRound {
    pub fn new(guesses: Vec<ContestantPair>, num_correct: usize, round_id: u32) -> SavedRound {
        let mut in_consideration = vec![];
        for i in 0..guesses.len() {
            in_consideration.push(i);
        }
        SavedRound {
            in_consideration,
            num_correct,
            guesses,
            round_id
        }
    }
    pub fn pick_from_round(&self, num: usize) -> Vec<ContestantPair> {
        if num > self.num_consideration() {
            panic!("Can't pick {} players because only {} have not been eliminated", num, self.num_consideration());
        }
        let mut picked_players = vec![];
        for i in self.in_consideration.iter() {
            picked_players.push(self.guesses.get(i.clone()).unwrap().clone());
        }
        return picked_players;
    }

    pub fn num_consideration(&self) -> usize {
        return self.in_consideration.len();
    }

    pub fn guesses_left(&self) -> usize {
        return self.in_consideration.len();
    }


    pub fn probability(&self) -> f32 {
        return (self.num_correct as f32) / (self.num_consideration() as f32);
    }

    pub fn eliminate_guesses(&mut self, pairs: &Vec<ContestantPair>) {
        self.in_consideration.retain(|&i| {
            let pair = self.guesses.get(i).unwrap();
            let should_keep = !pairs.contains(&pair);
            if !should_keep {
                debug!("Eliminating pair round #{}: {}", self.round_id, &pair);
            }
            return should_keep;
        });
    }

    pub fn eliminate_player(&mut self, player: &Player) {
        let mut num_removed = 0;
        let round_size = self.guesses_left();
        
        self.in_consideration.retain(|&i| {
            let pair = self.guesses.get(i).unwrap();
            let should_keep = !pair.has_player(player);
            if !should_keep {
                num_removed += 1;
                debug!("Eliminating pair: {} --- remaining in round {}", &pair, round_size - num_removed);
            }
            return should_keep;
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::ContestantPairs;
    use crate::utils::contestants_to_pairs;
    use crate::round::SavedRound;
    use crate::utils::get_matches;
    use crate::utils::gen_contestants;
    use log::{debug, error, trace, info};

    #[test]
    fn test_eliminate_pair() {
        let c = gen_contestants(12);
        let guesses = get_matches(&contestants_to_pairs(&c), 0, 6);

        // Test remove all items from round
        let mut round = SavedRound::new(guesses.clone(), 2, 1);

        assert_eq!(round.num_consideration(), 6);
        round.eliminate_guesses(&round.guesses[0..6].to_vec());
        assert_eq!(round.num_consideration(), 0);

        // Test that specific items are removed
        let new_guesses = guesses.clone();
        let mut round = SavedRound::new(new_guesses.clone(), 2, 1);

        for g in new_guesses.iter() {
            debug!("remove {}", g);
            
            assert_eq!(round.pick_from_round(round.num_consideration()).contains(g), true);
            debug!("{}", round.num_consideration());
            debug!("pre {}", &ContestantPairs(&round.pick_from_round(round.num_consideration())));
            round.eliminate_guesses(&vec![g.clone()]);
            debug!("post {}", &ContestantPairs(&round.pick_from_round(round.num_consideration())));
            assert_eq!(round.pick_from_round(round.num_consideration()).contains(g), false);
        }

    }


    #[test]
    fn test_get_highest_prob() {
        let c = gen_contestants(12);
        let guesses = get_matches(&contestants_to_pairs(&c), 0, 6);

        let mut round = SavedRound::new(guesses, 2, 1);
        assert_eq!(round.probability(), 2.0/6.0);

        // Test that probability changes when you eliminate options
        round.eliminate_guesses(&round.guesses[0..2].to_vec());
        assert_eq!(round.probability(), 2.0/4.0);
    }

    #[test]
    fn test_pick_from_round() {
        let c = gen_contestants(12);
        let guesses = get_matches(&contestants_to_pairs(&c), 0, 6);
        let copy = &guesses.to_vec();
        
        let mut round = SavedRound::new(guesses.clone(), 2, 1);
        round.eliminate_guesses(&copy[0..2].to_vec());

        assert_eq!(round.pick_from_round(4), guesses[..4])
    }

    #[test]
    fn test_eliminate_player() {
        let c = gen_contestants(12);
        let guesses = get_matches(&contestants_to_pairs(&c), 0, 6);

        let mut round = SavedRound::new(guesses.clone(), 2, 1);
        round.eliminate_player(&c[0]);
        round.eliminate_player(&c[1]);

        // there should only be 4 pairs left because c[0] and c[1] have different pairs
        assert_eq!(round.guesses.len(), 6);
        assert_eq!(round.guesses_left(), 4); 
    }
}