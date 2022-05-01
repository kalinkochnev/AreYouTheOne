use std::collections::HashSet;
use std::collections::HashMap;
use crate::contestant::Player;
use crate::contestant::ContestantPair;

pub fn pairs_to_contestants(pairs: &Vec<ContestantPair>) -> Vec<Player> {
    let mut players = Vec::new();
    for p in pairs.iter() {
        players.push(p.get_a().clone());
        players.push(p.get_b().clone());
    }
    return players;
}

pub fn contestants_to_pairs(contestants: &Vec<Player>) -> Vec<ContestantPair> {
    let mut pairs = Vec::new();

    for i in (0..contestants.len() - 1).step_by(2) {
        pairs.push(ContestantPair::new(contestants.get(i).unwrap().clone(), contestants.get(i + 1).unwrap().clone()));
    }
    pairs
}


pub fn get_matches(perf_matches: &Vec<ContestantPair>, num_perfect: usize, num_unperfect: usize) -> Vec<ContestantPair> {
    if num_perfect + num_unperfect > perf_matches.len() {
        panic!("{} matches can't be made from {} contestants", perf_matches.len(), num_perfect + num_unperfect);
    }
    
    // Add the perfect matches to the arr
    let mut new_matches = vec![];
    for (i, pm) in perf_matches.iter().enumerate() {
        if i == num_perfect {
            break;
        }
        new_matches.push(pm.clone());
    }

    // do not include the already perfectly matched
    let to_unmatch = pairs_to_contestants(&perf_matches[num_perfect..].to_vec());

    // Work forwards and backwards and pair those two contestants together
    for i in 0..to_unmatch.len() / 2{
        let p1 = to_unmatch.get(i).unwrap();
        let p2 = to_unmatch.get(to_unmatch.len() - 1 - i).unwrap();

        new_matches.push(ContestantPair::new(p1.clone(), p2.clone()));
    }
    return new_matches;
}

pub fn gen_contestants(num: usize) -> Vec<Player> {
    let mut players = vec![];
    for i in 0..num {
        players.push(Player::new(i as i32));
    }
    players
}

pub fn pretty_string_poss(possibilities: &HashMap<Player, HashSet<Player>>) -> String {
    let mut total_str = String::new();
    for (player, player_poss) in possibilities.iter() {
        let mut poss_string = String::new();
        for poss in player_poss.iter() {
            poss_string.push_str((poss.id.to_string() + " ,").as_str());
        }
        total_str.push_str(format!("{}: {{{}}}\n", player.id, poss_string).as_str());
    }
    return total_str;
}


#[cfg(test)]
mod tests {
    
}