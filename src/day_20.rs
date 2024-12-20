use crate::file_utilities::read_lines;
use crate::map_utilities::{Point, DIRECTIONS};

use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

// TODO: Consider replacing HashMap with a vector that can be enumerated over.
// TODO: Consider iterating over the path twice instead of over the path * 41 * 41.

fn parse_line_to_chars(line: String) -> Vec<char> {
    line.chars().collect::<Vec<char>>()
}

fn parse_data(file_path: String) -> Vec<Vec<char>> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_chars)
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

fn get_distance_per_point(map: &[Vec<char>]) -> HashMap<Point, usize> {
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for (i, row) in map.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char == 'S' {
                start = Point::new(i as isize, j as isize);
            } else if *char == 'E' {
                end = Point::new(i as isize, j as isize);
            } else {
                continue;
            }
        }
    }

    let mut distance_per_point = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((point, distance_from_start)) = queue.pop_front() {
        if map[point.row()][point.column()] == '#' {
            continue;
        }

        if point == end {
            distance_per_point.insert(point, distance_from_start);
            continue;
        }

        if distance_per_point.contains_key(&point) {
            // We've been here before, no need to keep searching.
            continue;
        }

        distance_per_point.insert(point, distance_from_start);

        for direction in DIRECTIONS.into_iter() {
            let neighbour = point.unbound_neighbour(direction);
            queue.push_back((neighbour, distance_from_start + 1));
        }
    }

    distance_per_point
}

fn get_all_tunnels(
    distance_per_point: &HashMap<Point, usize>,
    time_limit: isize,
) -> Vec<(Point, Point, usize)> {
    let path = distance_per_point.keys().copied().collect_vec();

    let mut result = vec![];

    for point_in_path in &path {
        let cost_at_point = distance_per_point.get(point_in_path).unwrap();

        for new_point in &path {
            let delta_row = (point_in_path.row - new_point.row).abs();
            let delta_column = (point_in_path.column - new_point.column).abs();

            let distance_travelled = delta_row + delta_column;

            if distance_travelled > time_limit {
                // We can only get within 20 steps. Maybe 21? Who knows, we'll find out.
                continue;
            }

            let cost_at_new_point = distance_per_point.get(new_point).unwrap();

            if *cost_at_new_point > *cost_at_point + distance_travelled as usize {
                // println!("Jumping from {point_in_path:?} to {new_point:?} gives {cost_at_point} -> {cost_at_new_point}");
                result.push((
                    *point_in_path,
                    *new_point,
                    cost_at_new_point - cost_at_point - distance_travelled as usize,
                ));
            }
        }
    }

    result
}

fn part_1(file_path: String) -> u64 {
    let is_test = file_path.contains("test");
    let limit = if is_test { 0 } else { 100 };

    let map = parse_data(file_path);

    let distance_per_point = get_distance_per_point(&map);
    let all_tunnels = get_all_tunnels(&distance_per_point, 2);

    all_tunnels
        .into_iter()
        .filter(|(_p1, _p2, cut)| *cut >= limit)
        .count() as u64
}

fn part_2(file_path: String) -> u64 {
    let is_test = file_path.contains("test");
    let limit = if is_test { 50 } else { 100 };

    let map = parse_data(file_path);

    let distance_per_point = get_distance_per_point(&map);
    let all_tunnels = get_all_tunnels(&distance_per_point, 20);

    all_tunnels
        .into_iter()
        .filter(|(_p1, _p2, cut)| *cut >= limit)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 44)]
    #[case(false, 1445)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 20, None)));
    }

    #[rstest]
    #[case(true, 285)]
    #[case(false, 1008040)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 20, None)));
    }
}
