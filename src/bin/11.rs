advent_of_code::solution!(11);
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Map {
    data: HashMap<Key, char>,
    h: usize,
    w: usize,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Key {
    x: usize,
    y: usize,
}

fn manhattan_path(val1: Key, val2: Key) -> HashSet<Key> {
    let mut path = HashSet::new();

    println!("val1  {:?} val 2{:?}", val1, val2);

    // Move vertically to reach target y-coordinate
    let y_step: i32 = if val1.y <= val2.y { 1 } else { -1 };
    let mut y: i32 = val1.y as i32;
    while y != val2.y as i32 {
        path.insert(Key {
            x: val2.x,
            y: y as usize,
        });
        y += y_step;
    }

    // Move horizontally to align with target x-coordinate
    let x_step: i32 = if val1.x <= val2.x { 1 } else { -1 };
    let mut x: i32 = val1.x as i32;
    while x != val2.x as i32 {
        path.insert(Key {
            x: x as usize,
            y: val1.y,
        });
        x += x_step;
    }

    path
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

        for y in 0..self.h {
            for x in 0..self.w {
                let key = Key { x, y };
                let character = self.data.get(&key).unwrap_or(&'N');

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
                        '*',
                    );
                }
            }
            lk_x = 0;
            if new_rows.contains(&y) {
                lk_y += 1;
                for x in 0..(self.w + new_cols.len()) {
                    new_map.insert(Key { x: x, y: y + lk_y }, '*');
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

    fn sum_distances(&self, mul: u32) -> u32 {
        let mut acc = 0;
        // find all positions of #

        // compare each position the all the others,
        let mut pos: HashMap<u32, Key> = HashMap::new();

        let mut pos_count = 1;
        for (key, val) in self.data.iter() {
            if val == &'#' {
                pos.insert(pos_count, *key);
                pos_count += 1;
            }
        }

        for (_, val1) in pos.iter() {
            for (_, val2) in pos.iter() {
                if val1 == val2 {
                    continue;
                }
                // from val1, get directions to val2,
                let mut expansion = 0;
                let path = manhattan_path(*val1, *val2);

                for p in path {
                    if self.data.get(&p).unwrap() == &'*' {
                        expansion += 1;
                        println!("expasion at {:?}", p);
                    }
                }
                break;
                // everytime a * shows up in the path, compute the multiplier

                acc += (val2.x as i32 - val1.x as i32).abs()
                    + (val2.y as i32 - val1.y as i32).abs()
                    + mul as i32 * expansion as i32;
            }
        }

        // summ all the distances in accumulator
        acc as u32 / 2
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

    Some(map.sum_distances(1))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse(input);
    // exapnd the matrix
    map.expand();
    map.print();

    Some(map.sum_distances(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1030));
    }
}
