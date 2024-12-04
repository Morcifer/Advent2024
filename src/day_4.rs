use itertools::Itertools;

use crate::file_utilities::read_lines;

fn parse_data(file_path: String) -> Vec<Vec<char>> {
    read_lines(file_path)
        .into_iter()
        .map(|s| s.chars().collect_vec())
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> i32 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> i32 {
    let data = parse_data(file_path);
    let length = data.len();

    let mut count = 0;
    let neighbours = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let target = ['X', 'M', 'A', 'S'];

    for i in 0..length {
        for j in 0..length {
            for neighbour in neighbours.iter() {
                let mut valid = true;

                for (step, target_char) in target.iter().enumerate() {
                    let current_i = (i as isize) + (step as isize) * neighbour.0;
                    let current_j = (j as isize) + (step as isize) * neighbour.1;

                    if current_i < 0
                        || current_i >= (length as isize)
                        || current_j < 0
                        || current_j >= (length as isize)
                    {
                        valid = false;
                        break;
                    }

                    valid = valid && (*target_char == data[current_i as usize][current_j as usize]);
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_2(file_path: String) -> i32 {
    let data = parse_data(file_path);
    let length = data.len();

    let mut count = 0;

    let targets = ["MMASS", "MSAMS", "SMASM", "SSAMM"];

    for i in 1..length - 1 {
        for j in 1..length - 1 {
            let letters = vec![
                data[i - 1][j - 1],
                data[i - 1][j + 1],
                data[i][j],
                data[i + 1][j - 1],
                data[i + 1][j + 1],
            ];

            let string = String::from_iter(letters);
            let valid = targets.contains(&string.as_str());

            if valid {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 18)]
    #[case(false, 2447)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 4, None)));
    }

    #[rstest]
    #[case(true, 9)]
    #[case(false, 1868)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 4, None)));
    }
}
