use itertools::Itertools;

use crate::file_utilities::read_as_single_line;

fn parse_data(file_path: String) -> String {
    read_as_single_line(file_path)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> i32 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn validate_character(program: &[char], spot: &mut usize, expected_character: char) -> bool {
    if program[*spot] != expected_character {
        return false;
    }

    *spot += 1;
    true
}

fn get_number(program: &[char], spot: &mut usize) -> Option<i32> {
    let number_start = *spot;

    while program[*spot].is_numeric() {
        *spot += 1;
    }

    let number_end = *spot;

    if !(1..=3).contains(&(number_end - number_start)) {
        return None;
    }

    Some(
        String::from_iter(program[number_start..number_end].iter())
            .parse::<i32>()
            .unwrap(),
    )
}

fn run_program(program_line: String, use_extra_instructions: bool) -> i32 {
    let mut result = 0;

    let mul_spots = program_line.match_indices("mul");
    let do_spots = program_line.match_indices("do()");
    let do_not_spots = program_line.match_indices("don't()");

    let all_spots = mul_spots
        .chain(do_spots.chain(do_not_spots))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect::<Vec<_>>();

    let program = program_line.chars().collect::<Vec<char>>();

    let mut is_do = true;

    for (spot, instruction) in all_spots {
        if instruction == "do()" {
            is_do = true;
            continue;
        }

        if instruction == "don't()" {
            is_do = false;
            continue;
        }

        if use_extra_instructions && !is_do {
            continue;
        }

        let mut current_spot = spot + 3;

        if !validate_character(&program, &mut current_spot, '(') {
            continue;
        }

        let first_number = get_number(&program, &mut current_spot);

        if first_number.is_none() {
            continue;
        }

        if !validate_character(&program, &mut current_spot, ',') {
            continue;
        }

        let second_number = get_number(&program, &mut current_spot);

        if second_number.is_none() {
            continue;
        }

        if !validate_character(&program, &mut current_spot, ')') {
            continue;
        }

        result += first_number.unwrap() * second_number.unwrap();
    }

    result
}

fn part_1(file_path: String) -> i32 {
    let program_line = parse_data(file_path);
    run_program(program_line, false)
}

fn part_2(file_path: String) -> i32 {
    let program_line = parse_data(file_path);
    run_program(program_line, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 161)]
    #[case(false, 180233229)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 3, None)));
    }

    #[rstest]
    #[case(true, 48)]
    #[case(false, 95411583)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 3, None)));
    }
}
