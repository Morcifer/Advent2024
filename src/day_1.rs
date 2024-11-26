use std::collections::HashSet;
use nohash_hasher::BuildNoHashHasher;
use crate::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> i32 {
    line.trim_start_matches('+').parse::<i32>().unwrap()
}

fn parse_data(file_path: String) -> Vec<i32> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_int)
        .collect()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> i32 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope.")
    }
}

fn part_1(file_path: String) -> i32 {
    let numbers = parse_data(file_path);
    numbers.into_iter().sum()
}

fn part_2(file_path: String) -> i32 {
    let numbers = parse_data(file_path);
    let mut index = 0;
    let mut value = 0;
    let mut history: HashSet<i32, BuildNoHashHasher<i32>> = HashSet::with_hasher(BuildNoHashHasher::default());

    while history.insert(value)  // insert returns false if the value is already there.
    {
        value += numbers[index];
        index = (index + 1) % numbers.len();
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use crate::file_utilities::get_file_path;

    #[rstest]
    #[case(true, 3)]
    #[case(false, 400)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 1, None)));
    }

    #[rstest]
    #[case(true, 2)]
    #[case(false, 232)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 1, None)));
    }
}