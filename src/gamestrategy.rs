use crate::contestant::{ContestantPair};

#[derive(Debug, PartialEq)]
pub enum Feedback {
    Correct(ContestantPair),
    Wrong
}

pub trait GameStrategy {
    fn send_to_booth(&mut self) -> Option<ContestantPair>;
    fn booth_feedback(&mut self, feedback: Feedback);
    fn ceremony_pairs(&mut self) -> Vec<ContestantPair>;
    fn ceremony_feedback(&mut self, num_right: usize);
}