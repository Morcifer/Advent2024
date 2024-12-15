use crate::file_utilities::read_two_chunks;
use crate::map_utilities::{Direction, Point};
use std::collections::HashSet;

use itertools::Itertools;

struct Map {
    robot: Point,
    walls: HashSet<Point>,
    boxes: HashSet<Point>,
    large_boxes_lefts: HashSet<Point>,
    large_boxes_rights: HashSet<Point>,
}

impl Map {
    fn new() -> Self {
        Self {
            robot: Point::new(0, 0),
            walls: HashSet::new(),
            boxes: HashSet::new(),
            large_boxes_lefts: HashSet::new(),
            large_boxes_rights: HashSet::new(),
        }
    }

    fn print_to_console(&self) {
        let max_row = self.walls.iter().map(|w| w.row).max().unwrap();
        let max_column = self.walls.iter().map(|w| w.column).max().unwrap();

        let mut map = (0..=max_row).map(|_| (0..=max_column).map(|_| '.').collect_vec()).collect_vec();

        for wall in self.walls.iter() {
            map[wall.row as usize][wall.column as usize] = '#';
        }

        for box_ in self.boxes.iter() {
            map[box_.row as usize][box_.column as usize] = 'O';
        }

        for box_ in self.large_boxes_lefts.iter() {
            map[box_.row as usize][box_.column as usize] = '[';
        }

        for box_ in self.large_boxes_rights.iter() {
            map[box_.row as usize][box_.column as usize] = ']';
        }

        map[self.robot.row as usize][self.robot.column as usize] = '@';

        for row in map {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn print_larger_map(&self) {

    }

    fn make_map_bigger(&mut self) {
        self.robot = Point::new(self.robot.row, 2 * self.robot.column);

        self.walls = self
            .walls
            .iter()
            .copied()
            .flat_map(|point| {
                vec![
                    Point::new(point.row, 2 * point.column),
                    Point::new(point.row, 2 * point.column + 1),
                ]
            })
            .collect();

        self.large_boxes_lefts = self
            .boxes
            .iter()
            .copied()
            .map(|point| Point::new(point.row, 2 * point.column))
            .collect();

        self.large_boxes_rights = self
            .boxes
            .iter()
            .copied()
            .map(|point| Point::new(point.row, 2 * point.column + 1))
            .collect();

        self.boxes.clear();
    }

    fn move_robot(&mut self, direction: Direction) {
        let desired_spot = self.robot.unbound_neighbour(direction);

        // Wall - we're done.
        if self.walls.contains(&desired_spot) {
            return;
        }

        // No wall, and no box - we move.
        if !self.boxes.contains(&desired_spot) {
            self.robot = desired_spot;
            return;
        }

        // Box - keep going until there's no more box.
        let mut target = desired_spot;

        while self.boxes.contains(&target) {
            target = target.unbound_neighbour(direction);
        }

        // If target is a wall, you're stuck.
        if self.walls.contains(&target) {
            return;
        }

        // If target is free, you move the desired_spot box there and move the robot.
        self.boxes.remove(&desired_spot);
        self.boxes.insert(target);
        self.robot = desired_spot;
    }

    fn move_big_robot(&mut self, direction: Direction) {
        // if direction == Direction::Up {
        //     println!("Stop here please!");
        // }
        let desired_spot = self.robot.unbound_neighbour(direction);

        // Wall - we're done.
        if self.walls.contains(&desired_spot) {
            return;
        }

        // No wall, and no box - we move.
        if !self.large_boxes_lefts.contains(&desired_spot)
            && !self.large_boxes_rights.contains(&desired_spot)
        {
            self.robot = desired_spot;
            return;
        }

        // Box - keep going until there's no more boxes.
        let mut spots_to_move = vec![]; // Vector of hashsets, might not need it.
        let mut box_lefts_to_move = HashSet::new();

        match direction {
            Direction::Right => spots_to_move.push(vec![desired_spot.unbound_neighbour(Direction::Right)].into_iter().collect::<HashSet<_>>()),
            Direction::Left => spots_to_move.push(vec![desired_spot.unbound_neighbour(Direction::Left)].into_iter().collect::<HashSet<_>>()),
            _ => {
                if self.large_boxes_lefts.contains(&desired_spot) {
                    spots_to_move.push(vec![desired_spot, desired_spot.unbound_neighbour(Direction::Right)].into_iter().collect::<HashSet<_>>());
                    // box_lefts_to_move.push(desired_spot);
                } else {
                    spots_to_move.push(vec![desired_spot, desired_spot.unbound_neighbour(Direction::Left)].into_iter().collect::<HashSet<_>>());
                    // box_lefts_to_move.push(desired_spot.unbound_neighbour(Direction::Left));
                };
            }
        }

        let mut last_spots_to_move = spots_to_move.last().unwrap().clone();

        while last_spots_to_move.iter().any(|spot| self.large_boxes_lefts.contains(&spot)|| self.large_boxes_rights.contains(&spot)) {
            // println!("Need to move spots {last_spots_to_move:?} to {direction:?}");

            let mut new_spots_to_move = HashSet::new();

            for spot in last_spots_to_move {
                // Update left-sides for spots
                if self.large_boxes_lefts.contains(&spot) {
                    box_lefts_to_move.insert(spot);
                } else if self.large_boxes_rights.contains(&spot){
                    box_lefts_to_move.insert(spot.unbound_neighbour(Direction::Left));
                }

                match direction {
                    Direction::Right => {
                        new_spots_to_move.insert(spot.unbound_neighbour(Direction::Right));
                    },
                    Direction::Left => {
                        new_spots_to_move.insert(spot.unbound_neighbour(Direction::Left));
                    },
                    _ => {
                        if self.large_boxes_lefts.contains(&spot) {
                            new_spots_to_move.insert(spot.unbound_neighbour(direction));
                            new_spots_to_move.insert(spot.unbound_neighbour(direction).unbound_neighbour(Direction::Right));
                        } else if self.large_boxes_rights.contains(&spot) {
                            new_spots_to_move.insert(spot.unbound_neighbour(direction));
                            new_spots_to_move.insert(spot.unbound_neighbour(direction).unbound_neighbour(Direction::Left));
                        };
                    }
                }

                // // And update for new spots.
                // let new_spot = spot.unbound_neighbour(direction);
                // new_spots_to_move.insert(new_spot);
            }

            spots_to_move.push(new_spots_to_move.clone());
            last_spots_to_move = new_spots_to_move.clone();
        }

        if spots_to_move.into_iter().flatten().any(|spot| self.walls.contains(&spot)) {
            // any sort of wall would get us stuck.
            // println!("Hit a wall when going {direction:?}");
            return;
        } else {
            // println!("Can indeed go {direction:?}");
        }

        // If target is free, you move the desired_spot box there and move the robot.
        // But when moving the boxes, do it in reverse or you'll be in trouble, I think.
        self.robot = desired_spot;

        let old_box_lefts_to_move = box_lefts_to_move.clone();
        // println!("Need to move left-box {old_box_lefts_to_move:?}");

        let old_box_rights_to_move = box_lefts_to_move.iter().copied().map(|spot| spot.unbound_neighbour(Direction::Right)).collect_vec();

        let new_box_lefts = box_lefts_to_move
            .into_iter()
            .map(|spot| spot.unbound_neighbour(direction))
            .collect_vec();
        // println!("Will move them to {new_box_lefts:?}");

        let new_box_rights = new_box_lefts.iter().copied().map(|spot| spot.unbound_neighbour(Direction::Right)).collect_vec();

        for old in old_box_lefts_to_move {
            self.large_boxes_lefts.remove(&old);
        }
        for old in old_box_rights_to_move {
            self.large_boxes_rights.remove(&old);
        }

        for new in new_box_lefts {
            self.large_boxes_lefts.insert(new);
        }
        for new in new_box_rights {
            self.large_boxes_rights.insert(new);
        }
    }
}

fn parse_data(file_path: String) -> (Map, Vec<Direction>) {
    let (map_lines, direction_lines) = read_two_chunks(file_path);

    let mut map = Map::new();

    for (row_index, row) in map_lines.into_iter().enumerate() {
        for (column_index, character) in row.chars().enumerate() {
            match character {
                '.' => continue,
                '#' => {
                    map.walls
                        .insert(Point::new(row_index as isize, column_index as isize));
                }
                'O' => {
                    map.boxes
                        .insert(Point::new(row_index as isize, column_index as isize));
                }
                '@' => map.robot = Point::new(row_index as isize, column_index as isize),
                _ => panic!("Got an invalid map thingie."),
            }
        }
    }

    let directions = direction_lines
        .into_iter()
        .flat_map(move |line| line.chars().collect_vec())
        .map(|c| match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Got an invalid movement."),
        })
        .collect_vec();

    (map, directions)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> usize {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> usize {
    let (mut map, directions) = parse_data(file_path);

    // println!("robot {:?}, walls {:?}, boxes {:?}", map.robot, map.walls, map.boxes);
    // println!("{directions:?}");
    map.print_to_console();

    for direction in directions {
        map.move_robot(direction);
    }

    map.boxes
        .into_iter()
        .map(|point| (point.row * 100) as usize + point.column as usize)
        .sum()
}

fn part_2(file_path: String) -> usize {
    let (mut map, directions) = parse_data(file_path);
    map.make_map_bigger();
    map.print_to_console();

    for direction in directions {
        // println!("Doing direction {direction:?}");
        map.move_big_robot(direction);
        // map.print_to_console();
        // break;
    }

    map.large_boxes_lefts
        .into_iter()
        .map(|point| (point.row * 100) as usize + point.column as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 10092)]
    #[case(false, 1538871)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 15, None)));
    }

    #[rstest]
    #[case(true, 9021)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 15, None)));
    }
}
