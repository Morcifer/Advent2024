use itertools::Itertools;
use std::collections::HashSet;

use crate::file_utilities::read_lines;

fn parse_data(file_path: String) -> (Vec<HashSet<(isize, isize)>>, isize) {
    let lines = read_lines(file_path);
    let size = lines.len() as isize;

    (
        lines
            .into_iter()
            .enumerate()
            .flat_map(move |(i, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_j, c)| *c != '.')
                    .map(move |(j, c)| (i, j, c))
                    .collect::<Vec<_>>()
            })
            .sorted_by_key(|&(_, _, c)| c)
            .chunk_by(|&(_, _, c)| c)
            .into_iter()
            .map(|(_key, chunk)| {
                chunk
                    .map(|(i, j, _c)| (i as isize, j as isize))
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>(),
        size,
    )
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> u64 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> u64 {
    let (antenna_sets, map_size) = parse_data(file_path);

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for antenna_set in antenna_sets {
        for antenna_from in &antenna_set {
            for antenna_to in &antenna_set {
                if antenna_from == antenna_to {
                    // Antenna to itself doesn't count!
                    continue;
                }

                let diff_i = antenna_from.0 - antenna_to.0;
                let diff_j = antenna_from.1 - antenna_to.1;

                let antinode_i = antenna_to.0 + 2 * diff_i;
                let antinode_j = antenna_to.1 + 2 * diff_j;

                if antinode_i >= 0
                    && antinode_j >= 0
                    && antinode_i < map_size
                    && antinode_j < map_size
                {
                    antinodes.insert((antinode_i, antinode_j));
                }
            }
        }
    }

    antinodes.into_iter().unique().count() as u64
}

fn part_2(file_path: String) -> u64 {
    let (antenna_sets, map_size) = parse_data(file_path);

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for antenna_set in antenna_sets {
        for antenna_from in &antenna_set {
            for antenna_to in &antenna_set {
                if antenna_from == antenna_to {
                    // Going from the same antenna to itself doesn't count!
                    continue;
                }

                let (antenna_from_i, antenna_from_j) = antenna_from;
                let (antenna_to_i, antenna_to_j) = antenna_to;

                let diff_i = antenna_from_i - antenna_to_i;
                let diff_j = antenna_from_j - antenna_to_j;

                for multiplier in 0..map_size {
                    let new_i = antenna_from_i + diff_i * multiplier;
                    let new_j = antenna_from_j + diff_j * multiplier;

                    if new_i < 0 || new_i >= map_size || new_j < 0 || new_j >= map_size {
                        // Out of bounds for the map.
                        break;
                    }

                    antinodes.insert((new_i, new_j));
                }
            }
        }
    }

    antinodes.into_iter().unique().count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 14)]
    #[case(false, 311)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 8, None)));
    }

    #[rstest]
    #[case(true, 34)]
    #[case(false, 1115)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 8, None)));
    }
}
