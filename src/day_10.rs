use crate::file_utilities::read_lines;
use itertools::Itertools;

use std::collections::{HashMap, VecDeque};
use std::iter;

fn parse_line_to_numbers(line: String) -> Vec<usize> {
    line.chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}

fn parse_data(file_path: String) -> Vec<Vec<usize>> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_numbers)
        .collect()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> u64 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

type Point = (isize, isize);

fn get_trails_per_origin(
    map: Vec<Vec<usize>>,
) -> HashMap<Point, Vec<Vec<Point>>> {
    let neighbours = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let map_size = map.len() as isize;

    let mut trails: HashMap<Point, Vec<Vec<Point>>> = HashMap::new(); // origin -> list of trails

    let mut queue = map
        .iter()
        .enumerate()
        .flat_map(move |(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, h)| **h == 0)
                .map(move |(j, h)| (i as isize, j as isize, *h, vec![(i as isize, j as isize)]))
        })
        .collect::<VecDeque<_>>();

    // println!("Starting queue {queue:?}");

    while let Some((current_i, current_j, current_h, current_trail)) = queue.pop_front() {
        for (delta_i, delta_j) in &neighbours {
            let next_i = current_i + delta_i;
            let next_j = current_j + delta_j;

            if next_i < 0 || next_j < 0 || next_i >= map_size || next_j >= map_size {
                // Out of bounds for map.
                continue;
            }

            let next_h = map[next_i as usize][next_j as usize];

            if next_h != current_h + 1 {
                // Wrong height
                continue;
            }

            let next_trail = current_trail
                .iter()
                .copied()
                .chain(iter::once((next_i, next_j)))
                .collect::<Vec<_>>();

            if next_h == 9 {
                let origin = next_trail[0];
                // println!("New trail found {next_trail:?}");

                let values = trails.entry(origin).or_default();
                values.push(next_trail);

                continue;
            }

            queue.push_back((next_i, next_j, next_h, next_trail));
        }
    }

    trails
}

fn part_1(file_path: String) -> u64 {
    let map = parse_data(file_path);
    let trails = get_trails_per_origin(map);

    trails
        .into_values()
        .map(|trail_list| {
            trail_list
                .into_iter()
                .map(|trail| *trail.last().unwrap())
                .unique()
                .count()
        })
        .sum::<usize>() as u64
}

fn part_2(file_path: String) -> u64 {
    let map = parse_data(file_path);
    let trails = get_trails_per_origin(map);

    trails
        .into_values()
        .map(|trail_list| trail_list.len())
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 36)]
    #[case(false, 782)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 10, None)));
    }

    #[rstest]
    #[case(true, 81)]
    #[case(false, 1694)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 10, None)));
    }
}
