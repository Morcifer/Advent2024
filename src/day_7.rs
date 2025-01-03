use std::collections::VecDeque;

use crate::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> Vec<u64> {
    line.replace(":", "")
        .split(" ")
        .map(str::trim)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn parse_data(file_path: String) -> Vec<Vec<u64>> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_int)
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> u64 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn get_new_value(current_value: u64, value_at_new_index: u64, operator_to_index: char) -> u64 {
    match operator_to_index {
        '+' => current_value + value_at_new_index,
        '*' => current_value * value_at_new_index,
        '|' => current_value * 10_u64.pow(value_at_new_index.ilog10() + 1) + value_at_new_index,
        _ => panic!("Not a value operator, how did you get here?!"),
    }
}

fn find_solution(datum: Vec<u64>, valid_operators: Vec<char>) -> Option<u64> {
    let final_value = *datum.first().unwrap();
    let values = datum.into_iter().skip(1).collect::<Vec<u64>>();

    let mut queue = VecDeque::new();

    for valid_operator in &valid_operators {
        queue.push_front((values[0], 1, *valid_operator)); // (current value, new index, operator to index)
    }

    while let Some((current_value, new_index, operator_to_index)) = queue.pop_front() {
        let new_value = get_new_value(current_value, values[new_index], operator_to_index);

        if new_value == final_value && new_index == values.len() - 1 {
            // Hurray, we're here! And we even used up all of our numbers!
            return Some(final_value);
        }

        if new_value > final_value {
            // Search tree is dead, because there's no '-' or '/'. Yet.
            continue;
        }

        if new_index == values.len() - 1 {
            // Search tree is dead, because we ran out of values and aren't at the correct result yet
            continue;
        }

        for valid_operator in &valid_operators {
            queue.push_front((new_value, new_index + 1, *valid_operator));
        }
    }

    None
}

fn part_1(file_path: String) -> u64 {
    let data = parse_data(file_path);

    data.into_iter()
        .filter_map(|datum| find_solution(datum, vec!['+', '*']))
        .sum()
}

fn part_2(file_path: String) -> u64 {
    let data = parse_data(file_path);

    data.into_iter()
        .filter_map(|datum| find_solution(datum, vec!['+', '*', '|']))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 3749)]
    #[case(false, 4555081946288)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 7, None)));
    }

    #[rstest]
    #[case(true, 11387)]
    #[case(false, 227921760109726)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 7, None)));
    }

    #[rstest]
    #[case(12, 345, 12345)]
    #[case(1234567, 1, 12345671)]
    #[case(1234567, 12345, 123456712345)]
    fn test_concatenation(
        #[case] first_value: u64,
        #[case] second_value: u64,
        #[case] expected: u64,
    ) {
        assert_eq!(expected, get_new_value(first_value, second_value, '|'));
    }

    #[test]
    fn test_weird_line() {
        let datum = vec![572800, 5, 727, 18, 82, 2];
        assert!(find_solution(datum, vec!['+', '*', '|']).is_none());
    }
}
