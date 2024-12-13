use std::cmp;

use crate::file_utilities::read_chunks;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct GameData {
    button_a_move: (i64, i64),
    button_b_move: (i64, i64),
    prize: (i64, i64),
}

fn parse_lines_to_game_data(line: Vec<String>) -> GameData {
    let mut game_data = GameData {
        button_a_move: (0, 0),
        button_b_move: (0, 0),
        prize: (0, 0),
    };
    for (index, line) in line.iter().enumerate() {
        let colon_split = line.split(":").map(str::trim).last().unwrap();
        let comma_split = colon_split.split(",").map(str::trim).collect::<Vec<_>>();
        let x = comma_split[0][2..].parse::<i64>().unwrap();
        let y = comma_split[1][2..].parse::<i64>().unwrap();

        match index {
            0 => game_data.button_a_move = (x, y),
            1 => game_data.button_b_move = (x, y),
            2 => game_data.prize = (x, y),
            _ => panic!("I'm having too many strings here..."),
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
pub fn run(file_path: String, part: i32) -> i64 {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn get_cost_for_clicks(button_a_clicks: i64, button_b_clicks: i64) -> i64 {
    3 * button_a_clicks + button_b_clicks
}

fn get_cost_for_game(game: GameData) -> i64 {
    // Wikipedia expression for inverse of 2x2 matrix for the win.
    let a = game.button_a_move.0;
    let b = game.button_b_move.0;
    let c = game.button_a_move.1;
    let d = game.button_b_move.1;

    let composite = a * d - b * c;

    if composite == 0 {
        // One of the buttons is a linear combination of the other.
        // We'll return the one that's cheaper.
        let required_button_a = game.prize.0 / game.button_a_move.0;
        let required_button_b = game.prize.0 / game.button_b_move.0;

        let target_x = required_button_a * game.button_a_move.0;
        let target_y = required_button_a * game.button_a_move.1;

        if target_x == game.prize.0 && target_y == game.prize.1 {
            return cmp::min(
                get_cost_for_clicks(required_button_a, 0),
                get_cost_for_clicks(0, required_button_b),
            );
        }

        return 0;
    }

    // Combination of A and B!
    let prize_a_clicks = (d * game.prize.0 - b * game.prize.1) / composite;
    let prize_b_clicks = (-c * game.prize.0 + a * game.prize.1) / composite;

    let final_x = prize_a_clicks * game.button_a_move.0 + prize_b_clicks * game.button_b_move.0;
    let final_y = prize_a_clicks * game.button_a_move.1 + prize_b_clicks * game.button_b_move.1;

    if final_x == game.prize.0 && final_y == game.prize.1 {
        return get_cost_for_clicks(prize_a_clicks, prize_b_clicks);
    }

    0
}

fn part_1(file_path: String) -> i64 {
    let games = parse_data(file_path);

    games.into_iter().map(get_cost_for_game).sum()
}

fn part_2(file_path: String) -> i64 {
    let games = parse_data(file_path);

    games
        .into_iter()
        .map(|game| {
            let mut game = game;
            game.prize = (game.prize.0 + 10000000000000, game.prize.1 + 10000000000000);
            get_cost_for_game(game)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 480)]
    #[case(false, 39290)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 13, None)));
    }

    #[rstest]
    #[case(true, 875318608908)]
    #[case(false, 73458657399094)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: i64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 13, None)));
    }
}
