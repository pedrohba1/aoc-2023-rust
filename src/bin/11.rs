advent_of_code::solution!(11);
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Map {
    data: HashMap<Key, char>,
    h: usize,
    w: usize,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Key {
    x: usize,
    y: usize,
}

impl Map {
    fn expand(&mut self) {
        let mut new_rows = HashSet::new();
        let mut new_cols = HashSet::new();

        // find rows that can be expanded
        for y in 0..self.h {
            if (0..self.w).all(|x| *self.data.get(&Key { x, y }).unwrap() == '.') {
                new_rows.insert(y);
            }
        }

        // find columns that can be expanded
        // Check columns
        for x in 0..self.w {
            if (0..self.h).all(|y| *self.data.get(&Key { x, y }).unwrap() == '.') {
                new_cols.insert(x);
            }
        }

        // expand rows
        let mut new_map: HashMap<Key, char> = HashMap::new();
        let mut lk_x = 0;
        let mut lk_y = 0;
        println!("new cols: {:?}", new_cols);

        for y in 0..self.h {
            for x in 0..self.w {
                let key = Key { x, y };
                let character = self.data.get(&key).unwrap_or(&'N');
                print!("{}", character);

                new_map.insert(
                    Key {
                        x: x + lk_x,
                        y: y + lk_y,
                    },
                    *character,
                );

                if new_cols.contains(&x) {
                    lk_x += 1;
                    new_map.insert(
                        Key {
                            x: x + lk_x,
                            y: y + lk_y,
                        },
                        '.',
                    );
                }
            }
            println!("");
            lk_x = 0;
            if new_rows.contains(&y) {
                lk_y += 1;
                for x in 0..self.w + new_cols.len() {
                    new_map.insert(Key { x: x, y: y + lk_y }, '.');
                }
            }
        }

        self.data = new_map;
        self.w += new_cols.len();
        self.h += new_rows.len();
    }

    // Function to print the map
    fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                let key = Key { x, y };
                // Get the character at the current position, defaulting to a space if none is found
                let character = self.data.get(&key).unwrap_or(&'N');
                print!("{}", character);
            }
            println!(); // New line at the end of each row
        }
    }
}

fn parse(input: &str) -> (Map) {
    let mut map = HashMap::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    for (y, line) in input.lines().enumerate() {
        let y = y as usize;
        max_y = max_y.max(y);

        for (x, ch) in line.chars().enumerate() {
            let x = x as usize;
            max_x = max_x.max(x);

            map.insert(Key { x, y }, ch);
        }
    }

    println!("max w, {} max_h, {}", max_x, max_y);
    Map {
        data: map,
        h: max_y + 1,
        w: max_x + 1,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    // exapnd the matrix
    map.expand();

    println!("new map");
    map.print();
    // get all # as numbers, mapping each
    // number as a key of a map

    // calcualte manhattan distance betwwen each and all of them.

    None
}

pub fn part_two(input: &str) -> Option<u32> {
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
