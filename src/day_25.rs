use itertools::Itertools;

use crate::file_utilities::read_chunks;

fn parse_lines_to_char_vecs(lines: Vec<String>) -> Vec<Vec<char>> {
    lines
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn parse_data(file_path: String) -> Vec<Vec<Vec<char>>> {
    read_chunks(file_path)
        .into_iter()
        .map(parse_lines_to_char_vecs)
        .collect_vec()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> i64 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> i64 {
    let keys_and_locks = parse_data(file_path);
    let width = keys_and_locks.first().unwrap()[0].len();
    let height = keys_and_locks.first().unwrap().len();

    let mut locks = vec![];
    let mut keys = vec![];

    for key_or_lock in keys_and_locks.into_iter() {
        let mut heights = vec![];
        if key_or_lock[0].iter().all(|c| *c == '#') {
            // Is lock!
            for column in 0..width {
                for (height, row) in key_or_lock.iter().skip(1).enumerate() {
                    if row[column] == '.' {
                        heights.push(height);
                        break;
                    }
                }
            }
            locks.push(heights);
        } else {
            // Is key!
            for column in 0..width {
                for (height, row) in key_or_lock.iter().rev().skip(1).enumerate() {
                    if row[column] == '.' {
                        heights.push(height);
                        break;
                    }
                }
            }
            keys.push(heights);
        }
    }

    let mut result = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            let mut overlap = false;
            for column in 0..width {
                if lock[column] + key[column] > height - 2 {
                    // println!("{lock:?} and {key:?} overlap at column {column}");
                    overlap = true;
                    break;
                }
            }

            if !overlap {
                // println!("{lock:?} and {key:?} do NOT overlap!");
                result += 1;
            }
        }
    }

    result
}

fn part_2(_file_path: String) -> i64 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 3)]
    #[case(false, 3133)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 25, None)));
    }
}
