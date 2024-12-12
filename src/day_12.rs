use crate::file_utilities::read_lines;

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

fn get_regions(map: Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let map_size = map.len() as isize;

    let mut regions: Vec<Vec<(usize, usize)>> = vec![];

    let mut nodes_to_explore = VecDeque::new();
    nodes_to_explore.push_back((0, 0));

    let mut explored = HashSet::new();

    // External search to go over the whole map
    while let Some(node_to_explore) = nodes_to_explore.pop_front() {
        if explored.contains(&node_to_explore) {
            continue;
        }

        explored.insert(node_to_explore);

        let (explore_row, explore_column) = node_to_explore;
        let explore_char = map[explore_row][explore_column];

        // Internal flood-fill
        let mut flood_fill_queue = VecDeque::new();
        flood_fill_queue.push_back((explore_row, explore_column));

        let mut flood_fill_explored = HashSet::new();

        let mut region = vec![];

        while let Some(node_for_flood_fill) = flood_fill_queue.pop_front() {
            if flood_fill_explored.contains(&node_for_flood_fill) {
                continue;
            }

            let (flood_fill_row, flood_fill_column) = node_for_flood_fill;

            let flood_fill_char = map[flood_fill_row][flood_fill_column];

            if flood_fill_char != explore_char {
                // This is for another search...
                // println!("I found ({flood_fill_row}, {flood_fill_column}) but it's for letter {flood_fill_char} and I'm in letter {explore_char}");
                nodes_to_explore.push_front((flood_fill_row, flood_fill_column));
                continue;
            }

            region.push((flood_fill_row, flood_fill_column));
            explored.insert((flood_fill_row, flood_fill_column));
            flood_fill_explored.insert((flood_fill_row, flood_fill_column));

            let neighbours = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (neighbour_row, neighbour_column) in neighbours {
                let new_row = flood_fill_row as isize + neighbour_row as isize;
                let new_column = flood_fill_column as isize + neighbour_column as isize;

                if new_row < 0 || new_row >= map_size || new_column < 0 || new_column >= map_size {
                    continue;
                }

                flood_fill_queue.push_back((new_row as usize, new_column as usize));
            }
        }

        // println!("{region:?} for letter {explore_char}");
        regions.push(region);
    }
    regions
}

fn part_1(file_path: String) -> u64 {
    let map = parse_data(file_path);
    let regions = get_regions(map);

    let mut result = 0;
    for region in regions {
        let region_size = region.len();
        let mut fence_size = 0;

        let region_hashset = region
            .iter()
            .copied()
            .map(|(row, column)| (row as isize, column as isize))
            .collect::<HashSet<_>>();

        for (node_row, node_column) in region {
            let neighbour_count = [(0, 1), (1, 0), (0, -1), (-1, 0)]
                .into_iter()
                .map(|(d_row, d_column)| {
                    (node_row as isize + d_row, node_column as isize + d_column)
                })
                .filter(|node| !region_hashset.contains(node))
                .count();

            fence_size += neighbour_count;
        }

        result += region_size * fence_size;
    }

    result as u64
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn part_2(file_path: String) -> u64 {
    let map = parse_data(file_path);
    let regions = get_regions(map);

    let mut result = 0;
    for region in regions {
        let region_size = region.len();
        let region_hashset = region
            .iter()
            .copied()
            .map(|(row, column)| (row as isize, column as isize))
            .collect::<HashSet<_>>();

        let mut fence_points = HashSet::new();
        let mut fence_edges = HashSet::new();

        for (node_row, node_column) in region {
            let (node_row, node_column) = (node_row as isize, node_column as isize);
            let up_neighbour = (node_row - 1, node_column);
            let down_neighbour = (node_row + 1, node_column);
            let left_neighbour = (node_row, node_column - 1);
            let right_neighbour = (node_row, node_column + 1);

            if !region_hashset.contains(&up_neighbour) {
                fence_points.insert((node_row, node_column));
                fence_points.insert((node_row, node_column + 1));
                fence_edges.insert((
                    (node_row, node_column),
                    (node_row, node_column + 1),
                    Direction::Up,
                ));
            }

            if !region_hashset.contains(&down_neighbour) {
                fence_points.insert((node_row + 1, node_column));
                fence_points.insert((node_row + 1, node_column + 1));
                fence_edges.insert((
                    (node_row + 1, node_column),
                    (node_row + 1, node_column + 1),
                    Direction::Down,
                ));
            }
            if !region_hashset.contains(&left_neighbour) {
                fence_points.insert((node_row, node_column));
                fence_points.insert((node_row + 1, node_column));
                fence_edges.insert((
                    (node_row, node_column),
                    (node_row + 1, node_column),
                    Direction::Left,
                ));
            }

            if !region_hashset.contains(&right_neighbour) {
                fence_points.insert((node_row, node_column + 1));
                fence_points.insert((node_row + 1, node_column + 1));
                fence_edges.insert((
                    (node_row, node_column + 1),
                    (node_row + 1, node_column + 1),
                    Direction::Right,
                ));
            }
        }

        // println!("Fence points for region {region_hashset:?} are {fence_points:?}");
        // println!("Fence edges for region {region_hashset:?} are {fence_edges:?}");
        let mut combined_fence_edges = HashSet::new();
        let mut fence_points_vec = fence_points.iter().copied().collect::<Vec<_>>();

        while let Some(point) = fence_points_vec.pop() {
            // println!("Checking point {point:?}");
            for direction in [Direction::Up, Direction::Down] {
                // Go left
                let mut left_point = point;
                while fence_edges.contains(&(
                    (left_point.0, left_point.1 - 1),
                    left_point,
                    direction,
                )) {
                    left_point = (left_point.0, left_point.1 - 1);
                }
                // println!("Went left to point {left_point:?}");

                // Go right
                let mut right_point = point;
                while fence_edges.contains(&(
                    right_point,
                    (right_point.0, right_point.1 + 1),
                    direction,
                )) {
                    right_point = (right_point.0, right_point.1 + 1);
                }
                // println!("Went right to point {right_point:?}");

                if right_point != left_point {
                    combined_fence_edges.insert((left_point, right_point));
                }
            }

            for direction in [Direction::Right, Direction::Left] {
                // Go up
                let mut up_point = point;
                while fence_edges.contains(&((up_point.0 - 1, up_point.1), up_point, direction)) {
                    up_point = (up_point.0 - 1, up_point.1);
                }
                // println!("Went up to point {up_point:?}");

                // Go down
                let mut down_point = point;
                while fence_edges.contains(&(
                    down_point,
                    (down_point.0 + 1, down_point.1),
                    direction,
                )) {
                    down_point = (down_point.0 + 1, down_point.1);
                }
                // println!("Went down to point {down_point:?}");

                if up_point != down_point {
                    combined_fence_edges.insert((up_point, down_point));
                }
            }
        }

        // println!("Combined fence edges for region {region_hashset:?} are {combined_fence_edges:?}");
        // println!("So I'm going to add {} and {} for {}", region_size, combined_fence_edges.len(), region_size * combined_fence_edges.len());
        result += region_size * combined_fence_edges.len();
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
