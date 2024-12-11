use crate::file_utilities::read_lines;
use itertools::Itertools;

fn parse_line_to_numbers(line: String) -> Vec<usize> {
    line.chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}

fn parse_data(file_path: String) -> Vec<Vec<usize>> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_numbers)
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

fn blink(stones: Vec<u128>) -> Vec<u128> {
    let mut offset = 0;

    let original_stones = stones.clone();
    let mut stones = stones.clone();

    for (index, stone) in original_stones.into_iter().enumerate() {
        match stone {
            0 => stones[index + offset] = 1,
            number if (stone.ilog10() % 2) == 1 => {
                let length = number.ilog10() + 1;
                let half_length = length / 2;
                let power_of_ten = 10_u128.pow(half_length);
                stones[index + offset] = number / power_of_ten;
                stones.insert(index+offset+1, number % power_of_ten);

                offset += 1;
            },
            number => stones[index + offset] = number * 2024
        };
    }

    stones
}

fn blink_many_times(stones: Vec<u128>, times: usize) -> usize {
    // println!("After 0 blinks we have {} stones", stones.len());
    let mut stones = stones.clone();

    for _blink_count in 0..times {
        stones = blink(stones);
        // println!("After {_blink_count } blinks we have {} stones", stones.len());
    }

    stones.len()
}

fn part_1(file_path: String) -> u64 {
    // let mut stones: Vec<u128> = vec![125, 17];
    let stones: Vec<u128> = vec![965842,9159,3372473,311,0,6,86213,48];

    blink_many_times(stones, 25) as u64
}

fn part_2(file_path: String) -> u64 {
    let data = parse_data(file_path);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(vec![125, 17], 6, 22)]
    #[case(vec![125, 17], 25, 55312)]
    #[case(vec![965842,9159,3372473,311,0,6,86213,48], 25, 183435)]
    fn test_blink_many_times(#[case] stones: Vec<u128>, #[case] times: usize, #[case] expected: usize) {
        assert_eq!(expected, blink_many_times(stones, times));
    }
}
