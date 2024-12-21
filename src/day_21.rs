use crate::map_utilities::Point;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter;
use std::iter::Once;
// TODO: real input from file!

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> u64 {
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

fn get_pad_paths(pad: &[&[char]]) -> HashMap<(char, char), Vec<Vec<char>>> {
    // let numerical_pad = [['7', '8', '9'], ['4', '5', '6'], ['1', '2', '3'], ['X', '0', 'A']];
    // let directional_pad = [['X', '^', 'A'], ['<', 'v', '>']];

    let mut mapping = HashMap::new();

    for numerical_start in pad.to_vec().into_iter().flatten() {
        let numerical_start_point = find_button(*numerical_start, pad);

        for numerical_end in pad.to_vec().into_iter().flatten() {
            let numerical_end_point = find_button(*numerical_end, pad);

            let mut all_arrows = vec![];

            let row_button = if numerical_end_point.row > numerical_start_point.row {
                'v'
            } else {
                '^'
            };
            let column_button = if numerical_end_point.column > numerical_start_point.column {
                '>'
            } else {
                '<'
            };

            for _ in 0..(numerical_start_point.row - numerical_end_point.row).abs() {
                all_arrows.push(row_button);
            }

            for _ in 0..(numerical_start_point.column - numerical_end_point.column).abs() {
                all_arrows.push(column_button);
            }

            let len = all_arrows.len();

            // TODO: Remember that you're not allowed to be over the 'X'...

            mapping.insert(
                (*numerical_start, *numerical_end),
                all_arrows
                    .into_iter()
                    .permutations(len)
                    .unique()
                    .map(|perm| perm.iter().copied().collect_vec())
                    .collect_vec(),
            );
        }
    }

    mapping
}


fn get_sequence(
    numerical_pad_paths: &HashMap<(char, char), Vec<Vec<char>>>,
    sequence_chars: &Vec<char>,
) -> Vec<Vec<char>> {

    let mut first_directional_paths = numerical_pad_paths[&('A', sequence_chars[0])]
        .clone()
        .into_iter()
        .map(|path| path.into_iter().chain(iter::once('A')).collect_vec())
        .collect_vec();

    for (button_from, button_to) in sequence_chars.into_iter().tuple_windows() {
        first_directional_paths = first_directional_paths
            .clone()
            .into_iter()
            .flat_map(|path| {
                numerical_pad_paths[&(*button_from, *button_to)]
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

    // for first_directional_path in first_directional_paths.iter() {
    //     println!("Option is {first_directional_path:?}");
    // }

    first_directional_paths
}


const TEST_CASE: &[&str] = &["029A", "980A", "179A", "456A", "379A"];
const REAL_CASE: &[&str] = &["463A", "340A", "129A", "083A", "341A"];

fn part_1(file_path: String) -> u64 {
    let sequences = if file_path.contains("test") {
        TEST_CASE
    } else {
        REAL_CASE
    };

    let numerical_pad_paths = get_pad_paths(NUMERICAL_PAD);
    // println!("{numerical_pad_paths:?}");

    let directional_pad_paths = get_pad_paths(DIRECTIONAL_PAD);
    // println!("{directional_pad_paths:?}");
    let mut result = 0;

    for sequence in sequences {
        println!("Handling sequence {sequence:?}");

        let numeric_part = sequence[0..sequence.len() - 1].parse::<u64>().unwrap();

        let sequence_chars = sequence.chars().collect_vec();
        let first_sequences = get_sequence(&numerical_pad_paths, &sequence_chars);

        println!(
            "Shortest first sequence: {}, longest {}",
            first_sequences.iter().map(|s| s.len()).min().unwrap(),
            first_sequences.iter().map(|s| s.len()).max().unwrap(),
        );

        let mut second_sequences = first_sequences
            .into_iter()
            .flat_map(|sequence| get_sequence(&directional_pad_paths, &sequence))
            .collect_vec();

        let shortest_second = second_sequences.iter().map(|s| s.len()).min().unwrap();
        second_sequences = second_sequences.into_iter().filter(|s| s.len() == shortest_second).collect_vec();

        let mut third_sequences = second_sequences
            .into_iter()
            .flat_map(|sequence| get_sequence(&directional_pad_paths, &sequence))
            .collect_vec();

        let shortest_third = third_sequences.iter().map(|s| s.len()).min().unwrap();
        third_sequences = third_sequences.into_iter().filter(|s| s.len() == shortest_third).collect_vec();

        println!(
            "Shortest third sequence: {}, longest {}",
            third_sequences.iter().map(|s| s.len()).min().unwrap(),
            third_sequences.iter().map(|s| s.len()).max().unwrap(),
        );

        let shortest_sequence = third_sequences.into_iter().map(|sequence| sequence.len()).min().unwrap() as u64;

        println!("Adding {shortest_sequence} and {numeric_part}");
        result += shortest_sequence * numeric_part;
    }

    result
}

fn part_2(file_path: String) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 126384)] //  123844
    #[case(false, 0)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 21, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 21, None)));
    }
}
