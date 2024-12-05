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

fn parse_data(file_path: String) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
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

fn part_1(file_path: String) -> i32 {
    let (rules, page_lists) = parse_data(file_path);

    let mut result = 0;

    for pages in page_lists {
        let mut page_map: [Option<usize>; 100] = [None; 100];
        pages
            .iter()
            .copied()
            .enumerate()
            .for_each(|(index, page)| page_map[page] = Some(index));

        let mut is_valid = true;

        for rule in &rules {
            if page_map[rule.0].is_none() || page_map[rule.1].is_none() {
                continue;
            }

            is_valid = is_valid && page_map[rule.0].unwrap() < page_map[rule.1].unwrap();
        }

        if is_valid {
            result += pages[pages.len() / 2] as i32;
        }
    }

    result
}

fn part_2(file_path: String) -> i32 {
    let (rules, page_lists) = parse_data(file_path);

    let mut result = 0;

    for pages in page_lists {
        let mut pages = pages;

        let mut page_map: [Option<usize>; 100] = [None; 100];
        pages
            .iter()
            .copied()
            .enumerate()
            .for_each(|(index, page)| page_map[page] = Some(index));

        let mut was_changed = false;

        loop {
            let mut is_valid = true;

            for rule in &rules {
                if page_map[rule.0].is_none() || page_map[rule.1].is_none() {
                    continue;
                }

                let index_0 = page_map[rule.0].unwrap();
                let index_1 = page_map[rule.1].unwrap();

                if index_0 < index_1 {
                    continue;
                }

                pages[index_0] = rule.1;
                pages[index_1] = rule.0;

                page_map[rule.0] = Some(index_1);
                page_map[rule.1] = Some(index_0);

                is_valid = false;
                was_changed = true;
                break;
            }

            if is_valid {
                break;
            }
        }

        if was_changed {
            result += pages[pages.len() / 2] as i32;
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
