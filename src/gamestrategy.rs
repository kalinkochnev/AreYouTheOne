use crate::contestant::{Player, ContestantPair};


pub trait GameStrategy {
    fn guess(&self) -> ContestantPair;
    fn handle_feedback(&self, guessed_right: bool);
}

pub struct SimpleStrategy {
    contestants: Vec<Player>
}
impl SimpleStrategy {
    pub fn initialize(contestants: Vec<&Player>) -> SimpleStrategy {
        let mut cloned_contestants: Vec<Player> = vec![];
        
        for player in contestants.iter() {
            cloned_contestants.push((*player).clone());
        }

        SimpleStrategy { contestants: cloned_contestants }
    }

}
impl GameStrategy for SimpleStrategy {

    fn guess(&self) -> ContestantPair {
        let player_a = self.contestants.get(0).unwrap();
        let player_b = self.contestants.get(1).unwrap();
        
        ContestantPair::new(player_a.clone(), player_b.clone())
    }

    fn handle_feedback(&self, guessed_right: bool) {
        
    }
}