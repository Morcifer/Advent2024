use crate::file_utilities::read_lines;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Robot {
    position: (i64, i64), // x is the column, y is the row!
    velocity: (i64, i64),
}

fn parse_line_to_robot(line: String) -> Robot {
    let space_split = line.split(" ").map(str::trim).collect::<Vec<_>>();

    let position = space_split[0][2..]
        .split(",")
        .map(str::trim)
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let velocity = space_split[1][2..]
        .split(",")
        .map(str::trim)
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // println!("position: {position:?}, velocity: {velocity:?}");

    Robot {
        position: (position[0], position[1]),
        velocity: (velocity[0], velocity[1]),
    }
}

fn parse_data(file_path: String) -> Vec<Robot> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_robot)
        .collect()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> usize {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn print_robots(positions: &Vec<(i64, i64)>, max_x: i64, max_y: i64) {
    let max_x = max_x as usize;
    let max_y = max_y as usize;

    let mut map = (0..=max_y)
        .map(|_| (0..=max_x).map(|_| '.').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (x, y) in positions {
        let x = *x as usize;
        let y = *y as usize;

        let current = map[y][x];
        match current {
            '.' => map[y][x] = '1',
            other => map[y][x] = ((other as u8) + 1) as char,
        }
    }

    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
}

fn get_robots_after_seconds(
    robots: &[Robot],
    max_x: i64,
    max_y: i64,
    seconds: usize,
) -> Vec<(i64, i64)> {
    let seconds = seconds as i64;

    robots
        .iter()
        .copied()
        .map(|robot| {
            (
                robot.position.0 + robot.velocity.0 * seconds,
                robot.position.1 + robot.velocity.1 * seconds,
            )
        })
        .map(|(x, y)| (x.rem_euclid(max_x + 1), y.rem_euclid(max_y + 1)))
        .collect::<Vec<_>>()
}

fn part_1(file_path: String) -> usize {
    let robots = parse_data(file_path);

    let robot_positions = robots
        .iter()
        .map(|robot| robot.position)
        .collect::<Vec<_>>();

    let max_x = *robot_positions.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *robot_positions.iter().map(|(_, y)| y).max().unwrap();

    let robots_after_100_seconds = get_robots_after_seconds(&robots, max_x, max_y, 100);

    let quadrant_x = max_x / 2;
    let quadrant_y = max_y / 2;

    let mut quadrants = vec![vec![0, 0], vec![0, 0]];

    for (x, y) in robots_after_100_seconds.into_iter() {
        if x == quadrant_x || y == quadrant_y {
            continue;
        }

        let q_x = if x > quadrant_x { 1 } else { 0 };
        let q_y = if y > quadrant_y { 1 } else { 0 };

        quadrants[q_x][q_y] += 1;
    }

    quadrants.into_iter().flatten().product()
}

fn part_2(file_path: String) -> usize {
    let robots = parse_data(file_path);

    let robot_positions = robots
        .iter()
        .map(|robot| robot.position)
        .collect::<Vec<_>>();

    let max_x = *robot_positions.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *robot_positions.iter().map(|(_, y)| y).max().unwrap();

    for seconds in 0..20000 {
        let robots_after_seconds = get_robots_after_seconds(&robots, max_x, max_y, seconds);

        if robots_after_seconds.iter().unique().count() == robots_after_seconds.len() {
            print_robots(&robots_after_seconds, max_x, max_y);
            return seconds;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 12)]
    #[case(false, 226236192)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 14, None)));
    }

    #[rstest]
    #[case(false, 8168)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 14, None)));
    }
}
