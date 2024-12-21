use itertools::Itertools;
use std::cmp::Ordering;

use std::collections::HashMap;
use std::iter;

use crate::map_utilities::{Direction, Point};

// TODO: real input from file!

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> usize {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

const NUMERICAL_PAD: &[&[char]] = &[
    &['7', '8', '9'],
    &['4', '5', '6'],
    &['1', '2', '3'],
    &['X', '0', 'A'],
];
const DIRECTIONAL_PAD: &[&[char]] = &[&['X', '^', 'A'], &['<', 'v', '>']];

fn find_button(button: char, pad: &[&[char]]) -> Point {
    for (i, row) in pad.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if button == *char {
                return Point::new(i as isize, j as isize);
            }
        }
    }

    panic!("I couldn't find button {button} for some reason!")
}

fn is_valid(pad: &[&[char]], numerical_start: Point, perm: &Vec<char>) -> bool {
    let mut current = numerical_start;

    for direction_char in perm {
        let direction = Direction::from_char(*direction_char);

        current = current.unbound_neighbour(direction);

        if pad[current.row()][current.column()] == 'X' {
            return false;
        }
    }

    true
}

fn get_buttons(start_point: Point, end_point: Point) -> (char, char) {
    let row_button = match end_point.row.cmp(&start_point.row) {
        Ordering::Greater => 'v',
        _ => '^',
    };

    let column_button = match end_point.column.cmp(&start_point.column) {
        Ordering::Greater => '>',
        _ => '<',
    };

    (row_button, column_button)
}

fn get_valid_arrow_permutations(
    pad: &[&[char]],
    start_point: Point,
    end_point: Point,
) -> Vec<Vec<char>> {
    let (row_button, column_button) = get_buttons(start_point, end_point);

    let row_diff = (start_point.row - end_point.row).unsigned_abs();
    let row_thing = itertools::repeat_n(row_button, row_diff).collect_vec();

    let column_diff = (start_point.column - end_point.column).unsigned_abs();
    let column_thing = itertools::repeat_n(column_button, column_diff).collect_vec();

    let all_arrows = row_thing.into_iter().chain(column_thing).collect_vec();

    let len = all_arrows.len();

    all_arrows
        .into_iter()
        .permutations(len)
        .unique()
        .map(|perm| perm.iter().copied().collect_vec())
        .filter(|perm| is_valid(pad, start_point, perm))
        .collect_vec()
}

fn recursive_run(
    pad: &[&[char]],
    from: char,
    to: char,
    depth: usize,
    cache: &mut HashMap<(char, char, usize), usize>,
) -> usize {
    // println!("Getting paths from {from} to {to} at depth {depth}");

    if let Some(result) = cache.get(&(from, to, depth)) {
        // println!("Found cache of {result}");
        return *result;
    }

    let from_point = find_button(from, pad);
    let to_point = find_button(to, pad);

    if depth == 0 {
        // Don't forget to actually press the button!
        return from_point.manhattan_distance(&to_point) + 1;
    }

    let result = get_valid_arrow_permutations(pad, from_point, to_point)
        .into_iter()
        .map(|perm| {
            let new_perm = iter::once('A')
                .chain(perm)
                .chain(iter::once('A'))
                .collect_vec();
            // println!("Investigating new permutation {}", new_perm.iter().join(""));

            new_perm
                .into_iter()
                .tuple_windows()
                .map(|(from, to)| recursive_run(DIRECTIONAL_PAD, from, to, depth - 1, cache))
                .sum()
        })
        .min()
        .unwrap();

    cache.insert((from, to, depth), result);
    result
}

fn run_for_robots(sequences: &[&str], robots: usize) -> usize {
    let mut result = 0;

    let mut cache = HashMap::new();

    for sequence in sequences {
        // println!("Handling sequence {sequence:?}");

        let numeric_part = sequence[0..sequence.len() - 1].parse::<usize>().unwrap();

        let sequence_chars = iter::once('A').chain(sequence.chars()).collect_vec();

        let shortest_sequence: usize = sequence_chars
            .into_iter()
            .tuple_windows()
            .map(|(from, to)| recursive_run(NUMERICAL_PAD, from, to, robots, &mut cache))
            .sum();

        // println!("{sequence}: length {shortest_sequence}, numeric {numeric_part}");
        result += shortest_sequence * numeric_part;
    }

    result
}

const TEST_CASE: &[&str] = &["029A", "980A", "179A", "456A", "379A"];
const REAL_CASE: &[&str] = &["463A", "340A", "129A", "083A", "341A"];

fn part_1(file_path: String) -> usize {
    let sequences = if file_path.contains("test") {
        TEST_CASE
    } else {
        REAL_CASE
    };

    run_for_robots(sequences, 2)
}

fn part_2(file_path: String) -> usize {
    let sequences = if file_path.contains("test") {
        TEST_CASE
    } else {
        REAL_CASE
    };

    run_for_robots(sequences, 25)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 126384)]
    #[case(false, 94426)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 21, None)));
    }

    #[rstest]
    #[case(true, 154115708116294)]
    #[case(false, 118392478819140)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 21, None)));
    }
}
