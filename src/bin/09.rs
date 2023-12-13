advent_of_code::solution!(9);
use std::{collections::VecDeque, i32};

use regex::Regex;

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"\w+").unwrap();
    let mut histories = Vec::new();

    for line in input.lines() {
        let nums = re
            .find_iter(line)
            .map(|item: regex::Match<'_>| item.as_str().parse::<i32>().unwrap())
            .collect();
        histories.push(nums);
    }

    return histories;
}

pub fn part_one(input: &str) -> Option<i32> {
    let histories = parse(input);

    for history in histories.iter() {
        let mut diffs: VecDeque<i32> = VecDeque::new();
        diffs.extend(history);
        let mut last_list = VecDeque::new();
        last_list.push_front(diffs[diffs.len() - 1]);
        let mut aux = diffs;
        'outer: loop {
            println!("aux list, {:?}", aux);

            let mut next_aux = VecDeque::new();
            let mut diff_val = 0;
            let last_val = aux[aux.len() - 1];
            while aux.len() != 0 {
                let (i1, i2) = (aux.pop_front().unwrap(), aux.pop_front().unwrap());
                diff_val = i2 - i1;
                next_aux.push_front(diff_val);
            }

            // check if all values in the array are zero. If so, break the loop, use that
            if next_aux.iter().all(|&x| x == 0) {
                last_list.push_front(diff_val);
                break 'outer;
            }

            last_list.push_front(last_val);

            // continue until the list gets to 0
            aux = next_aux;
        }
        println!("last list, {:?}", last_list);
    }

    print!("{:?}", histories);
    None
}

pub fn part_two(input: &str) -> Option<i32> {
    None
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
