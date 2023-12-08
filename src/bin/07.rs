advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    // these are tuples, containing a duple of (card, bid), and a rank in the first value
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

    //TODO: implemnt comparator.
    hands.sort_by(|a, b| compare_cards(a.0, b.0));
    println!("{:?}", hands);
    return Some(1);
}

// Define a function to compare two card lists
fn compare_cards(new_cards: &str, existing_cards: &str) -> std::cmp::Ordering {
    // Implement your logic to compare the card lists
    use std::cmp::Ordering;

    // Example comparison logic
    if new_cards < existing_cards {
        Ordering::Less
    } else if new_cards > existing_cards {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    return Some(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
