use crate::file_utilities::read_lines;
use crate::map_utilities::{Point, DIRECTIONS};

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

struct Map {
    size: usize,
    walls: HashSet<Point>,
    start: Point,
    end: Point,
}

impl Map {
    fn new(size: usize, walls: HashSet<Point>) -> Self {
        Self {
            size,
            walls,
            start: Point::new(0, 0),
            end: Point::new((size - 1) as isize, (size - 1) as isize),
        }
    }

    fn find_shortest_path(&self) -> Vec<Point> {
        let mut queue = VecDeque::new();
        queue.push_back((self.start, vec![self.start]));

        let mut visited = HashSet::new();

        while let Some((current_point, current_path)) = queue.pop_front() {
            if visited.contains(&current_point) {
                // Already been here, no need.
                continue;
            }

            visited.insert(current_point);

            if current_point == self.end {
                // We made it, huzzah!
                return current_path;
            }

            if self.walls.contains(&current_point) {
                // We've hit a wall.
                continue;
            }

            for direction in DIRECTIONS.into_iter() {
                if let Some(neighbour) = current_point.neighbour(direction, self.size + 1) {
                    let mut new_path = current_path.clone();
                    new_path.push(neighbour);

                    queue.push_back((neighbour, new_path));
                }
            }
        }

        vec![]
    }
}

fn parse_line_to_point(line: String) -> Point {
    let values = line
        .split(",")
        .map(str::trim)
        .map(|x| x.parse::<isize>().unwrap())
        .collect_vec();

    Point::new(values[1], values[0])
}

fn parse_data(file_path: String) -> Vec<Point> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_point)
        .collect()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> (usize, usize) {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> (usize, usize) {
    let is_test = file_path.contains("test");
    let map_size = if is_test { 6 } else { 70 };
    let bytes_to_take = if is_test { 12 } else { 1024 };

    let bytes = parse_data(file_path);

    let map = Map::new(map_size, bytes.into_iter().take(bytes_to_take).collect());

    (map.find_shortest_path().len() - 1, 0)
}

fn part_2(file_path: String) -> (usize, usize) {
    let is_test = file_path.contains("test");
    let map_size = if is_test { 6 } else { 70 };
    let bytes_to_start_from = if is_test { 12 } else { 1024 };

    let bytes = parse_data(file_path);

    for bytes_to_take in bytes_to_start_from..bytes.len() {
        println!("Trying out {bytes_to_take} bytes out of {}.", bytes.len());

        let map = Map::new(
            map_size,
            bytes.iter().copied().take(bytes_to_take).collect(),
        );

        let shortest_path = map.find_shortest_path();

        if shortest_path.is_empty() {
            let blocking_byte = bytes[bytes_to_take - 1];
            return (blocking_byte.column(), blocking_byte.row());
        }
    }

    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, (22, 0))]
    #[case(false, (278, 0))]
    fn test_part_1(#[case] is_test: bool, #[case] expected: (usize, usize)) {
        assert_eq!(expected, part_1(get_file_path(is_test, 18, None)));
    }

    #[rstest]
    #[case(true, (6, 1))]
    #[case(false, (43, 12))]
    fn test_part_2(#[case] is_test: bool, #[case] expected: (usize, usize)) {
        assert_eq!(expected, part_2(get_file_path(is_test, 18, None)));
    }
}
