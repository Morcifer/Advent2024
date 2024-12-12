#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub row: isize,
    pub column: isize,
}

impl Point {
    pub fn new(row: isize, column: isize) -> Self {
        Self { row, column }
    }

    pub fn row(&self) -> usize {
        self.row as usize
    }

    pub fn column(&self) -> usize {
        self.column as usize
    }

    pub fn unbound_neighbour(&self, direction: Direction) -> Self {
        let (row, column) = match direction {
            Direction::Up => (self.row - 1, self.column),
            Direction::Right => (self.row, self.column + 1),
            Direction::Down => (self.row + 1, self.column),
            Direction::Left => (self.row, self.column - 1),
        };

        Self::new(row, column)
    }

    pub fn neighbour(&self, direction: Direction, map_size: usize) -> Option<Self> {
        let map_size = map_size as isize;
        let neighbour = self.unbound_neighbour(direction);

        if neighbour.row < 0 || neighbour.row >= map_size || neighbour.column < 0 || neighbour.column >= map_size {
            return None;
        }

        Some(neighbour)
    }
}