mod file_utilities;

mod day_16;
mod map_utilities;

use std::time::Instant;

use crate::day_16::run;
use crate::file_utilities::get_file_path;

fn main() {
    let day = 16;
    let is_test = true;

    for part in [1, 2] {
        let start = Instant::now();
        let result = run(get_file_path(is_test, day, None), part);
        let end = Instant::now();

        let duration = end - start;

        println!("Day {day} Part {part}: {result}, in {duration:?}.");
    }
}
