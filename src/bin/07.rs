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
        println!("ordering: {:?}", result);
        return result;
    } else {
        println!("cards are equal");

        Ordering::Equal
    }
}

fn count_most_dupes(string: &str) -> (Option<char>, u32) {
    let mut count_map = HashMap::new();

    for c in string.chars() {
        *count_map.entry(c).or_insert(0) += 1;
    }

    let mut dupes = 0;
    let mut duped: Option<char> = None; // Use Option type to allow nil character
    for (c, amount) in count_map {
        if amount > dupes {
            dupes = amount;
            duped = Some(c);
        }
    }

    return (duped, dupes);
}

fn rank(string: &str) -> u32 {
    let mut rank;
    let (c, dupes_amount) = count_most_dupes(string);

    rank = dupes_amount;

    println!("string: {:?} duped char: {:?} ", string, c);

    if rank == 0 {
        println!("high card {:?} ", string);
    }
    if rank == 1 {
        println!("one pair {:?} ", string);
    }
    if rank == 2 {
        println!("two pair {:?} ", string);
    }

    if dupes_amount == 3 {
        let new_str = string.replace(c.unwrap(), "");
        println!("new str after extracting dupes {:?} ", new_str);

        if count_most_dupes(&new_str).1 == 2 {
            rank = 4;
            println!("full house {:?} ", string);
        } else {
            println!("three of a kind {:?} ", string);
            rank = 3
        }
    }

    if dupes_amount == 4 {
        rank = 5;
        println!("four of a kind {:?} ", string);
    }
    if dupes_amount == 5 {
        rank = 6;
        println!("five of a kind{:?} ", string);
    }
    return rank;
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
    println!("comparing strings:");
    println!("cards 1: {:?}, cards 2 {:?}", n_cards, e_cards);

    for (i, char) in n_chars.iter().enumerate() {
        let val1 = convert_char(char.to_owned());
        let val2 = convert_char(e_chars[i]);

        println!("val1: {}, val2: {}", val1, val2);

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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
