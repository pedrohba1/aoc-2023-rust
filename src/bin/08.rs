use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
struct Node {
    origin: String,
    l: String,
    r: String,
}

struct Map {
    dirs: Vec<char>,
    nodes: HashMap<String, Node>,
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

impl Map {
    pub fn new() -> Self {
        Map {
            dirs: Vec::new(),
            nodes: HashMap::new(),
        }
    }
    pub fn parse(&mut self, input: &str) {
        let re = Regex::new(r"\w+").unwrap();

        for (i, line) in input.lines().enumerate() {
            if i == 0 {
                for c in line.chars() {
                    self.dirs.push(c);
                }
                continue;
            }
            let m: Vec<_> = re
                .find_iter(line)
                .map(|m: regex::Match<'_>| m.as_str())
                .collect();
            if m.len() > 0 {
                let n = Node {
                    origin: m[0].to_string(),
                    l: m[1].to_string(),
                    r: m[2].to_string(),
                };

                self.nodes.insert(m[0].to_string(), n);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut moves = 0;
    let mut map = Map::new();
    map.parse(input);
    let mut current_node = map.nodes.get("AAA");

    while let Some(node) = current_node.take() {
        let i = moves % map.dirs.len() as usize;
        if node.origin == "ZZZ" {
            break;
        }

        if map.dirs[i] == 'L' {
            if let Some(next_node) = map.nodes.get(&node.l as &str) {
                current_node = Some(next_node);
            }
        }
        if map.dirs[i] == 'R' {
            if let Some(next_node) = map.nodes.get(&node.r as &str) {
                current_node = Some(next_node);
            }
        }
        moves += 1;
    }

    return Some(moves as u32);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map = Map::new();
    map.parse(input);
    let mut start_nodes = Vec::new();
    for key in map.nodes.keys() {
        if key.chars().nth(2) == Some('A') {
            start_nodes.push(map.nodes.get(key));
        }
    }
    let mut move_list: Vec<u64> = Vec::new();

    for (_, current_node) in start_nodes.iter().enumerate() {
        let mut current_node = current_node.unwrap();
        let mut moves = 0;
        loop {
            let node = current_node;

            let i = moves % map.dirs.len() as u64;

            if node.origin.chars().nth(2) == Some('Z') {
                break;
            }
            moves += 1;
            if map.dirs[i as usize] == 'L' {
                let next_node = map.nodes.get(&node.l as &str);
                current_node = next_node.unwrap();
            }
            if map.dirs[i as usize] == 'R' {
                let next_node = map.nodes.get(&node.r as &str);
                current_node = next_node.unwrap();
            }
        }
        move_list.push(moves);
    }
    let mut acc = move_list[0] as usize;
    for i in 1..move_list.len() {
        acc = lcm(move_list[i] as usize, acc as usize);
    }
    return Some(acc as u64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));

        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));

        assert_eq!(result, Some(6));
    }
}
