use crate::file_utilities::read_two_chunks;
use crate::map_utilities::{Direction, Point};

use itertools::Itertools;

struct Map {
    robot: Point,
    walls: Vec<Point>,
    boxes: Vec<Point>,
}

impl Map {
    fn new() -> Self {
        Self {
            robot: Point::new(0, 0),
            walls: vec![],
            boxes: vec![],
        }
    }
}

fn parse_data(file_path: String) -> (Map, Vec<Direction>) {
    let (map_lines, direction_lines) = read_two_chunks(file_path);

    let mut map = Map::new();

    for (row_index, row) in map_lines.into_iter().enumerate() {
        for (column_index, character) in row.chars().enumerate() {
            match character {
                '.' => continue,
                '#' => map
                    .walls
                    .push(Point::new(row_index as isize, column_index as isize)),
                'O' => map
                    .boxes
                    .push(Point::new(row_index as isize, column_index as isize)),
                '@' => map.robot = Point::new(row_index as isize, column_index as isize),
                _ => panic!("Got an invalid map thingie."),
            }
        }
    }

    let directions = direction_lines
        .into_iter()
        .flat_map(move |line| line.chars().collect_vec())
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Got an invalid movement."),
        })
        .collect_vec();

    (map, directions)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> usize {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> usize {
    let (map, directions) = parse_data(file_path);
    // println!("robot {:?}, walls {:?}, boxes {:?}", map.robot, map.walls, map.boxes);
    // println!("{directions:?}");

    0
}

fn part_2(file_path: String) -> usize {
    let _ = parse_data(file_path);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 15, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 15, None)));
    }
}
