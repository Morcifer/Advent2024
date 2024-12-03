use itertools::Itertools;

use crate::file_utilities::read_lines;

fn parse_data(file_path: String) -> Vec<String> {
    read_lines(file_path)
        .into_iter()
        .collect()
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
    let program_lines = parse_data(file_path);
    println!("{:?}", program_lines);

    let mut result = 0;

    for program_line in program_lines {
        let mul_spots = program_line.match_indices("mul");
        println!("{:?}", mul_spots);

        let program = program_line.chars().collect_vec();

        for (mul_spot, _) in mul_spots {
            let mut current_spot = mul_spot;

            // Validate opening parenthesis
            current_spot += 3;

            if program[current_spot] != '(' {
                continue;
            }

            current_spot += 1;

            // Find first digit
            let number_start = current_spot;

            while program[current_spot].is_numeric() {
                current_spot += 1;
            }

            let number_end = current_spot;

            if !(1..=3).contains(&(number_end - number_start)) {
                continue;
            }

            let first_number = String::from_iter(program[number_start..number_end].iter()).parse::<i32>().unwrap();

            // Validate comma
            if program[current_spot] != ',' {
                continue;
            }

            current_spot += 1;

            // Find second digit
            let number_start = current_spot;

            while program[current_spot].is_numeric() {
                current_spot += 1;
            }

            let number_end = current_spot;

            if !(1..=3).contains(&(number_end - number_start)) {
                continue;
            }

            let second_number = String::from_iter(program[number_start..number_end].iter()).parse::<i32>().unwrap();

            // Validate closing parenthesis
            if program[current_spot] != ')' {
                continue;
            }

            current_spot += 1;

            result += first_number * second_number;
        }
    }

    result
}

fn part_2(file_path: String) -> i32 {
    let program_lines = parse_data(file_path);
    println!("{:?}", program_lines);

    let mut combined_program = "".to_string();

    for program_line in program_lines {
        combined_program.push_str(&program_line);
    }

    let program_lines = vec![combined_program];

    let mut result = 0;

    for program_line in program_lines {
        let mul_spots = program_line.match_indices("mul");

        let do_spots = program_line.match_indices("do()").map(|(spot, string)| spot).collect::<Vec<_>>();
        let do_not_spots = program_line.match_indices("don't()").map(|(spot, string)| spot).collect::<Vec<_>>();

        println!("{:?}", mul_spots);

        let program = program_line.chars().collect_vec();

        for (mul_spot, _) in mul_spots {
            let mut current_spot = mul_spot;
            let recent_do_spot = do_spots.iter().filter(|s| **s < mul_spot).last().unwrap_or_else(|| { &0 });
            let recent_do_not_spot = do_not_spots.iter().filter(|s| **s < mul_spot).last();

            if recent_do_not_spot.is_some() && recent_do_not_spot.unwrap() > recent_do_spot {
                continue;
            }

            // Validate opening parenthesis
            current_spot += 3;

            if program[current_spot] != '(' {
                continue;
            }

            current_spot += 1;

            // Find first digit
            let number_start = current_spot;

            while program[current_spot].is_numeric() {
                current_spot += 1;
            }

            let number_end = current_spot;

            if !(1..=3).contains(&(number_end - number_start)) {
                continue;
            }

            let first_number = String::from_iter(program[number_start..number_end].iter()).parse::<i32>().unwrap();

            // Validate comma
            if program[current_spot] != ',' {
                continue;
            }

            current_spot += 1;

            // Find second digit
            let number_start = current_spot;

            while program[current_spot].is_numeric() {
                current_spot += 1;
            }

            let number_end = current_spot;

            if !(1..=3).contains(&(number_end - number_start)) {
                continue;
            }

            let second_number = String::from_iter(program[number_start..number_end].iter()).parse::<i32>().unwrap();

            // Validate closing parenthesis
            if program[current_spot] != ')' {
                continue;
            }

            current_spot += 1;

            result += first_number * second_number;
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
    #[case(true, -1)]
    #[case(false, -1)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 3, None)));
    }

    #[rstest]
    #[case(true, -1)]
    #[case(false, -1)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 3, None)));
    }
}
