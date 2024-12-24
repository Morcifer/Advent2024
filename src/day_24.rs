use crate::file_utilities::read_two_chunks;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    pub fn from_string(s: String) -> Self {
        match s.as_str() {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => panic!("Unknown gate type: {s}"),
        }
    }

    pub fn calculate(&self, input_1: usize, input_2: usize) -> usize {
        match self {
            // These work well with 0's and 1's, so no need to change to booleans.
            Gate::And => input_1 & input_2,
            Gate::Or => input_1 | input_2,
            Gate::Xor => input_1 ^ input_2,
        }
    }
}

fn parse_input_line(line: String) -> (String, usize) {
    let values = line.split(" ").map(str::trim).collect_vec();

    let gate = values[0][0..3].to_string();
    let value = values[1].parse::<usize>().unwrap();

    (gate, value)
}

fn parse_gate_line(line: String) -> (String, (String, Gate, String)) {
    let values = line.split(" ").map(str::trim).collect_vec();

    let input_1 = values[0].to_string();
    let gate = Gate::from_string(values[1].to_string());
    let input_2 = values[2].to_string();
    let output = values[4].to_string();

    (output, (input_1, gate, input_2))
}

fn parse_data(file_path: String) -> (Vec<(String, usize)>, Vec<(String, (String, Gate, String))>) {
    let (input_lines, gate_lines) = read_two_chunks(file_path);

    let inputs = input_lines
        .into_iter()
        .map(parse_input_line)
        .collect::<Vec<_>>();

    let gates = gate_lines
        .into_iter()
        .map(parse_gate_line)
        .collect::<Vec<_>>();

    (inputs, gates)
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> usize {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn part_1(file_path: String) -> usize {
    let (inputs, gates) = parse_data(file_path);

    let mut known_registers: HashMap<String, usize> = inputs.into_iter().collect();

    let connected_gates: HashMap<String, (String, Gate, String)> = gates.into_iter().collect();

    // println!("inputs {known_registers:?}");
    // println!("gates {connected_gates:?}");

    let all_registers: HashSet<String> = connected_gates
        .iter()
        .flat_map(|(key, value)| vec![key.clone(), value.0.clone(), value.2.clone()])
        .collect();

    // println!("{:?}", all_registers);

    while known_registers.len() != all_registers.len() {
        let known_register_map: HashSet<String> = known_registers.keys().cloned().collect();
        let unknown_registers = all_registers.difference(&known_register_map).collect_vec();
        println!("I have {} unknown registers left", unknown_registers.len());

        for unknown_register in unknown_registers {
            let (from, gate, to) = connected_gates.get(unknown_register).unwrap();
            if let Some(from_value) = known_registers.get(from) {
                if let Some(to_value) = known_registers.get(to) {
                    let new_value = gate.calculate(*from_value, *to_value);
                    known_registers.insert(unknown_register.clone(), new_value);
                }
            }
        }
    }

    let output_keys = known_registers
        .keys()
        .filter(|key| key.starts_with("z"))
        .sorted()
        .rev()
        .collect_vec();

    println!("{output_keys:?}");

    let output = output_keys
        .into_iter()
        .map(|key| *known_registers.get(key).unwrap())
        .map(|value| value.to_string())
        .collect_vec()
        .join("");

    println!("{output:?}");
    usize::from_str_radix(output.as_str(), 2).unwrap()
}

fn part_2(file_path: String) -> usize {
    let (_inputs, _gates) = parse_data(file_path);

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, 2024)]
    #[case(false, 58639252480880)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_1(get_file_path(is_test, 24, None)));
    }

    #[rstest]
    #[case(true, 0)]
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 24, None)));
    }
}
