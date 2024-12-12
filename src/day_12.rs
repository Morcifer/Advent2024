use crate::file_utilities::read_lines;
use crate::map_utilities::{DIRECTIONS, Direction, Point};

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

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

fn flood_fill_for_region(
    map: &[Vec<char>],
    current_node_to_explore: (Point, char),
    nodes_to_explore: &mut VecDeque<Point>,
    explored: &mut HashSet<Point>,
) -> Vec<Point> {
    let map_size = map.len();

    let (explore_point, explore_char) = current_node_to_explore;

    let mut flood_fill_queue = VecDeque::new();
    flood_fill_queue.push_back(explore_point);

    let mut flood_fill_explored = HashSet::new();

    let mut region = vec![];

    while let Some(flood_fill_point) = flood_fill_queue.pop_front() {
        if flood_fill_explored.contains(&flood_fill_point) {
            continue;
        }

        let flood_fill_char = map[flood_fill_point.row()][flood_fill_point.column()];

        if flood_fill_char != explore_char {
            // This will have to be saved for another search...
            nodes_to_explore.push_back(flood_fill_point);
            continue;
        }

        region.push(flood_fill_point);
        explored.insert(flood_fill_point);
        flood_fill_explored.insert(flood_fill_point);

        let neighbours = DIRECTIONS
            .into_iter()
            .filter_map(|direction| flood_fill_point.neighbour(direction, map_size))
            .collect::<Vec<Point>>();

        flood_fill_queue.extend(neighbours);
    }

    region
}

fn get_regions(map: &[Vec<char>]) -> Vec<Vec<Point>> {
    let mut regions: Vec<Vec<Point>> = vec![];

    let mut nodes_to_explore = VecDeque::new();
    nodes_to_explore.push_back(Point::new(0, 0));

    let mut explored = HashSet::new();

    // External search to go over the whole map
    while let Some(node_to_explore) = nodes_to_explore.pop_front() {
        if explored.contains(&node_to_explore) {
            continue;
        }

        explored.insert(node_to_explore);

        let explore_char = map[node_to_explore.row()][node_to_explore.column()];

        let region = flood_fill_for_region(
            map,
            (node_to_explore, explore_char),
            &mut nodes_to_explore,
            &mut explored,
        );

        // println!("{region:?} for letter {explore_char}");
        regions.push(region);
    }

    regions
}

fn get_edges(region: &Vec<Point>) -> HashSet<(Point, Point, Direction)> {
    let region_hashset = region
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    let mut fence_edges = HashSet::new();

    for region_point in region {
        let (node_row, node_column) = (region_point.row() as isize, region_point.column() as isize);

        let up_neighbour = region_point.unbound_neighbour(Direction::Up);
        let down_neighbour = region_point.unbound_neighbour(Direction::Down);
        let left_neighbour = region_point.unbound_neighbour(Direction::Left);
        let right_neighbour = region_point.unbound_neighbour(Direction::Right);

        if !region_hashset.contains(&up_neighbour) {
            fence_edges.insert((
                Point::new(node_row, node_column),
                Point::new(node_row, node_column + 1),
                Direction::Up,
            ));
        }

        if !region_hashset.contains(&down_neighbour) {
            fence_edges.insert((
                Point::new(node_row + 1, node_column),
                Point::new(node_row + 1, node_column + 1),
                Direction::Down,
            ));
        }
        if !region_hashset.contains(&left_neighbour) {
            fence_edges.insert((
                Point::new(node_row, node_column),
                Point::new(node_row + 1, node_column),
                Direction::Left,
            ));
        }

        if !region_hashset.contains(&right_neighbour) {
            fence_edges.insert((
                Point::new(node_row, node_column + 1),
                Point::new(node_row + 1, node_column + 1),
                Direction::Right,
            ));
        }
    }

    fence_edges
}

fn part_1(file_path: String) -> u64 {
    let map = parse_data(file_path);
    let regions = get_regions(&map);

    let mut result = 0;
    for region in regions {
        let fence_edges = get_edges(&region);

        result += region.len() * fence_edges.len();
    }

    result as u64
}

fn part_2(file_path: String) -> u64 {
    let map = parse_data(file_path);
    let regions = get_regions(&map);

    let mut result = 0;
    for region in regions {
        let fence_edges = get_edges(&region);

        let mut fence_points = fence_edges
            .iter()
            .copied()
            .flat_map(|(from, to, _)| vec![from, to])
            .unique()
            .collect::<Vec<_>>();

        let mut combined_fence_edges = HashSet::new();

        while let Some(point) = fence_points.pop() {
            for direction in [Direction::Up, Direction::Down] {
                // Go left
                let mut left_point = point;
                while fence_edges.contains(&(
                    Point::new(left_point.row, left_point.column - 1),
                    left_point,
                    direction,
                )) {
                    left_point = Point::new(left_point.row, left_point.column - 1);
                }

                // Go right
                let mut right_point = point;
                while fence_edges.contains(&(
                    right_point,
                    Point::new(right_point.row, right_point.column + 1),
                    direction,
                )) {
                    right_point = Point::new(right_point.row, right_point.column + 1);
                }

                if right_point != left_point {
                    combined_fence_edges.insert((left_point, right_point));
                }
            }

            for direction in [Direction::Right, Direction::Left] {
                // Go up
                let mut up_point = point;
                while fence_edges.contains(&(Point::new(up_point.row - 1, up_point.column), up_point, direction)) {
                    up_point = Point::new(up_point.row - 1, up_point.column);
                }

                // Go down
                let mut down_point = point;
                while fence_edges.contains(&(
                    down_point,
                    Point::new(down_point.row + 1, down_point.column),
                    direction,
                )) {
                    down_point = Point::new(down_point.row + 1, down_point.column);
                }

                if up_point != down_point {
                    combined_fence_edges.insert((up_point, down_point));
                }
            }
        }

        result += region.len() * combined_fence_edges.len();
    }

    result as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 1930)]
    #[case(false, 1488414)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 12, None)));
    }

    #[rstest]
    #[case(true, 1206)]
    #[case(false, 911750)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 12, None)));
    }
}
