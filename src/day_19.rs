use std::collections::{VecDeque};

use crate::file_utilities::read_two_chunks;

fn parse_rule_line(line: String) -> (usize, usize) {
    let values = line
        .split("|")
        .map(str::trim)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (values[0], values[1])
}

fn parse_pages_line(line: String) -> Vec<usize> {
    line.split(",")
        .map(str::trim)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_data(file_path: String) -> (Vec<String>, Vec<String>) {
    let (patterns_line, designs) = read_two_chunks(file_path);

    let patterns = patterns_line[0].split(",").map(str::trim).map(|s| s.to_string()).collect();

    (patterns, designs)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> i32 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn design_can_be_created(design: String, patterns: &Vec<String>) -> bool {
    println!("Design: {design}");
    let mut queue = VecDeque::new();
    queue.push_front(0);

    while let Some(index) = queue.pop_front() {
        if index == design.len() {
             return true;
        }

        for pattern in patterns {
            // println!("Checking pattern {pattern} at index {index}");
            let end_of_check = index+pattern.len();

            if end_of_check > design.len() {
                // println!("Design is too long!");
                // Too long, wouldn't work.
                continue;
            }

            if design[index..end_of_check] == *pattern {
                // println!("Design could work, moving to index {end_of_check}!");
                queue.push_front(end_of_check);
            }
        }
    }

    false
}


fn part_1(file_path: String) -> i32 {
    let (patterns, designs) = parse_data(file_path);

    let mut result = 0;

    for design in designs {
        if design_can_be_created(design, &patterns) {
            result += 1;
        }
    }

    result
}

fn part_2(file_path: String) -> i32 {
    let (patterns, designs) = parse_data(file_path);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 143)]
    #[case(false, 5588)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 5, None)));
        assert_eq!(expected, alternative(get_file_path(is_test, 5, None), true));
    }

    #[rstest]
    #[case(true, 123)]
    #[case(false, 5331)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 5, None)));
        assert_eq!(
            expected,
            alternative(get_file_path(is_test, 5, None), false)
        );
    }
}
