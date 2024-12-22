use itertools::Itertools;

use crate::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> u64 {
    line.parse::<u64>().expect("Failed with {line}")
}

fn parse_data(file_path: String) -> Vec<u64> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_int)
        .collect::<Vec<_>>()
}

fn evolve(secret: u64) -> u64 {
    let step_1 = step_1(secret);
    let step_2 = step_2(step_1);
    step_3(step_2)
}

fn step_1(secret: u64) -> u64 {
    let result = secret * 64;
    let result = mix(result, secret);
    prune(result)
}

fn step_2(secret: u64) -> u64 {
    let result = secret / 32;
    let result = mix(result, secret);
    prune(result)
}

fn step_3(secret: u64) -> u64 {
    let result = secret * 2048;
    let result = mix(result, secret);
    prune(result)
}

fn mix(number: u64, secret: u64) -> u64 {
    number ^ secret
}

fn prune(number: u64) -> u64 {
    number % 16777216
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> u64 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}
fn part_1(file_path: String) -> u64 {
    let data = parse_data(file_path);

    let mut numbers = data.clone();

    for _evolution in 0..2000 {
        // println!("Evolving {_evolution} out of 2000");
        numbers = numbers.into_iter().map(evolve).collect_vec();
    }

    numbers.into_iter().sum()
}

fn part_2(file_path: String) -> u64 {
    let _data = parse_data(file_path);

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 37327623)]
    #[case(false, 20071921341)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 22, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 22, None)));
    }

    #[rstest]
    #[case(123, 15887950)]
    #[case(15887950, 16495136)]
    #[case(527345, 704524)]
    fn test_evolve(#[case] secret: u64, #[case] expected: u64) {
        assert_eq!(expected, evolve(secret));
    }
}