use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn get_file_path(is_test: bool, day: u32, suffix: Option<&str>) -> String {
    let sub_folder = if is_test { "test" } else { "real" };
    let suffix = suffix.unwrap_or("");
    format!("./data/{sub_folder}/day_{day}{suffix}.txt")
}

#[allow(dead_code)]
pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn read_as_single_line<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(filename).unwrap()
}
