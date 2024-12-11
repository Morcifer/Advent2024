use std::collections::HashMap;

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> u64 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn blink(stone: u128, times_remaining: usize, cache: &mut HashMap<(u128, usize), usize>) -> usize {
    if times_remaining == 0 {
        return 1;
    }

    let key = (stone, times_remaining);

    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let result = match stone {
        0 => blink(1, times_remaining - 1, cache),
        number if (stone.ilog10() % 2) == 1 => {
            let length = number.ilog10() + 1;
            let half_length = length / 2;
            let power_of_ten = 10_u128.pow(half_length);
            blink(number / power_of_ten, times_remaining - 1, cache)
                + blink(number % power_of_ten, times_remaining - 1, cache)
        }
        number => blink(number * 2024, times_remaining - 1, cache),
    };

    cache.insert(key, result);
    result
}

fn blink_many_times(stones: Vec<u128>, times: usize) -> usize {
    let mut cache: HashMap<(u128, usize), usize> = HashMap::new();

    stones
        .into_iter()
        .map(|stone| blink(stone, times, &mut cache))
        .sum()
}

const TEST_CASE: &[u128] = &[125, 17];
const REAL_CASE: &[u128] = &[965842, 9159, 3372473, 311, 0, 6, 86213, 48];

fn part_1(file_path: String) -> u64 {
    let stones = if file_path.contains("test") {
        TEST_CASE
    } else {
        REAL_CASE
    };
    blink_many_times(stones.to_vec(), 25) as u64
}

fn part_2(file_path: String) -> u64 {
    let stones = if file_path.contains("test") {
        TEST_CASE
    } else {
        REAL_CASE
    };
    blink_many_times(stones.to_vec(), 75) as u64
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
    #[case(vec![965842,9159,3372473,311,0,6,86213,48], 75, 218279375708592)]
    fn test_blink_many_times(
        #[case] stones: Vec<u128>,
        #[case] times: usize,
        #[case] expected: usize,
    ) {
        assert_eq!(expected, blink_many_times(stones, times));
    }

    #[rstest]
    #[case(true, 55312)]
    #[case(false, 183435)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 11, None)));
    }

    #[rstest]
    #[case(false, 218279375708592)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 11, None)));
    }
}
