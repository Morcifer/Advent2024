#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn get_file_path(is_test: bool, day: u32, suffix: Option<&str>) -> String {
    let sub_folder = if is_test { "test" } else { "real" };
    let suffix = suffix.unwrap_or("");
    format!("./data/{sub_folder}/day_{day}{suffix}.txt")
}

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

pub fn read_two_chunks<P>(filename: P) -> (Vec<String>, Vec<String>)
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();

    let mut collection_1 = Vec::new();
    let mut collection_2 = Vec::new();

    let mut split_found = false;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            split_found = true;
            continue;
        }

        if split_found {
            collection_2.push(line);
        } else {
            collection_1.push(line);
        }
    }

    (collection_1, collection_2)
}

pub fn read_chunks<P>(filename: P) -> Vec<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();

    let mut collections = Vec::new();
    let mut current_collection = Vec::new();

    // TODO: Is there a way to linq-statement this?
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            collections.push(current_collection);
            current_collection = Vec::new();
            continue;
        }

        current_collection.push(line);
    }

    collections.push(current_collection);
    collections
}

pub fn read_as_single_line<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(filename).unwrap()
}
