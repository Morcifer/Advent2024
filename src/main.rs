mod file_utilities;

mod day_1;

use crate::day_1::run;
use crate::file_utilities::get_file_path;

fn main() {
    let day = 1;
    let is_test = false;

    for part in [1, 2] {
        println!(
            "Day {day} Part {part}: {}",
            run(get_file_path(is_test, day, None), part),
        );
    }
}
