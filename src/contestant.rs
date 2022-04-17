use std::fmt;
use std::fs::{File};
use std::io;
use std::io::{BufRead};
use rand::seq::IteratorRandom;

#[derive(Debug, std::cmp::Eq, std::hash::Hash)]
pub struct Player {
    pub id: i32,
    pub name: String
}

impl Player {
    const NAME_FILE_PATH: &'static str = "./names.txt";

    pub fn new(id: i32) -> Player {
        return Player { id: id, name: Player::pick_name() };
    }

    fn pick_name() -> String {
        let file = File::open(Player::NAME_FILE_PATH).unwrap_or_else(|e| panic!("unable to open ./names.txt"));
        
        let buffer = io::BufReader::new(file);
        let lines = buffer.lines().map(|line| line.expect("Could not read line"));

        lines.choose(&mut rand::thread_rng()).expect("File had no lines")
    }
}
impl std::clone::Clone for Player {
    fn clone(&self) -> Self {
        return Self {id: self.id, name: self.name.clone() }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Contestant {} ({})", self.id, self.name)
    }
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}



pub struct Players<'a>(pub  &'a Vec<&'a Player>);
impl<'a> fmt::Display for Players<'a> {
    // https://medium.com/apolitical-engineering/how-do-you-impl-display-for-vec-b8dbb21d814f
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, player| {
            result.and_then(|_| {writeln!(f, "{}, ", player)})
        })
    }
}

#[derive(Debug, Clone, std::cmp::Eq, std::hash::Hash)]
pub struct ContestantPair {
    a: Player,
    b: Player,
}
impl ContestantPair {
    pub fn new(a: Player, b: Player) -> ContestantPair {
        ContestantPair {a, b}
    }
    pub fn get_a(&self) -> &Player {
        return &self.a;
    }
    pub fn get_b(&self) -> &Player {
        return &self.b;
    }

    pub fn has_player(&self, player: &Player) -> bool {
        // TODO test this
        return &self.a == player || &self.b == player;
    }
}
impl fmt::Display for ContestantPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.get_a(), self.get_b())
    }
}

impl PartialEq for ContestantPair {
    fn eq(&self, other: &Self) -> bool {
        // Check that the pair .matches regardless of contestant a or b's order
        return (self.a.id == other.a.id && self.b.id == other.b.id) || (self.a.id == other.b.id && self.b.id == other.a.id); 
    }
}
pub struct ContestantPairs<'a>(pub  &'a Vec<ContestantPair>);
impl<'a> fmt::Display for ContestantPairs<'a> {
    // https://medium.com/apolitical-engineering/how-do-you-impl-display-for-vec-b8dbb21d814f
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, pair| {
            result.and_then(|_| {writeln!(f, "{}", pair)})
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::contestant::{Player, ContestantPair};
    #[test]
    fn test_player_equal() {
        assert_eq!(Player::new(1), Player::new(1));
        assert_ne!(Player::new(1), Player::new(100));
    }

    #[test]
    fn test_clone() {
        let player = Player {id: 1, name: String::from("Janina")};
        let cloned = player.clone();

        assert_eq!(cloned.name, player.name);
        assert_eq!(cloned.id, player.id);
    }
    
    #[test]
    fn test_pair_equal() {
        let p1 = Player::new(1);
        let p2 = Player::new(2);
        let matched = ContestantPair::new(p1, p2);

        assert_eq!(ContestantPair::new(Player::new(1), Player::new(2)), matched);
        assert_eq!(ContestantPair::new(Player::new(2), Player::new(1)), matched);
        assert_ne!(ContestantPair::new(Player::new(3), Player::new(1)), matched);
    }
}
