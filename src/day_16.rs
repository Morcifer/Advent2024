use crate::file_utilities::read_lines;
use crate::map_utilities::{Direction, Point, DIRECTIONS};
use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

struct Node {
    point: Point,
    direction: Direction,
    score: usize,
    history: HashSet<(Point, Direction)>,
}

impl Node {
    fn new(
        point: Point,
        direction: Direction,
        score: usize,
        old_history: HashSet<(Point, Direction)>,
    ) -> Self {
        let mut history = old_history.clone();
        history.insert((point, direction));

        Self {
            point,
            direction,
            score,
            history,
        }
    }

    fn turn_right(&self) -> Option<Self> {
        if self
            .history
            .contains(&(self.point, self.direction.turn_right()))
        {
            return None;
        }

        Some(Self::new(
            self.point,
            self.direction.turn_right(),
            self.score + 1000,
            self.history.clone(),
        ))
    }

    fn turn_left(&self) -> Option<Self> {
        if self
            .history
            .contains(&(self.point, self.direction.turn_left()))
        {
            return None;
        }

        Some(Self::new(
            self.point,
            self.direction.turn_left(),
            self.score + 1000,
            self.history.clone(),
        ))
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

struct Graph {
    start: Point,
    end: Point,
    nodes: HashSet<(Point, Direction)>,
    edges: HashMap<(Point, Direction), (Point, usize)>, // From (point, direction) to (point, required cost).
}

impl Graph {
    fn new(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Point) {
        for direction in DIRECTIONS.into_iter() {
            self.nodes.insert((node, direction));
        }
    }

    fn add_edge(&mut self, from: Point, to: Point, direction: Direction, cost: usize) {
        let key = (from, direction);

        if self.edges.contains_key(&key) {
            return;
        }

        self.edges.insert(key, (to, cost));
    }

    fn find_path(&self) -> usize {
        let mut heap = BinaryHeap::new();
        // let mut vec_deque = VecDeque::new();

        let mut best_nodes = self
            .nodes
            .iter()
            .map(|&node| (node, usize::MAX))
            .collect::<HashMap<(Point, Direction), usize>>();

        let mut best_found = usize::MAX;

        heap.push(Node::new(self.start, Direction::Right, 0, HashSet::new()));
        // vec_deque.push_front(Node::new(self.start, Direction::Right, 0, HashSet::new()));

        while let Some(current_node) = heap.pop() {
        // while let Some(current_node) = vec_deque.pop_front() {
        //     let heap_size = vec_deque.len();
            let heap_size = heap.len();
            if heap_size % 100000 == 0 {
                println!(
                    "Size of heap is {heap_size}. Current score is {} and best score {best_found}.",
                    current_node.score
                );
            }

            if current_node.point == self.end {
                // println!("Score {}", current_node.score);
                // println!("History {:?}", current_node.history);
                best_found = cmp::min(current_node.score, best_found);
                println!("Found best score of {best_found}!");
                continue;
                // return current_node.score;
            }

            if current_node.score > best_found {
                println!("Size of heap is {heap_size}. Cutting branch due to best solution found...");
                continue;
            }

            let key = (current_node.point, current_node.direction);
            let best_cost = best_nodes.get(&key).unwrap();

            if *best_cost <= current_node.score {
                println!("Size of heap is {heap_size}. Cutting branch at {key:?} due to same or better way to get here with {} vs. {best_cost}...", current_node.score);
                continue;
            }

            best_nodes.insert(key, current_node.score);

            if let Some(edge) = self
                .edges
                .get(&(current_node.point, current_node.direction))
            {
                // if DIRECTIONS
                //     .into_iter()
                //     .any(|direction| current_node.history.contains(&(edge.0, direction)))
                // {
                //     continue;
                // }

                // if current_node.history.contains(&(edge.0, current_node.direction))
                // {
                //     println!("History repeats itself at {:?}, {:?}", edge.0, current_node.direction);
                //     continue;
                // }

                let new_node = Node::new(
                    edge.0,
                    current_node.direction,
                    current_node.score + edge.1,
                    current_node.history.clone(),
                );

                heap.push(new_node);
                // vec_deque.push_back(new_node);
            }

            if let Some(right_node) = current_node.turn_right() {
                heap.push(right_node);
                // vec_deque.push_back(right_node);
            }

            if let Some(left_node) = current_node.turn_left() {
                heap.push(left_node);
                // vec_deque.push_back(left_node);
            }

            // if let Some(left_node) = current_node.turn_left() {
            //     // heap.push(left_node);
            //     if let Some(left_left_node) = left_node.turn_left() {
            //         // heap.push(left_node);
            //         heap.push_back(left_left_node);
            //     }
            // }

            // let heap_size = heap.len();
            // if heap_size > 2000000 {
            //     println!("Draining heap");
            //     heap = heap.drain().take(1000000).collect();
            //     // println!("Size of heap is {heap_size}. Current best score is {}.", current_node.score);
            // }
        }

        best_found
    }
}

impl Map {
    fn new() -> Self {
        Self {
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            walls: HashSet::new(),
        }
    }

    fn into_graph(self) -> Graph {
        let mut graph = Graph::new(self.start, self.end);

        let mut nodes = HashSet::new();

        // First - look for intersections and corners. Those will be our nodes.
        for row in 0..=self.start.row() {
            for column in 0..=self.end.column() {
                let point = Point::new(row as isize, column as isize);

                if self.walls.contains(&point) {
                    continue;
                }

                let neighbours = DIRECTIONS
                    .into_iter()
                    .map(|direction| (direction, point.unbound_neighbour(direction)))
                    .filter(|(_, point)| !self.walls.contains(point))
                    .collect::<HashMap<_, _>>();

                if neighbours.len() != 2 {
                    nodes.insert(point);
                    graph.add_node(point);
                }

                if neighbours.contains_key(&Direction::Up)
                    && neighbours.contains_key(&Direction::Down)
                {
                    continue;
                }

                if neighbours.contains_key(&Direction::Left)
                    && neighbours.contains_key(&Direction::Right)
                {
                    continue;
                }

                nodes.insert(point);
                graph.add_node(point);
            }
        }

        // println!("{nodes:?}");

        for node in nodes.iter().cloned() {
            // Keep going straight in any direction, and stop when you hit an edge
            for direction in DIRECTIONS.into_iter() {
                let mut point = node.unbound_neighbour(direction);

                while !nodes.contains(&point) && !self.walls.contains(&point) {
                    point = point.unbound_neighbour(direction);
                }

                if self.walls.contains(&point) {
                    continue;
                }

                let diff_row = (point.row - node.row).abs();
                let diff_col = (node.column - point.column).abs();

                let cost = cmp::max(diff_row, diff_col) as usize;

                graph.add_edge(node, point, direction, cost);
                graph.add_edge(point, node, direction.reverse(), cost);
            }
        }

        graph
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
    println!("Made map");
    let graph = map.into_graph();
    println!("Made graph");

    graph.find_path()
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
