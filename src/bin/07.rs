advent_of_code::solution!(7);
use std::{cmp::Ordering, collections::HashMap};

// this function receives the entire input from the file
// and is supposed to return only the result
pub fn part_one(input: &str) -> Option<u32> {
    // these are tuples, containing a duple of (card, bid) in the first value
    let mut hands: Vec<(&str, u32)> = Vec::new();

    for line in input.lines() {
        let [cards, bid]: [&str; 2] = line
            .split_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let bid_val = bid.parse::<u32>().unwrap();
        hands.push((cards, bid_val))
    }

    hands.sort_by(|a, b| compare_cards(a.0, b.0));
    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(index, item)| item.1 * (index as u32 + 1))
        .collect::<Vec<_>>();

    let sum: u32 = total_winnings.iter().sum();

    return Some(sum);
}

fn compare_cards(new_cards: &str, existing_cards: &str) -> std::cmp::Ordering {
    let dupes_nc = rank(new_cards);
    let dupes_ec = rank(existing_cards);

    // Example comparison logic
    if dupes_nc < dupes_ec {
        Ordering::Less
    } else if dupes_nc > dupes_ec {
        Ordering::Greater
    } else if dupes_nc == dupes_ec {
        let result = untie(new_cards, existing_cards);
        return result;
    } else {
        Ordering::Equal
    }
}

fn rank(string: &str) -> u32 {
    let mut count_map = HashMap::new();

    for c in string.chars() {
        *count_map.entry(c).or_insert(0) += 1;
    }

    if count_map.values().any(|&val| val == 5) {
        return 7;
    };
    if count_map.values().any(|&val| val == 4) {
        return 6;
    };
    if count_map.values().any(|&val| val == 3) && count_map.values().any(|&val| val == 2) {
        return 5;
    };
    if count_map.values().any(|&val| val == 3) && count_map.values().any(|&val| val != 2) {
        return 4;
    };
    if count_map.values().filter(|&&val| val == 2).count() > 1 {
        return 3;
    };
    if count_map.values().filter(|&&val| val == 2).count() == 1 {
        return 2;
    };
    return 1;
}

fn convert_char(c: char) -> u32 {
    if c == 'K' {
        return 'A' as u32 - 1;
    }
    if c == 'Q' {
        return 'A' as u32 - 2;
    }
    if c == 'J' {
        return 'A' as u32 - 3;
    }
    if c == 'T' {
        return 'A' as u32 - 4;
    }
    c as u32
}

fn untie(n_cards: &str, e_cards: &str) -> Ordering {
    // extract ascII characters
    let n_chars: Vec<char> = n_cards.chars().collect();
    let e_chars: Vec<char> = e_cards.chars().collect();

    for (i, char) in n_chars.iter().enumerate() {
        let val1 = convert_char(char.to_owned());
        let val2 = convert_char(e_chars[i]);

        if val1 > val2 {
            return Ordering::Greater;
        }
        if val1 < val2 {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    //my solution so far: 245578416
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(245794640));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
