use itertools::Itertools;
use std::cmp::Ordering;

use std::collections::HashMap;
use std::iter;

use crate::map_utilities::{Direction, Point};

// TODO: see if the code can be changed to get rid of get_pad_paths,
//  and use the same recursive code for the number pad.

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

fn get_pad_paths(pad: &[&[char]]) -> HashMap<(char, char), Vec<Vec<char>>> {
    let mut mapping = HashMap::new();

    for start in pad.iter().copied().flatten() {
        if *start == 'X' {
            continue;
        }

        let start_point = find_button(*start, pad);

        for end in pad.iter().copied().flatten() {
            if *end == 'X' {
                continue;
            }

            let end_point = find_button(*end, pad);

            mapping.insert(
                (*start, *end),
                get_valid_arrow_permutations(pad, start_point, end_point),
            );

            if mapping[&(*start, *end)].is_empty() {
                panic!("I shouldn't have dead ends from {start} to {end}!");
            }
        }
    }

    mapping
}

fn get_numerical_sequences(
    pad_paths: &HashMap<(char, char), Vec<Vec<char>>>,
    sequence_chars: &[char],
) -> Vec<Vec<char>> {
    // println!("Getting numerical sequences for {:?}", sequence_chars.iter().join(""));

    let mut paths: Vec<Vec<char>> = vec![vec![]];

    for (button_from, button_to) in sequence_chars.iter().tuple_windows() {
        paths = paths
            .clone()
            .into_iter()
            .flat_map(|path| {
                pad_paths[&(*button_from, *button_to)]
                    .clone()
                    .into_iter()
                    .map(|sub_path| {
                        path.iter()
                            .copied()
                            .chain(sub_path.into_iter())
                            .chain(iter::once('A'))
                            .collect_vec()
                    })
                    .collect_vec()
            })
            .collect_vec();
    }

    paths
}

fn recursive_directional_run(
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

    let from_point = find_button(from, DIRECTIONAL_PAD);
    let to_point = find_button(to, DIRECTIONAL_PAD);

    if depth == 0 {
        let result = from_point.manhattan_distance(&to_point) + 1; // Don't forget to actually press the button!

        return result;
    }

    let result = get_valid_arrow_permutations(DIRECTIONAL_PAD, from_point, to_point)
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
                .map(|(from, to)| recursive_directional_run(from, to, depth - 1, cache))
                .sum()
        })
        .min()
        .unwrap();

    cache.insert((from, to, depth), result);
    result
}

fn run_for_robots(sequences: &[&str], robots: usize) -> usize {
    let numerical_pad_paths = get_pad_paths(NUMERICAL_PAD);

    let mut result = 0;

    let mut directional_cache = HashMap::new();

    for sequence in sequences {
        // println!("Handling sequence {sequence:?}");

        let numeric_part = sequence[0..sequence.len() - 1].parse::<usize>().unwrap();

        let sequence_chars = iter::once('A').chain(sequence.chars()).collect_vec();
        let relevant_char_sequences =
            get_numerical_sequences(&numerical_pad_paths, &sequence_chars);

        let shortest_sequence: usize = relevant_char_sequences
            .into_iter()
            .map(|seq| {
                let full_seq = iter::once('A').chain(seq.into_iter()).collect_vec();
                // println!("Investigating sequence {}", full_seq.iter().join(""));

                full_seq
                    .into_iter()
                    .tuple_windows()
                    .map(|(from, to)| {
                        recursive_directional_run(from, to, robots - 1, &mut directional_cache)
                    })
                    .sum()
            })
            .min()
            .unwrap();

        result += shortest_sequence * numeric_part;
        println!("{sequence}: {shortest_sequence}, {numeric_part}");
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
