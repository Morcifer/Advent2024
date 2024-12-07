use std::collections::VecDeque;

use crate::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> Vec<i64> {
    line.replace(":", "")
        .split(" ")
        .map(str::trim)
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_data(file_path: String) -> Vec<Vec<i64>> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_int)
        .collect::<Vec<_>>()
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
    let data = parse_data(file_path);

    let mut result = 0;

    for datum in data {
        let final_value = *datum.first().unwrap();
        let values = datum.into_iter().skip(1).collect::<Vec<i64>>();

        let mut queue = VecDeque::new();

        queue.push_front((values[0], 1, '+')); // (current value, new index, operator to index)
        queue.push_front((values[0], 1, '*')); // (current value, new index, operator to index)

        while let Some((current_value, new_index, operator_to_index)) = queue.pop_front() {
            let new_value = match operator_to_index {
                '+' => current_value + values[new_index],
                '*' => current_value * values[new_index],
                _ => panic!("Not a value operator, how did you get here?!"),
            };

            if new_value == final_value {
                // Hurray, we're here!
                result += final_value;
                break;
            }

            if new_value > final_value {
                // Search tree is dead, because there's no '-' or '/'. Yet.
                continue;
            }

            if new_index == values.len() - 1 {
                // Search tree is dead, because we ran out of values and aren't at the correct result yet
                continue;
            }

            queue.push_front((new_value, new_index + 1, '+'));
            queue.push_front((new_value, new_index + 1, '*'));
        }
    }

    result
}

fn part_2(file_path: String) -> i64 {
    let data = parse_data(file_path);

    let mut result = 0;

    for datum in data {
        let final_value = *datum.first().unwrap();
        let values = datum.into_iter().skip(1).collect::<Vec<i64>>();

        let mut queue = VecDeque::new();

        queue.push_front((values[0], 1, '+')); // (current value, new index, operator to index)
        queue.push_front((values[0], 1, '*')); // (current value, new index, operator to index)
        queue.push_front((values[0], 1, '|')); // (current value, new index, operator to index)

        while let Some((current_value, new_index, operator_to_index)) = queue.pop_front() {
            let value_at_new_index = values[new_index];

            let new_value = match operator_to_index {
                '+' => current_value + value_at_new_index,
                '*' => current_value * value_at_new_index,
                '|' => {
                    current_value * 10_i64.pow(value_at_new_index.to_string().len() as u32)
                        + value_at_new_index
                }
                _ => panic!("Not a value operator, how did you get here?!"),
            };

            if new_value == final_value {
                // Hurray, we're here!
                result += final_value;
                break;
            }

            if new_value > final_value {
                // Search tree is dead, because there's no '-' or '/'. Yet.
                continue;
            }

            if new_index == values.len() - 1 {
                // Search tree is dead, because we ran out of values and aren't at the correct result yet
                continue;
            }

            queue.push_front((new_value, new_index + 1, '+'));
            queue.push_front((new_value, new_index + 1, '*'));
            queue.push_front((new_value, new_index + 1, '|'));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 3749)]
    #[case(false, 4555081946288)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 7, None)));
    }

    #[rstest]
    #[case(true, 11387)]
    #[case(false, -1)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 7, None)));
    }
}
