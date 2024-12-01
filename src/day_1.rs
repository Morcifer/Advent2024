use counter::Counter;

use crate::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> (i32, i32) {
    let values = line
        .split("   ")
        .map(str::trim)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    (values[0], values[1])
}

fn parse_data(file_path: String) -> (Vec<i32>, Vec<i32>) {
    let all_values = read_lines(file_path)
        .into_iter()
        .map(parse_line_to_int)
        .collect::<Vec<(i32, i32)>>();

    all_values.into_iter().unzip()
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
    let (mut list1, mut list2) = parse_data(file_path);

    list1.sort();
    list2.sort();

    list1
        .into_iter()
        .zip(list2)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_2(file_path: String) -> i32 {
    let (list1, list2) = parse_data(file_path);
    let counter = list2.into_iter().collect::<Counter<_>>();

    list1
        .into_iter()
        .map(|item1| item1 * (counter[&item1] as i32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 11)]
    #[case(false, 2756096)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 1, None)));
    }

    #[rstest]
    #[case(true, 31)]
    #[case(false, 23117829)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 1, None)));
    }
}
