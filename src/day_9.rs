use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;

use crate::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> Vec<u32> {
    line.chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn parse_data(file_path: String) -> Vec<u32> {
    read_lines(file_path)
        .into_iter()
        .map(parse_line_to_int)
        .next()
        .unwrap()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> u64 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn get_files_and_gaps(data: Vec<u32>) -> (Vec<(u32, u32, i32)>, Vec<(u32, u32)>) {
    let mut data_index_length_id = vec![];
    let mut gap_index_length = vec![];

    let mut file_index = 0;
    let mut memory_index = 0;

    let data_iteration_length = (data.len() - 1) / 2;

    for i in 0..data_iteration_length {
        let file_size = data[i * 2];
        let gap_size = data[i * 2 + 1];

        data_index_length_id.push((memory_index, file_size, file_index));
        memory_index += file_size;

        gap_index_length.push((memory_index, gap_size));
        memory_index += gap_size;

        file_index += 1;
    }

    data_index_length_id.push((memory_index, *data.last().unwrap(), file_index));
    (data_index_length_id, gap_index_length)
}

fn get_checksum(ordered: Vec<(u32, u32, i32)>) -> u64 {
    ordered
        .into_iter()
        .flat_map(|(memory_index, file_size, file_id)| {
            (0..file_size).map(move |position_delta| {
                (memory_index as u64 + position_delta as u64) * file_id as u64
            })
        })
        .sum()
}

fn part_1(file_path: String) -> u64 {
    let data = parse_data(file_path);

    let (mut data_index_length_id, gap_index_length) = get_files_and_gaps(data);
    let mut gap_index_length = gap_index_length.into_iter().collect::<VecDeque<_>>();

    // Start pushing things from the end to the beginning.
    let mut file_index = data_index_length_id.len() - 1;

    while let Some((gap_memory, gap_size)) = gap_index_length.pop_front() {
        let (file_memory, file_size, file_id) = data_index_length_id[file_index];

        if gap_memory >= file_memory {
            break;
        }

        match gap_size.cmp(&file_size) {
            Ordering::Equal => {
                // Move entire file into gap, and move to the next file
                data_index_length_id[file_index] = (gap_memory, file_size, file_id);
                file_index -= 1;
            }
            Ordering::Greater => {
                // Move entire file into gap, move to the next file, and put a leftover gap.
                data_index_length_id[file_index] = (gap_memory, file_size, file_id);
                file_index -= 1;

                gap_index_length.push_front((gap_memory + file_size, gap_size - file_size));
            }
            Ordering::Less => {
                // Move part of the file into the gap (appending to end of list),
                // and then update a leftover file.
                data_index_length_id.push((gap_memory, gap_size, file_id));
                data_index_length_id[file_index] =
                    (file_memory + gap_size, file_size - gap_size, file_id);
            }
        }
    }

    let mut ordered = data_index_length_id
        .into_iter()
        .sorted_by_key(|v| v.0)
        .collect_vec();

    // now hacky close for last gap
    let before_last = ordered[ordered.len() - 2];
    let last = ordered[ordered.len() - 1];
    let index = ordered.len() - 1;

    ordered[index] = (before_last.0 + before_last.1, last.1, last.2);

    get_checksum(ordered)
}

fn part_2(file_path: String) -> u64 {
    let data = parse_data(file_path);

    let (mut data_index_length_id, mut gap_index_length) = get_files_and_gaps(data);

    // Now start pushing things from the end to the beginning.
    let mut file_index = data_index_length_id.len() - 1;

    loop {
        let (file_memory, file_size, file_id) = data_index_length_id[file_index];

        for gap_item in &mut gap_index_length {
            let (gap_memory, gap_size) = *gap_item;

            if gap_memory >= file_memory {
                break;
            }

            match gap_size.cmp(&file_size) {
                Ordering::Equal => {
                    // Move entire file into gap, and move to the next file
                    data_index_length_id[file_index] = (gap_memory, file_size, file_id);
                    *gap_item = (gap_memory, 0);

                    break;
                }
                Ordering::Greater => {
                    // Move entire file into gap, move to the next file, and put a leftover gap.
                    data_index_length_id[file_index] = (gap_memory, file_size, file_id);
                    *gap_item = (gap_memory + file_size, gap_size - file_size);

                    break;
                }
                Ordering::Less => {}
            }
        }

        if file_index == 0 {
            break;
        }

        file_index -= 1;
    }

    let ordered = data_index_length_id
        .into_iter()
        .sorted_by_key(|v| v.0)
        .collect_vec();

    get_checksum(ordered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 1928)]
    #[case(false, 6519155389266)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 9, None)));
    }

    #[rstest]
    #[case(true, 2858)]
    #[case(false, 6547228115826)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 9, None)));
    }
}
