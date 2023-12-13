advent_of_code::solution!(10);
use std::collections::VecDeque;
#[derive(Debug, Copy, Clone)]
struct Point {
    tile: char,
    distance: u32,
    x: usize,
    y: usize,
}

impl Point {
    pub fn set_dist(&mut self, distance: u32) {
        self.distance = distance;
    }
}

#[derive(Debug)]
struct Map {
    coords: Vec<Vec<Point>>,
    start: Option<Point>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            coords: Vec::new(),
            start: None,
        }
    }

    pub fn parse(&mut self, input: &str) {
        let mut coords = Vec::new();
        for (y, line) in input.lines().enumerate() {
            let mut xs = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let p = Point {
                    tile: (ch),
                    distance: 0,
                    x,
                    y,
                };
                if ch == 'S' {
                    self.start = Some(p);
                }
                xs.push(p);
            }
            coords.push(xs);
        }
        self.coords = coords;
    }

    pub fn get_next_points(&mut self, p: Point) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        if p.y > 0 {
            if let Some(row) = self.coords.get(p.y - 1) {
                if let Some(&top) = row.get(p.x) {
                    match top.tile {
                        '|' | 'F' | '7' if top.distance == 0 => points.push(top),
                        _ => {}
                    }
                }
            }
        }

        if p.y + 1 < self.coords.len() {
            if let Some(row) = self.coords.get(p.y + 1) {
                if let Some(&bottom) = row.get(p.x) {
                    match bottom.tile {
                        '|' | 'J' | 'L' if bottom.distance == 0 => points.push(bottom),
                        _ => {}
                    }
                }
            }
        }

        if p.x > 0 {
            if let Some(row) = self.coords.get(p.y) {
                if let Some(&left) = row.get(p.x - 1) {
                    match left.tile {
                        '-' | 'L' | 'F' if left.distance == 0 => points.push(left),
                        _ => {}
                    }
                }
            }
        }

        if p.x + 1 < self.coords[0].len() {
            if let Some(row) = self.coords.get(p.y) {
                if let Some(&right) = row.get(p.x + 1) {
                    match right.tile {
                        '-' | '7' | 'J' if right.distance == 0 => points.push(right),
                        _ => {}
                    }
                }
            }
        }

        points
    }

    pub fn flood_fill(&mut self) -> Option<u32> {
        let mut queue: VecDeque<Point> = VecDeque::new();
        queue.push_front(self.start.unwrap());

        let mut highest_dist: u32 = 0;
        while queue.len() != 0 {
            let current = queue.pop_front().unwrap();

            // check directions to go, get an array of possible and unvisited directions.
            let mut points = self.get_next_points(current);

            for p in points.iter_mut() {
                // on each direction, increment the point distance. {
                p.distance = current.distance + 1;
                self.coords[p.y][p.x] = *p;
                if p.distance > highest_dist {
                    highest_dist = p.distance;
                }
            }
            //start going into these directions by putting them on the queue
            queue.extend(points);
        }
        return Some(highest_dist);
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new();
    map.parse(input);
    // start flood fill algorithm
    map.flood_fill()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
