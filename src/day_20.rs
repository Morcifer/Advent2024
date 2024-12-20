use crate::file_utilities::read_lines;
use crate::map_utilities::{Direction, Point, DIRECTIONS};

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

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

fn get_distance_per_point(map: &Vec<Vec<char>>) -> HashMap<Point, usize> {
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
    map: &Vec<Vec<char>>,
    distance_per_point: &HashMap<Point, usize>,
) -> Vec<(Point, Point, usize)> {
    let map_size = map.len();

    let mut result = vec![];

    for (i, row) in map.iter().enumerate() {
        if i == 0 || i == map_size - 1 {
            continue;
        }

        for (j, char) in row.iter().enumerate() {
            if j == 0 || j == map_size - 1 {
                continue;
            }

            if *char != '#' {
                // No point in shortcut
                continue;
            }

            let point = Point::new(i as isize, j as isize);

            let neighbour_up = point.unbound_neighbour(Direction::Up);
            let neighbour_down = point.unbound_neighbour(Direction::Down);

            if map[neighbour_up.row()][neighbour_up.column()] != '#' {
                if map[neighbour_down.row()][neighbour_down.column()] != '#' {
                    let cost_up = distance_per_point.get(&neighbour_up).unwrap();
                    let cost_down = distance_per_point.get(&neighbour_down).unwrap();

                    if cost_up < cost_down {
                        result.push((point, neighbour_down, cost_down - cost_up - 2));
                        continue;
                    }

                    if cost_up > cost_down {
                        result.push((point, neighbour_up, cost_up - cost_down - 2));
                        continue;
                    }
                }
            }

            let neighbour_left = point.unbound_neighbour(Direction::Left);
            let neighbour_right = point.unbound_neighbour(Direction::Right);

            if map[neighbour_left.row()][neighbour_left.column()] != '#' {
                if map[neighbour_right.row()][neighbour_right.column()] != '#' {
                    let cost_left = distance_per_point.get(&neighbour_left).unwrap();
                    let cost_right = distance_per_point.get(&neighbour_right).unwrap();

                    if cost_left < cost_right {
                        result.push((point, neighbour_right, cost_right - cost_left - 2));
                        continue;
                    }

                    if cost_left > cost_right {
                        result.push((point, neighbour_left, cost_left - cost_right - 2));
                        continue;
                    }
                }
            }
        }
    }

    result
}

fn part_1(file_path: String) -> u64 {
    let map = parse_data(file_path);

    let distance_per_point = get_distance_per_point(&map);
    let all_tunnels = get_all_tunnels(&map, &distance_per_point);

    // println!("Tunnels found:");
    //
    // for tunnel in all_tunnels.iter() {
    //     println!("{:?} -> {:?} cuts {:?}", tunnel.0, tunnel.1, tunnel.2);
    // }

    all_tunnels
        .into_iter()
        .filter(|(_p1, _p2, cut)| *cut >= 100)
        .count() as u64
}

fn part_2(file_path: String) -> u64 {
    let _map = parse_data(file_path);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 0)]
    #[case(false, 1445)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 20, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 20, None)));
    }
}
