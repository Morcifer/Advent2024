use crate::file_utilities::read_lines;
use crate::map_utilities::{Direction, Point};
use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

struct Node {
    point: Point,
    direction: Direction,
    score: usize,
    history: Vec<(Point, Direction)>,
}

impl Node {
    fn new(
        point: Point,
        direction: Direction,
        score: usize,
        old_history: Vec<(Point, Direction)>,
    ) -> Self {
        let mut history = old_history.clone();
        history.push((point, direction));

        Self {
            point,
            direction,
            score,
            history,
        }
    }

    fn turn_right(&self) -> Self {
        Self::new(
            self.point,
            self.direction.turn_right(),
            self.score + 1000,
            self.history.clone(),
        )
    }

    fn turn_left(&self) -> Self {
        Self::new(
            self.point,
            self.direction.turn_left(),
            self.score + 1000,
            self.history.clone(),
        )
    }

    fn move_forward(&self) -> Self {
        Self::new(
            self.point.unbound_neighbour(self.direction),
            self.direction,
            self.score + 1,
            self.history.clone(),
        )
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

    fn find_paths(&self, known_best_cost: Option<usize>) -> Vec<Node> {
        let mut all_solutions = vec![];

        let mut heap = BinaryHeap::new();

        let mut best_nodes: HashMap<(Point, Direction), usize> = HashMap::new();

        let mut best_found = known_best_cost.unwrap_or(usize::MAX);

        heap.push(Node::new(self.start, Direction::Right, 0, vec![]));
        // vec_deque.push_front(Node::new(self.start, Direction::Right, 0, HashSet::new()));

        while let Some(current_node) = heap.pop() {
            if current_node.point == self.end {
                // println!("Score {}", current_node.score);
                // println!("History {:?}", current_node.history);

                if let Some(known_best_cost) = known_best_cost {
                    if current_node.score == known_best_cost {
                        all_solutions.push(current_node);
                    }
                } else {
                    best_found = cmp::min(current_node.score, best_found);
                    // println!("Found best score of {best_found}!");
                    all_solutions.push(current_node);
                }

                continue;
            }

            if self.walls.contains(&current_node.point) {
                continue;
            }

            if current_node.score > best_found {
                // println!("Size of heap is {heap_size}. Cutting branch due to best solution found...");
                continue;
            }

            let key = (current_node.point, current_node.direction);

            if let Some(best_known_cost) = best_nodes.get(&key) {
                if *best_known_cost < current_node.score {
                    // println!("Size of heap is {heap_size}. Cutting branch at {key:?} due to same or better way to get here with {} vs. {best_cost}...", current_node.score);
                    continue;
                }
            }

            best_nodes.insert(key, current_node.score);

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

        all_solutions
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
    map.find_paths(None)
        .into_iter()
        .map(|node| node.score)
        .min()
        .unwrap()
}

fn part_2(file_path: String) -> usize {
    let map = parse_data(file_path);
    let paths = map.find_paths(None);
    let best_cost = paths.iter().map(|node| node.score).min().unwrap();

    let best_paths = map.find_paths(Some(best_cost));
    best_paths
        .into_iter()
        .flat_map(|node| node.history)
        .map(|(point, _)| point)
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 11048)]
    #[case(false, 99488)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 16, None)));
    }

    #[rstest]
    #[case(true, 64)]
    #[case(false, 516)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 16, None)));
    }
}
