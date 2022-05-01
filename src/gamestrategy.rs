use crate::contestant::{ContestantPair};

#[derive(Debug, PartialEq)]
pub enum Feedback {
    Correct(ContestantPair),
    Wrong(ContestantPair)
}

pub trait GameStrategy {
    fn send_to_booth(&mut self) -> ContestantPair;
    fn booth_feedback(&mut self, feedback: Feedback);
    fn ceremony_pairs(&mut self) -> Vec<ContestantPair>;
    fn ceremony_feedback(&mut self, num_right: usize, guess: Vec<ContestantPair>);
}