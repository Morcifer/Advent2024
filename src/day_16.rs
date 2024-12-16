use crate::file_utilities::read_lines;
use crate::map_utilities::{Direction, Point};
use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;

struct Node {
    point: Point,
    direction: Direction,
    score: usize,
    history: Vec<(Point, Direction)>,
    history_hashset: HashSet<(Point, Direction)>,
}

impl Node {
    fn new(
        point: Point,
        direction: Direction,
        score: usize,
        old_history: Vec<(Point, Direction)>,
        old_history_hashset: HashSet<(Point, Direction)>,
    ) -> Self {
        let mut history = old_history;
        history.push((point, direction));

        let mut history_hashset = old_history_hashset.clone();
        history_hashset.insert((point, direction));

        Self {
            point,
            direction,
            score,
            history,
            history_hashset,
        }
    }

    fn turn_right(&self) -> Self {
        Self::new(
            self.point,
            self.direction.turn_right(),
            self.score + 1000,
            self.history.clone(),
            self.history_hashset.clone(),
        )
    }

    fn turn_left(&self) -> Self {
        Self::new(
            self.point,
            self.direction.turn_left(),
            self.score + 1000,
            self.history.clone(),
            self.history_hashset.clone(),
        )
    }

    fn move_forward(&self) -> Self {
        Self::new(
            self.point.unbound_neighbour(self.direction),
            self.direction,
            self.score + 1,
            self.history.clone(),
            self.history_hashset.clone(),
        )
    }

    fn has_loop(&self) -> bool {
        self.history.len() > self.history_hashset.len()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Node {}

// #[derive(Debug, PartialEq, Eq)]
struct Map {
    start: Point,
    end: Point,
    walls: HashSet<Point>,
}

impl Map {
    fn new() -> Self {
        Self {
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            walls: HashSet::new(),
        }
    }

    fn print_to_console(&self) {
        let max_row = self.walls.iter().map(|w| w.row).max().unwrap();
        let max_column = self.walls.iter().map(|w| w.column).max().unwrap();

        let mut map = (0..=max_row)
            .map(|_| (0..=max_column).map(|_| '.').collect_vec())
            .collect_vec();

        for wall in self.walls.iter() {
            map[wall.row()][wall.column()] = '#';
        }

        map[self.start.row()][self.start.column()] = 'S';
        map[self.end.row()][self.end.column()] = 'E';

        for row in map {
            println!("{}", row.iter().collect::<String>());
        }
    }

    fn find_path(&self) -> usize {
        let mut heap = BinaryHeap::new();
        heap.push(Node::new(
            self.start,
            Direction::Right,
            0,
            vec![],
            HashSet::new(),
        ));

        while let Some(current_node) = heap.pop() {
            if current_node.point == self.end {
                // println!("Score {}", current_node.score);
                // println!("History {:?}", current_node.history);

                return current_node.score;
            }

            if self.walls.contains(&current_node.point) {
                continue;
            }

            if current_node.has_loop() {
                continue;
            }

            heap.push(current_node.move_forward());

            // Check for hallway - only turn if you're not in one.
            match current_node.direction {
                Direction::Up | Direction::Down => {
                    if !self
                        .walls
                        .contains(&current_node.point.unbound_neighbour(Direction::Left))
                        || !self
                            .walls
                            .contains(&current_node.point.unbound_neighbour(Direction::Right))
                    {
                        heap.push(current_node.turn_right());
                        heap.push(current_node.turn_left());
                    }
                }

                Direction::Left | Direction::Right => {
                    if !self
                        .walls
                        .contains(&current_node.point.unbound_neighbour(Direction::Up))
                        || !self
                            .walls
                            .contains(&current_node.point.unbound_neighbour(Direction::Down))
                    {
                        heap.push(current_node.turn_right());
                        heap.push(current_node.turn_left());
                    }
                }
            }
        }

        // I shouldn't be here, there's a problem!
        0
    }
}

fn parse_data(file_path: String) -> Map {
    let mut map = Map::new();

    for (row_index, row) in read_lines(file_path).into_iter().enumerate() {
        for (column_index, character) in row.chars().enumerate() {
            match character {
                '.' => continue,
                '#' => {
                    map.walls
                        .insert(Point::new(row_index as isize, column_index as isize));
                }
                'S' => map.start = Point::new(row_index as isize, column_index as isize),
                'E' => map.end = Point::new(row_index as isize, column_index as isize),
                _ => panic!("Got an invalid map thingie."),
            }
        }
    }

    map
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
    let map = parse_data(file_path);
    map.find_path()
}

fn part_2(_file_path: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 11048)]
    #[case(false, 0)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 16, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 16, None)));
    }
}
