mod file_utilities;

mod day_2;

use crate::day_2::run;
use crate::file_utilities::get_file_path;

fn main() {
    let day = 2;
    let is_test = true;

    for part in [1, 2] {
        println!(
            "Day {day} Part {part}: {}",
            run(get_file_path(is_test, day, None), part),
        );
    }
}
