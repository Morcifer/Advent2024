use crate::file_utilities::read_chunks;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GameData {
    button_a_move: (i32, i32),
    button_b_move: (i32, i32),
    prize: (i32, i32),
}

fn parse_lines_to_game_data(line: Vec<String>) -> GameData {
    let mut game_data = GameData { button_a_move: (0, 0), button_b_move: (0, 0), prize: (0, 0) };
    for (index, line) in line.iter().enumerate() {
        let colon_split = line.split(":").map(str::trim).last().unwrap();
        let comma_split = colon_split.split(",").map(str::trim).collect::<Vec<_>>();
        let x = comma_split[0][2..].parse::<i32>().unwrap();
        let y = comma_split[1][2..].parse::<i32>().unwrap();

        match index {
            0 => game_data.button_a_move = (x, y),
            1 => game_data.button_b_move = (x, y),
            2 => game_data.prize = (x, y),
            _ => panic!("I'm having too many strings here...")
        };
    }

    game_data
}

fn parse_data(file_path: String) -> Vec<GameData> {
    read_chunks(file_path)
        .into_iter()
        .map(parse_lines_to_game_data)
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> i32 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> i32 {
    let games = parse_data(file_path);

    let mut result = 0;

    for game in games {
        println!("Handling game {game:?}");
        let button_a_x_moves_more = game.button_a_move.0 >= game.button_b_move.0;

        let (modulo_x, modulo_y) = if button_a_x_moves_more {
            (game.button_a_move.0 % game.button_b_move.0, game.button_a_move.1 % game.button_b_move.1)
        } else {
            (game.button_b_move.0 % game.button_a_move.0, game.button_b_move.1 % game.button_a_move.1)
        };

        if modulo_x == 0 && modulo_y == 0 {
            // Button A is a multiple of button B, or vice versa
            // println!("Button A is a multiple of button B, or vice versa");
            if button_a_x_moves_more && (game.button_a_move.0 / game.button_b_move.0 > 3) {
                // Use only button A, it's cheaper
                let prize_clicks = game.prize.0 / game.button_a_move.0;
                let target_x = prize_clicks * game.button_a_move.0;
                let target_y = prize_clicks * game.button_a_move.1;

                if target_x == game.prize.0 && target_y == game.prize.1 {
                    result += 3 * (game.prize.0 / game.button_a_move.0);
                } else {
                    // Game can't be solved
                    // println!("Game is unwinnable");
                }
            } else {
                // Use only button A, it's cheaper
                let prize_clicks = game.prize.0 / game.button_b_move.0;
                let target_x = prize_clicks * game.button_b_move.0;
                let target_y = prize_clicks * game.button_b_move.1;

                if target_x == game.prize.0 && target_y == game.prize.1 {
                    result += 1 * (game.prize.0 / game.button_b_move.0);
                } else {
                    // Game can't be solved
                    // println!("Game is unwinnable");
                }
            }

            continue;
        }

        // Combination of A and B!
        let a = game.button_a_move.0;
        let b = game.button_b_move.0;
        let c = game.button_a_move.1;
        let d = game.button_b_move.1;

        let number = a * d - b * c;
        let prize_a_clicks = (d * game.prize.0 - b * game.prize.1) / number;
        let prize_b_clicks = (-c * game.prize.0 + a * game.prize.1) / number;

        let final_x = prize_a_clicks * game.button_a_move.0 + prize_b_clicks * game.button_b_move.0;
        let final_y = prize_a_clicks * game.button_a_move.1 + prize_b_clicks * game.button_b_move.1;

        // println!("Might need {prize_a_clicks} a clicks and {prize_b_clicks} b clicks");

        if final_x == game.prize.0 && final_y == game.prize.1 {
            result += 3 * prize_a_clicks + 1 * prize_b_clicks;
        } else
        {
            // println!("Game is unwinnable");
        }
    }

    result as i32
}

fn part_2(file_path: String) -> i32 {
    let games = parse_data(file_path);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_1(get_file_path(is_test, 13, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i32) {
        assert_eq!(expected, part_2(get_file_path(is_test, 13, None)));
    }
}
