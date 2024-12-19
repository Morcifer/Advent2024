use std::collections::{HashMap, VecDeque};

use crate::file_utilities::read_two_chunks;

fn parse_data(file_path: String) -> (Vec<String>, Vec<String>) {
    let (patterns_line, designs) = read_two_chunks(file_path);

    let patterns = patterns_line[0]
        .split(",")
        .map(str::trim)
        .map(|s| s.to_string())
        .collect();

    (patterns, designs)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> usize {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

#[allow(dead_code)]
fn create_design(design: &String, patterns: &Vec<String>, stop_on_first: bool) -> usize {
    let mut result = 0;

    println!("Design: {design}");
    let mut queue = VecDeque::new();
    queue.push_front(0);

    while let Some(index) = queue.pop_front() {
        if index == design.len() {
            result += 1;

            if stop_on_first {
                return result;
            }
        }

        for pattern in patterns {
            // println!("Checking pattern {pattern} at index {index}");
            let end_of_check = index + pattern.len();

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

    result
}

fn create_design_recursive(design: &String, patterns: &Vec<String>, index: usize) -> usize {
    if index == design.len() {
        return 1;
    }

    let mut result = 0;

    for pattern in patterns {
        // println!("Checking pattern {pattern} at index {index}");
        let end_of_check = index + pattern.len();

        if end_of_check > design.len() {
            // println!("Design is too long!");
            // Too long, wouldn't work.
            continue;
        }

        if design[index..end_of_check] == *pattern {
            // println!("Design could work, moving to index {end_of_check}!");
            result += create_design_recursive(design, patterns, end_of_check);
        }
    }

    result
}

fn part_1(file_path: String) -> usize {
    let (patterns, designs) = parse_data(file_path);

    designs
        .into_iter()
        // .filter(|design| design_can_be_created(design, &patterns, true) > 0)
        .filter(|design| create_design_recursive(design, &patterns, 0) > 0)
        .count()
}

fn part_2(file_path: String) -> usize {
    let (patterns, designs) = parse_data(file_path);

    designs
        .into_iter()
        // .map(|design| design_can_be_created(design, &patterns, false))
        .map(|design| create_design_recursive(&design, &patterns, 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 6)]
    #[case(false, 340)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 19, None)));
    }

    #[rstest]
    #[case(true, 16)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 19, None)));
    }
}
