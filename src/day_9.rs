use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;

use crate::file_utilities::read_lines;

fn parse_line_to_int(line: String) -> Vec<usize> {
    line.chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}

fn parse_data(file_path: String) -> Vec<usize> {
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct File {
    index: usize,
    size: usize,
    id: usize,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Gap {
    index: usize,
    size: usize,
}

fn get_files_and_gaps(data: Vec<usize>) -> (Vec<File>, Vec<Gap>) {
    let mut files = vec![];
    let mut gaps = vec![];

    let mut file_index = 0;
    let mut memory_index = 0;

    let data_iteration_length = (data.len() - 1) / 2;

    for i in 0..data_iteration_length {
        let file_size = data[i * 2];
        let gap_size = data[i * 2 + 1];

        files.push(File {
            index: memory_index,
            size: file_size,
            id: file_index,
        });
        memory_index += file_size;

        gaps.push(Gap {
            index: memory_index,
            size: gap_size,
        });
        memory_index += gap_size;

        file_index += 1;
    }

    files.push(File {
        index: memory_index,
        size: *data.last().unwrap(),
        id: file_index,
    });

    (files, gaps)
}

fn get_checksum(ordered: Vec<File>) -> u64 {
    ordered
        .into_iter()
        .flat_map(|file| {
            (0..file.size).map(move |position_delta| {
                (file.index as u64 + position_delta as u64) * file.id as u64
            })
        })
        .sum()
}

fn part_1(file_path: String) -> u64 {
    let data = parse_data(file_path);

    let (mut files, gaps) = get_files_and_gaps(data);
    let mut gaps = gaps.into_iter().collect::<VecDeque<_>>();

    // Start pushing things from the end to the beginning.
    let mut file_index = files.len() - 1;

    while let Some(gap) = gaps.pop_front() {
        let file = files[file_index];

        if gap.index >= file.index {
            break;
        }

        match gap.size.cmp(&file.size) {
            Ordering::Equal => {
                // Move entire file into gap, and move to the next file
                files[file_index] = File {
                    index: gap.index,
                    size: file.size,
                    id: file.id,
                };
                file_index -= 1;
            }
            Ordering::Greater => {
                // Move entire file into gap, move to the next file, and put a leftover gap.
                files[file_index] = File {
                    index: gap.index,
                    size: file.size,
                    id: file.id,
                };
                file_index -= 1;

                gaps.push_front(Gap {
                    index: gap.index + file.size,
                    size: gap.size - file.size,
                });
            }
            Ordering::Less => {
                // Move part of the file into the gap (appending to end of list),
                // and then update a leftover file.
                files.push(File {
                    index: gap.index,
                    size: gap.size,
                    id: file.id,
                });
                files[file_index] = File {
                    index: file.index + gap.size,
                    size: file.size - gap.size,
                    id: file.id,
                };
            }
        }
    }

    let mut ordered = files.into_iter().sorted_by_key(|v| v.index).collect_vec();

    // now hacky close for last gap
    let before_last = ordered[ordered.len() - 2];
    let last = ordered[ordered.len() - 1];
    let index = ordered.len() - 1;

    ordered[index] = File {
        index: before_last.index + before_last.size,
        size: last.size,
        id: last.id,
    };

    get_checksum(ordered)
}

fn part_2(file_path: String) -> u64 {
    let data = parse_data(file_path);

    let (mut files, mut gaps) = get_files_and_gaps(data);

    // Now start pushing things from the end to the beginning.
    let mut file_index = files.len() - 1;

    loop {
        for gap in &mut gaps {
            let file = files[file_index];

            if gap.index >= file.index {
                break;
            }

            match gap.size.cmp(&file.size) {
                Ordering::Equal => {
                    // Move entire file into gap, and move to the next file
                    files[file_index] = File {
                        index: gap.index,
                        size: file.size,
                        id: file.id,
                    };
                    *gap = Gap {
                        index: gap.index,
                        size: 0,
                    };

                    break;
                }
                Ordering::Greater => {
                    // Move entire file into gap, move to the next file, and put a leftover gap.
                    files[file_index] = File {
                        index: gap.index,
                        size: file.size,
                        id: file.id,
                    };
                    *gap = Gap {
                        index: gap.index + file.size,
                        size: gap.size - file.size,
                    };

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

    let ordered = files.into_iter().sorted_by_key(|v| v.index).collect_vec();

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
