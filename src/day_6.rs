use std::collections::HashSet;
use itertools::Itertools;
use crate::file_utilities::read_lines;


fn parse_data(file_path: String) -> Vec<String> {
    read_lines(file_path)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> i32 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Default,
    Up,
    Right,
    Down,
    Left,
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Default => panic!("wrong direction"),
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn go_straight(row: usize, column: usize, direction: Direction, size: usize) -> Option<(usize, usize, Direction)> {
    let (new_row, new_column) = match direction {
        Direction::Default => panic!("wrong direction"),
        Direction::Up => (row as isize - 1, column as isize ),
        Direction::Right => (row as isize , column as isize  + 1),
        Direction::Down => (row as isize  + 1, column as isize ),
        Direction::Left => (row as isize , column as isize  - 1),
    };

    if new_row < 0 || new_column < 0 || new_row >= size as isize || new_column >= size as isize {
        return None;
    }

    Some((new_row as usize, new_column as usize, direction))
}

fn parse_guard(c: char) -> Direction {
    match c {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
        _ => panic!("... nope, specifically with a {c}."),
    }
}

fn part_1(file_path: String) -> i32 {
    let map = parse_data(file_path);
    let mut guard = (0, 0, Direction::Default);
    let mut obstacles = HashSet::new();

    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            match c {
                '#' => { obstacles.insert((i, j)); continue },
                '.' => continue,
                x if !x.is_alphanumeric() => guard = (i, j, parse_guard(x)),
                _ => panic!("... also nope, specifically with a {c}"),
            }
        }
    }

    let mut history = HashSet::new();

    while !history.contains(&guard) {
        println!("Guard reached {guard:?}");
        history.insert(guard.clone());

        let next = go_straight(guard.0, guard.1, guard.2, map.len());

        if let Some(next) = next {
            if obstacles.contains(&(next.0, next.1)) {
                guard = (guard.0, guard.1, turn_right(guard.2));
            }
            else
            {
                guard = next;
            }
            continue;
        }

        break;
    }

    history.into_iter().map(|g| (g.0, g.1)).unique().count() as i32
}

fn part_2(file_path: String) -> i32 {
    let map = parse_data(file_path);
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 41)]
    #[case(false, 5242)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 6, None)));
    }

    #[rstest]
    #[case(true, -1)]
    #[case(false, -1)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 6, None)));
    }
}
