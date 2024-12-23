use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::file_utilities::read_lines;

fn parse_edge_line(line: String) -> (String, String) {
    let values = line.split("-").map(str::trim).collect_vec();

    (values[0].to_string(), values[1].to_string())
}

fn parse_data(file_path: String) -> Vec<(String, String)> {
    read_lines(file_path)
        .into_iter()
        .map(parse_edge_line)
        .collect_vec()
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
    let edges = parse_data(file_path);

    let nodes = edges
        .iter()
        .flat_map(|(source, target)| vec![source.clone(), target.clone()])
        .unique()
        .collect_vec();
    let mut hash_map = nodes
        .iter()
        .map(|node| (node.clone(), HashSet::new()))
        .collect::<HashMap<String, HashSet<String>>>();

    for (node_1, node_2) in edges.iter() {
        hash_map
            .get_mut(&node_1.clone())
            .unwrap()
            .insert(node_2.clone());
        hash_map
            .get_mut(&node_2.clone())
            .unwrap()
            .insert(node_1.clone());
    }

    // println!("{hash_map:?}");

    let mut result: HashSet<(String, String, String)> = HashSet::new();

    for node_1 in nodes.iter() {
        if !node_1.starts_with("t") {
            continue;
        }

        for node_2 in nodes.iter() {
            if !hash_map[node_1].contains(node_2) {
                continue;
            }

            for node_3 in nodes.iter() {
                if node_3 == node_1 || node_3 == node_2 {
                    continue;
                }

                if !hash_map[node_1].contains(node_3) {
                    continue;
                }

                if !hash_map[node_2].contains(node_3) {
                    continue;
                }

                let new = [node_1.clone(), node_2.clone(), node_3.clone()];

                if new
                    .iter()
                    .permutations(3)
                    .any(|p| result.contains(&p.iter().copied().cloned().collect_tuple().unwrap()))
                {
                    continue;
                }

                result.insert(new.into_iter().collect_tuple().unwrap());
            }
        }
    }

    // println!("{result:?}");
    result.into_iter().unique().count() as i32
}

fn part_2(file_path: String) -> i32 {
    let edges = parse_data(file_path);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 7)]
    #[case(false, 1200)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 23, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 23, None)));
    }
}
