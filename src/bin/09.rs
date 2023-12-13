advent_of_code::solution!(9);
use std::{collections::VecDeque, i32};

use regex::Regex;

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let re = Regex::new(r"\w+|-\w+").unwrap();
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

    let mut acc = 0;
    for history in histories.iter() {
        let mut diffs: VecDeque<i32> = VecDeque::new();
        diffs.extend(history);
        let mut last_list = VecDeque::new();
        let mut aux = diffs;
        'outer: loop {
            last_list.push_front(aux[aux.len() - 1]);

            let mut next_aux = VecDeque::new();

            for i in 1..aux.len() {
                let (i1, i2) = (aux[i - 1], aux[i]);
                next_aux.push_back(i2 - i1);
            }

            // check if all values in the array are zero. If so, break the loop, use that
            if next_aux.iter().all(|&x| x == 0) {
                break 'outer;
            }

            // continue until the list gets to 0
            aux = next_aux;
        }

        acc += last_list.iter().sum::<i32>();
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<i32> {
    let histories = parse(input);

    let mut acc = 0;
    for history in histories.iter() {
        let mut diffs: VecDeque<i32> = VecDeque::new();
        let reversed: Vec<i32> = history.to_owned().into_iter().rev().collect();
        diffs.extend(reversed);
        let mut last_list = VecDeque::new();
        let mut aux = diffs;
        'outer: loop {
            last_list.push_front(aux[aux.len() - 1]);
            let mut next_aux = VecDeque::new();

            for i in 1..aux.len() {
                let (i1, i2) = (aux[i - 1], aux[i]);
                next_aux.push_back(i2 - i1);
            }

            // check if all values in the array are zero. If so, break the loop, use that
            if next_aux.iter().all(|&x| x == 0) {
                break 'outer;
            }

            // continue until the list gets to 0
            aux = next_aux;
        }

        acc += last_list.iter().sum::<i32>();
    }

    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result: Option<i32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
