use crate::file_utilities::read_two_chunks;
use crate::map_utilities::{Direction, Point};
use std::collections::HashSet;

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Thing {
    Empty,
    Wall,
    Box,
    LeftBox,
    RightBox,
    Robot,
}

impl Thing {
    fn from_char(c: char) -> Thing {
        match c {
            '#' => Thing::Wall,
            'O' => Thing::Box,
            '@' => Thing::Robot,
            '.' => Thing::Empty,
            // '[' => Thing::LeftBox, // Should not happen.
            // ']' => Thing::RightBox, // Should not happen.
            _ => panic! {"Did not expect this!"},
        }
    }

    fn from_char_bigger(c: char) -> Vec<Thing> {
        match c {
            '#' => vec![Thing::Wall, Thing::Wall],
            'O' => vec![Thing::LeftBox, Thing::RightBox],
            '@' => vec![Thing::Robot, Thing::Empty],
            '.' => vec![Thing::Empty, Thing::Empty],
            _ => panic! {"Did not expect this!"},
        }
    }

    fn to_char(&self) -> char {
        match self {
            Thing::Wall => '#',
            Thing::Box => 'O',
            Thing::Robot => '@',
            Thing::Empty => '.',
            Thing::LeftBox => '[',
            Thing::RightBox => ']',
        }
    }
}

struct Map {
    robot: Point,
    map: Vec<Vec<Thing>>,
}

impl Map {
    fn new(lines: Vec<String>, bigger: bool) -> Self {
        let mut map = vec![];
        let mut robot = Point::new(0, 0);

        for (row, line) in lines.into_iter().enumerate() {
            let mut map_line = vec![];

            for (column, char) in line.chars().enumerate() {
                if bigger {
                    let things = Thing::from_char_bigger(char);

                    if things[0] == Thing::Robot {
                        robot = Point::new(row as isize, (column * 2) as isize);
                    }

                    map_line.extend(things);
                } else {
                    let thing = Thing::from_char(char);
                    if thing == Thing::Robot {
                        robot = Point::new(row as isize, column as isize);
                    }
                    map_line.push(thing);
                }
            }

            map.push(map_line);
        }

        Self { map, robot }
    }

    fn get_boxes(&self) -> Vec<Point> {
        let mut result = vec![];

        for (i, row) in self.map.iter().enumerate() {
            for (j, thing) in row.iter().enumerate() {
                match thing {
                    Thing::Box | Thing::LeftBox => {
                        result.push(Point::new(i as isize, j as isize));
                    }
                    _ => continue,
                };
            }
        }

        result
    }

    fn print_to_console(&self) {
        for row in &self.map {
            let to_print = row.iter().map(Thing::to_char).join("");
            println!("{to_print}");
        }
    }

    fn move_robot(&mut self, direction: Direction) {
        let mut spots_to_move = vec![vec![self.robot].into_iter().collect::<HashSet<_>>()];
        let mut last_spots_to_move = spots_to_move.last().unwrap().clone();

        while last_spots_to_move
            .iter()
            .map(|spot| spot.unbound_neighbour(direction))
            .any(|spot| self.map[spot.row()][spot.column()] != Thing::Empty)
        {
            let mut new_spots_to_move = HashSet::new();

            for spot in last_spots_to_move {
                let new_spot = spot.unbound_neighbour(direction);
                let spot_thing = self.map[new_spot.row()][new_spot.column()];

                match direction {
                    Direction::Right | Direction::Left => {
                        match spot_thing {
                            Thing::Wall => {
                                // println!("There's nothing we can do, {new_spot:?} is a wall!");
                                return;
                            }
                            Thing::Empty => {
                                continue;
                            }
                            _ => {
                                new_spots_to_move.insert(new_spot);
                            }
                        }
                    }
                    Direction::Up | Direction::Down => {
                        match spot_thing {
                            Thing::Wall => {
                                // println!("There's nothing we can do, {new_spot:?} is a wall!");
                                return;
                            }
                            Thing::Empty => {
                                continue;
                            }
                            Thing::Box => {
                                new_spots_to_move.insert(new_spot);
                            }
                            Thing::LeftBox => {
                                new_spots_to_move.insert(new_spot);
                                new_spots_to_move
                                    .insert(new_spot.unbound_neighbour(Direction::Right));
                            }
                            Thing::RightBox => {
                                new_spots_to_move.insert(new_spot);
                                new_spots_to_move
                                    .insert(new_spot.unbound_neighbour(Direction::Left));
                            }
                            any => panic!("I got {any:?} but I shouldn't have!"),
                        }
                    }
                }
            }

            spots_to_move.push(new_spots_to_move.clone());
            last_spots_to_move = new_spots_to_move.clone();
        }

        // Move in reverse order, but skip the robot line and the wall line.
        // println!("{spots_to_move:?}");
        for last_spots_to_move in spots_to_move.iter().rev() {
            for spot in last_spots_to_move {
                let spot_to_move_to = spot.unbound_neighbour(direction);
                self.map[spot_to_move_to.row()][spot_to_move_to.column()] =
                    self.map[spot.row()][spot.column()];
                self.map[spot.row()][spot.column()] = Thing::Empty;
            }
        }

        self.robot = self.robot.unbound_neighbour(direction);
    }
}

fn parse_data(file_path: String, bigger: bool) -> (Map, Vec<Direction>) {
    let (map_lines, direction_lines) = read_two_chunks(file_path);

    let map = Map::new(map_lines, bigger);

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
    let (mut map, directions) = parse_data(file_path, false);
    map.print_to_console();

    for direction in directions {
        // println!("Moving {direction:?}:");
        map.move_robot(direction);
        // map.print_to_console();
    }

    map.print_to_console();

    map.get_boxes()
        .into_iter()
        .map(|point| (point.row * 100) as usize + point.column as usize)
        .sum()
}

fn part_2(file_path: String) -> usize {
    let (mut map, directions) = parse_data(file_path, true);
    map.print_to_console();

    for direction in directions {
        // println!("Moving {direction:?}:");
        map.move_robot(direction);
        // map.print_to_console();
    }

    map.print_to_console();

    map.get_boxes()
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
    #[case(false, 1543338)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 15, None)));
    }
}
