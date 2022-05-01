use crate::contestant::ContestantPair;

#[derive(Debug, PartialEq, Clone)]
pub struct SavedRound {
    guesses: Vec<ContestantPair>,
    in_consideration: Vec<usize>,
    pub num_correct: usize
}

impl SavedRound {
    pub fn new(guesses: Vec<ContestantPair>, num_correct: usize) -> SavedRound {
        let mut in_consideration = vec![];
        for i in 0..guesses.len() {
            in_consideration.push(i);
        }
        SavedRound {
            in_consideration,
            num_correct,
            guesses
        }
    }
    pub fn pick_from_round(&self, num: usize) -> Vec<ContestantPair> {
        if num > self.num_consideration() {
            panic!("Can't pick {} players because only {} have not been eliminated", num, self.num_consideration());
        }
        let mut picked_players = vec![];
        for i in 0..self.in_consideration.len() {
            picked_players.push(self.guesses.get(i).unwrap().clone());
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
        self.in_consideration.retain(|&i| !pairs.contains(self.guesses.get(i).unwrap()));
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::contestants_to_pairs;
    use crate::round::SavedRound;
    use crate::utils::get_matches;
    use crate::utils::gen_contestants;

    #[test]
    fn test_eliminate_pair() {
        let c = gen_contestants(12);
        let guesses = get_matches(&contestants_to_pairs(&c), 0, 6);

        let mut round = SavedRound::new(guesses, 2);
        assert_eq!(round.num_consideration(), 6);

        round.eliminate_guesses(&round.guesses[0..6].to_vec());

        assert_eq!(round.num_consideration(), 0);
    }


    #[test]
    fn test_get_highest_prob() {
        let c = gen_contestants(12);
        let guesses = get_matches(&contestants_to_pairs(&c), 0, 6);

        let mut round = SavedRound::new(guesses, 2);
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
        
        let mut round = SavedRound::new(guesses.clone(), 2);
        round.eliminate_guesses(&copy[0..2].to_vec());

        assert_eq!(round.pick_from_round(4), guesses[..4])
    }
}