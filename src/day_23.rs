use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::file_utilities::read_lines;

// TODO: Have part 1 use part 2 by going over clusters that are >= 3 and summing over n-choose-3.

fn parse_edge_line(line: String) -> (String, String) {
    line.split("-")
        .map(str::trim)
        .map(|s| s.to_string())
        .collect_tuple()
        .unwrap()
}

fn parse_data(file_path: String) -> (Vec<String>, HashMap<String, HashSet<String>>) {
    let input_edges = read_lines(file_path)
        .into_iter()
        .map(parse_edge_line)
        .collect_vec();

    let nodes = input_edges
        .iter()
        .flat_map(|(source, target)| vec![source.clone(), target.clone()])
        .unique()
        .collect_vec();

    let mut edges = nodes
        .iter()
        .map(|node| (node.clone(), HashSet::new()))
        .collect::<HashMap<String, HashSet<String>>>();

    for (node_1, node_2) in input_edges.iter() {
        edges
            .get_mut(&node_1.clone())
            .unwrap()
            .insert(node_2.clone());
        edges
            .get_mut(&node_2.clone())
            .unwrap()
            .insert(node_1.clone());
    }

    (nodes, edges)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> String {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> String {
    let (nodes, edges) = parse_data(file_path);

    let mut result: HashSet<(String, String, String)> = HashSet::new();

    for node_1 in nodes.iter() {
        if !node_1.starts_with("t") {
            continue;
        }

        for node_2 in nodes.iter() {
            if !edges[node_1].contains(node_2) {
                continue;
            }

            for node_3 in nodes.iter() {
                if node_3 == node_1 || node_3 == node_2 {
                    continue;
                }

                if !edges[node_1].contains(node_3) {
                    continue;
                }

                if !edges[node_2].contains(node_3) {
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
    result.into_iter().unique().count().to_string()
}

fn bron_kerbosch(
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    edges: &HashMap<String, HashSet<String>>,
    maximal_clique: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        maximal_clique.push(r.clone());
        return;
    }

    let all_vertices_in_p: Vec<String> = p.iter().cloned().collect_vec();

    for v in all_vertices_in_p.iter() {
        let v_set: HashSet<String> = vec![v.clone()].into_iter().collect();
        let neighbours = edges[v].clone();

        let mut new_r: HashSet<String> = r.union(&v_set).cloned().collect(); // Don't format!
        let mut new_p: HashSet<String> = p.intersection(&neighbours).cloned().collect(); //
        let mut new_x: HashSet<String> = x.intersection(&neighbours).cloned().collect(); //

        bron_kerbosch(&mut new_r, &mut new_p, &mut new_x, edges, maximal_clique);

        p.remove(&v.clone());
        x.insert(v.clone());
    }
}

fn part_2(file_path: String) -> String {
    let (nodes, edges) = parse_data(file_path);

    // Solve the clique problem!
    let mut p: HashSet<String> = nodes.iter().cloned().collect();

    let mut r = HashSet::new();
    let mut x = HashSet::new();

    let mut result = vec![];

    bron_kerbosch(&mut r, &mut p, &mut x, &edges, &mut result);

    // TODO: Is there a proper way to have max_by_key but for an iterator but without sorting?
    let max_length = result.iter().map(|r| r.len()).max().unwrap();

    for clique in result {
        if clique.len() == max_length {
            return clique.into_iter().sorted().join(",");
        }
    }

    "Oh no!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, "7")]
    #[case(false, "1200")]
    fn test_part_1(#[case] is_test: bool, #[case] expected: String) {
        assert_eq!(expected, part_1(get_file_path(is_test, 23, None)));
    }

    #[rstest]
    #[case(true, "co,de,ka,ta")]
    #[case(false, "ag,gh,hh,iv,jx,nq,oc,qm,rb,sm,vm,wu,zr")]
    fn test_part_2(#[case] is_test: bool, #[case] expected: String) {
        assert_eq!(expected, part_2(get_file_path(is_test, 23, None)));
    }
}
