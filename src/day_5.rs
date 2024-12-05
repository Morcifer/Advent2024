use std::collections::{HashMap, HashSet};

use crate::file_utilities::read_two_chunks;

fn parse_rule_line(line: String) -> (i32, i32) {
    let values = line
        .split("|")
        .map(str::trim)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    (values[0], values[1])
}

fn parse_pages_line(line: String) -> Vec<i32> {
    line.split(",")
        .map(str::trim)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn parse_data(file_path: String) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules_lines, pages_lines) = read_two_chunks(file_path);

    let rules = rules_lines
        .into_iter()
        .map(parse_rule_line)
        .collect::<Vec<_>>();
    let pages = pages_lines
        .into_iter()
        .map(parse_pages_line)
        .collect::<Vec<_>>();

    (rules, pages)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> i32 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn is_order_valid(rules: &Vec<(i32, i32)>, pages: &[i32]) -> bool {
    let page_set = pages.iter().copied().collect::<HashSet<i32>>();
    let page_map = pages
        .iter()
        .copied()
        .enumerate()
        .map(|(index, page)| (page, index))
        .collect::<HashMap<i32, usize>>();

    let mut is_valid = true;

    for rule in rules {
        if !page_set.contains(&rule.0) || !page_set.contains(&rule.1) {
            continue;
        }

        is_valid = is_valid && page_map[&rule.0] < page_map[&rule.1];
    }

    is_valid
}

fn part_1(file_path: String) -> i32 {
    let (rules, page_lists) = parse_data(file_path);

    let mut result = 0;

    for pages in page_lists {
        if is_order_valid(&rules, &pages) {
            result += pages[pages.len() / 2];
        }
    }

    result
}

fn part_2(file_path: String) -> i32 {
    let (rules, page_lists) = parse_data(file_path);

    let mut result = 0;

    for pages in page_lists {
        if is_order_valid(&rules, &pages) {
            continue;
        }

        let mut pages = pages;

        while !is_order_valid(&rules, &pages) {
            let page_set = pages.iter().copied().collect::<HashSet<i32>>();
            let page_map = pages
                .iter()
                .copied()
                .enumerate()
                .map(|(index, page)| (page, index))
                .collect::<HashMap<i32, usize>>();

            for rule in &rules {
                if !page_set.contains(&rule.0) || !page_set.contains(&rule.1) {
                    continue;
                }

                if page_map[&rule.0] < page_map[&rule.1] {
                    continue;
                }

                pages[page_map[&rule.0]] = rule.1;
                pages[page_map[&rule.1]] = rule.0;

                break;
            }
        }

        if is_order_valid(&rules, &pages) {
            result += pages[pages.len() / 2];
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
    #[case(true, 143)]
    #[case(false, 5588)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 5, None)));
    }

    #[rstest]
    #[case(true, 123)]
    #[case(false, 5331)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 5, None)));
    }
}
