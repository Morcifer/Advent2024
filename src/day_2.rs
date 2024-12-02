use itertools::Itertools;

use crate::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> Vec<i32> {
    line.split(" ")
        .map(str::trim)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn parse_data(file_path: String) -> Vec<Vec<i32>> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_int)
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

fn is_safe(report: &[i32]) -> bool {
    let diffs = report
        .iter()
        .tuple_windows()
        .map(|(current, next)| next - current)
        .collect::<Vec<_>>();

    let increasing = diffs.iter().all(|diff| *diff > 0);
    let decreasing = diffs.iter().all(|diff| *diff < 0);

    let in_range = diffs.iter().all(|diff| (1..=3).contains(&diff.abs()));

    (decreasing || increasing) && in_range
}

fn is_safe_with_tolerance(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for index in 0..report.len() {
        let new_report = report
            .iter()
            .take(index)
            .chain(report.iter().skip(index + 1))
            .copied()
            .collect::<Vec<i32>>();

        if is_safe(&new_report) {
            return true;
        }
    }

    false
}

fn part_1(file_path: String) -> i32 {
    let reports = parse_data(file_path);
    reports.into_iter().filter(|report| is_safe(report)).count() as i32
}

fn part_2(file_path: String) -> i32 {
    let reports = parse_data(file_path);
    reports
        .into_iter()
        .filter(|report| is_safe_with_tolerance(report))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 2)]
    #[case(false, 220)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 2, None)));
    }

    #[rstest]
    #[case(true, 4)]
    #[case(false, 296)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 2, None)));
    }
}
