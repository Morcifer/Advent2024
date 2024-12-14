mod file_utilities;

mod day_14;
mod map_utilities;

use std::time::Instant;

use crate::day_14::run;
use crate::file_utilities::get_file_path;

fn main() {
    let day = 14;
    let is_test = false;

    for part in [1, 2] {
        let start = Instant::now();
        let result = run(get_file_path(is_test, day, None), part);
        let end = Instant::now();

        let duration = end - start;

        println!("Day {day} Part {part}: {result}, in {duration:?}.");
    }
}
