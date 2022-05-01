use crate::ContestantPairs;
use crate::contestant::ContestantPair;
use std::collections::HashSet;
use crate::contestant::Player;
use std::collections::HashMap;
use crate::round::SavedRound;
use log::{debug, error, trace, info};

pub struct RoundManager {
    pub rounds: Vec<SavedRound>,
    pub times_round_used: usize,
    last_round_id: u32,
}

impl RoundManager {
    pub fn new() -> RoundManager {
        return RoundManager{
            rounds: Vec::new(),
            times_round_used: 0,
            last_round_id: 0,
        }
    }

    /* This retrieves the round with the highest probability of finding an item*/
    pub fn most_eff(&self) -> Option<&SavedRound> {
        if self.rounds.len() > 0 {
            let mut best_round = self.rounds.get(0).unwrap();

            for r in self.rounds[1..].iter() {
                if r.probability() > best_round.probability() {
                    best_round = r;
                }
            }
            return Some(best_round);
        }
        return None
    }

    /* We should use a round if it has a higher chance of finding a pair
    than just eliminating possibilities*/
    pub fn should_use_round(&mut self, possibilities: &HashMap<Player, HashSet<Player>>) -> bool {
        let best_round = match self.most_eff() {
            None => return false,
            Some(round) => { round },
        };

        // find the player key with the least number of poss pairs left
        let mut best_player: Option<(&Player, usize)> = None;
        for (player, poss_pairs) in possibilities.iter() {
            match best_player {
                Some(best) => {
                    if poss_pairs.len() < best.1 {
                        best_player = Some((player, poss_pairs.len()))
                    }
                }
                None => {
                    best_player = Some((player, poss_pairs.len()))
                }
            }
        }


        match best_player {
            Some(best) => {
                debug!("~~~~~~Round decider~~~~~~~~~\nRound: {} \nBest round guess: {} prob {} remaining {} num corr {}\n Best brute guess: {} prob {} remaining {}",
                    &ContestantPairs(&best_round.pick_from_round(best_round.num_consideration())),
                    best_round.pick_from_round(1).pop().unwrap(),
                    best_round.probability(),
                    best_round.num_consideration(),
                    best_round.num_correct,
                    best.0,
                    1.0 / best.1 as f32,
                    best.1
                );
                // TODO pick the contestant pair that has the best_player
                let best_player_prob = 1.0 / best.1 as f32;                
                return best_round.probability() > best_player_prob;
            },
            None => panic!("There should be possibilities still left in the game!")
        }
    }

    pub fn add_round(&mut self, guesses: Vec<ContestantPair>, num_correct: usize) {
        self.last_round_id += 1;
        self.rounds.push(SavedRound::new(guesses, num_correct, self.last_round_id));
    }

    pub fn best_guess(&mut self) -> Option<ContestantPair> {
        match self.most_eff() {
            Some(round) => {
                let mut player = round.pick_from_round(1);
                self.times_round_used += 1;
                return player.pop()
            },
            None => return None
        };
    }

    pub fn latest(&self) -> Option<&SavedRound> {
        return self.rounds.last();
    }
    pub fn latest_mut(&mut self) -> Option<&mut SavedRound> {
        return self.rounds.last_mut();
    }
    pub fn contains(&self, round: &SavedRound) -> bool {
        return self.rounds.contains(round);
    }

    pub fn perfect_match_found(&mut self, pair: &ContestantPair) {
        // Go through each round, see if it contains the pair. If it does,
        // eliminate guess and decrease number of perfect match
        for round in self.rounds.iter_mut() {

            if round.num_correct > 0 {
                round.num_correct -= 1;
            }
            round.eliminate_player(pair.get_a());
            round.eliminate_player(pair.get_b());
        }
    }

    pub fn eliminate_guesses(&mut self, guesses: Vec<ContestantPair>) {

        for round in self.rounds.iter_mut() {
            round.eliminate_guesses(&guesses);
        }
    }

    pub fn pretty_string(&self) -> String {
        let mut round_str = String::new();
        for r in self.rounds.iter() {
            round_str.push_str(format!("saved round #{} -- \n{}\n", r.round_id, ContestantPairs(&r.pick_from_round(r.num_consideration()))).as_str())
        }
        return round_str;
    }

    /* Returns how many rounds were pruned */
    pub fn prune_rounds(&mut self) -> usize {
        let num_saved = self.rounds.len();
        // eliminate any rounds that are not pruned
        self.rounds.retain(|round| round.num_consideration() > 0 && round.num_correct != 0);
        debug!("{} rounds pruned", num_saved - self.rounds.len());
        return num_saved - self.rounds.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::contestants_to_pairs;
    use crate::roundmanager::RoundManager;
    use crate::roundmanager::Player;
    use std::collections::HashSet;
    use std::collections::HashMap;
    use crate::utils::gen_contestants;

    #[test]
    fn test_most_eff_round() {
        let c = gen_contestants(10);
        let mut round_manager = RoundManager::new();

        round_manager.add_round(contestants_to_pairs(&c), 1); // round should have a 20% chance at a guess currently while possibilities only 33%
        assert_eq!(round_manager.most_eff(), round_manager.latest());
        
        round_manager.add_round(contestants_to_pairs(&c), 3); // round should have a 60% chance at a guess currently while possibilities only 33%
        let most_eff = round_manager.latest().cloned().unwrap();

        round_manager.add_round(contestants_to_pairs(&c), 2); // round should have a 40% chance at a guess currently while possibilities only 33%

        assert_eq!(round_manager.most_eff().unwrap(), &most_eff);
    }

    #[test]
    fn test_should_use_round() {
        let c = gen_contestants(4);
        let mut round_manager = RoundManager::new();
        // A round should only be used if it has a higher chance of finding a pair than just eliminating
        let mut possibilities: HashMap<Player, HashSet<Player>> = HashMap::from([
            (c[0].clone(), vec![              c[1].clone(), c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[1].clone(), vec![c[0].clone(),               c[2].clone(), c[3].clone()].into_iter().collect()),
            (c[2].clone(), vec![c[0].clone(), c[1].clone(),               c[3].clone()].into_iter().collect()),
            (c[3].clone(), vec![c[0].clone(), c[1].clone(), c[2].clone(),     ].into_iter().collect()),
        ]);

        round_manager.add_round(contestants_to_pairs(&c), 1); // round should have a 50% chance at a guess currently while possibilities only 33%
        assert_eq!(round_manager.should_use_round(&possibilities), true);

        let c0_poss = possibilities.get_mut(&c[0]).unwrap();
        c0_poss.clear();
        c0_poss.insert(c[1].clone()); // this should make a 100% chance for straight guessing from possibilities
        assert_eq!(round_manager.should_use_round(&possibilities), false);

    }

    #[test]
    fn test_prune_rounds() {
        let c = gen_contestants(10);
        let mut round_manager = RoundManager::new();
        round_manager.add_round(contestants_to_pairs(&c), 0);
        round_manager.add_round(contestants_to_pairs(&c), 0);
        round_manager.add_round(contestants_to_pairs(&c), 0);
        assert_eq!(round_manager.rounds.len(), 3);
        
        assert_eq!(round_manager.prune_rounds(), 3);
        assert_eq!(round_manager.rounds.len(), 0);

        let guess = contestants_to_pairs(&c);
        round_manager.add_round(guess.clone(), 0);
        let mut latest = round_manager.rounds.last_mut().unwrap();
        latest.eliminate_guesses(&guess);
        assert_eq!(round_manager.prune_rounds(), 1);
        assert_eq!(round_manager.rounds.len(), 0);
    }
}