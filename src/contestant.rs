use std::fmt;

#[derive(Debug)]
pub struct Player {
    pub id: i32
}

impl Player {
    pub fn new(id: i32) -> Player {
        return Player { id: id };
    }
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Player({})", self.id);
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

#[derive(Debug)]
pub struct ContestantPair {
    A: Player,
    B: Player,
}
impl ContestantPair {
    pub fn new(A: Player, B: Player) -> ContestantPair {
        ContestantPair {A, B}
    }
    pub fn getA(&self) -> &Player {
        return &self.A;
    }
    pub fn getB(&self) -> &Player {
        return &self.B;
    }
}
impl fmt::Display for ContestantPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.getA(), self.getB())
    }
}

impl PartialEq for ContestantPair {
    fn eq(&self, other: &Self) -> bool {
        // Check that the pair matches regardless of contestant A or B's order
        return (self.A.id == other.A.id && self.B.id == other.B.id) || (self.A.id == other.B.id && self.B.id == other.A.id); 
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
    fn test_pair_equal() {
        let p1 = Player::new(1);
        let p2 = Player::new(2);
        let matched = ContestantPair::new(p1, p2);

        assert_eq!(ContestantPair::new(Player::new(1), Player::new(2)), matched);
        assert_eq!(ContestantPair::new(Player::new(2), Player::new(1)), matched);
        assert_ne!(ContestantPair::new(Player::new(3), Player::new(1)), matched);
    }
}
