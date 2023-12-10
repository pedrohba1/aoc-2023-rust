use std::{collections::HashMap, ops::Index};

use regex::Regex;

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

impl Map {
    pub fn new() -> Self {
        Map {
            dirs: Vec::new(),
            nodes: HashMap::new(),
        }
    }
    pub fn parse(&mut self, input: &str) {
        let re = Regex::new(r"([A-Z])\w+").unwrap();

        for (i, line) in input.lines().enumerate() {
            if i == 0 {
                for c in line.chars() {
                    self.dirs.push(c);
                }
                continue;
            }
            let m: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();

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

pub fn part_two(input: &str) -> Option<u32> {
    let mut moves = 0;
    let mut map = Map::new();
    map.parse(input);

    let mut start_nodes = Vec::new();
    for key in map.nodes.keys() {
        // access the key here

        if key.chars().nth(2) == Some('A') {
            start_nodes.push(map.nodes.get(key));
        }
    }
    let mut moves = 0;

    for current_node in start_nodes.iter() {
        let mut current_node = current_node.unwrap();
        moves = 0;
        println!("current_node, {:?}", current_node);

        loop {
            let node = current_node;
            println!("node, {:?}", node);

            let i = moves % map.dirs.len() as usize;
            if node.origin.chars().nth(2) == Some('Z') {
                println!("Broke out in, {:?}", node);

                break;
            }
            if map.dirs[i] == 'L' {
                let next_node = map.nodes.get(&node.l as &str);
                current_node = next_node.unwrap();
            }
            if map.dirs[i] == 'R' {
                let next_node = map.nodes.get(&node.l as &str);
                current_node = next_node.unwrap();
            }
            moves += 1;
        }
        print!("moves before resetting, {}", moves);
    }
    return Some(moves as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
