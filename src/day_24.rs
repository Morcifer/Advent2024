use crate::file_utilities::read_two_chunks;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use crate::day_24::Gate::Xor;

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

fn parse_data(
    file_path: String,
) -> (
    HashMap<String, usize>,
    HashMap<String, (String, Gate, String)>,
) {
    let (input_lines, gate_lines) = read_two_chunks(file_path);

    let inputs = input_lines
        .into_iter()
        .map(parse_input_line)
        .collect::<Vec<_>>();

    let gates = gate_lines
        .into_iter()
        .map(parse_gate_line)
        .collect::<Vec<_>>();

    (inputs.into_iter().collect(), gates.into_iter().collect())
}

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> usize {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

fn calculate_output(
    known_registers: &HashMap<String, usize>,
    connected_gates: &HashMap<String, (String, Gate, String)>,
) -> usize {
    let mut known_registers = known_registers.clone();

    let all_registers: HashSet<String> = connected_gates
        .iter()
        .flat_map(|(key, value)| vec![key.clone(), value.0.clone(), value.2.clone()])
        .collect();

    // println!("{:?}", all_registers);

    while known_registers.len() != all_registers.len() {
        let known_register_map: HashSet<String> = known_registers.keys().cloned().collect();
        let unknown_registers = all_registers.difference(&known_register_map).collect_vec();
        // println!("I have {} unknown registers left", unknown_registers.len());
        let mut found_match = false;

        for unknown_register in unknown_registers {
            let (from, gate, to) = connected_gates.get(unknown_register).unwrap();
            if let Some(from_value) = known_registers.get(from) {
                if let Some(to_value) = known_registers.get(to) {
                    let new_value = gate.calculate(*from_value, *to_value);
                    known_registers.insert(unknown_register.clone(), new_value);
                    found_match = true;
                }
            }
        }

        if !found_match {
            // println!("I'm stuck!");
            return 0;
        }
    }

    let output_keys = known_registers
        .keys()
        .filter(|key| key.starts_with("z"))
        .sorted()
        .rev()
        .collect_vec();

    let output = output_keys
        .into_iter()
        .map(|key| *known_registers.get(key).unwrap())
        .map(|value| value.to_string())
        .collect_vec()
        .join("");

    // println!("{output:?}");
    usize::from_str_radix(output.as_str(), 2).unwrap()
}

fn part_1(file_path: String) -> usize {
    let (known_registers, connected_gates) = parse_data(file_path);
    calculate_output(&known_registers, &connected_gates)
}

fn get_standard_adder(
    x_inputs: &Vec<String>,
    y_inputs: &Vec<String>,
) -> Vec<(String, (String, Gate, String))> {
    let x_inputs = x_inputs.clone();
    let y_inputs = y_inputs.clone();

    let total_input_bits = x_inputs.len();

    let mut adder = vec![
        (
            "z00".to_string(),
            ("y00".to_string(), Gate::Xor, "x00".to_string()),
        ),
        (
            "c00".to_string(),
            ("x00".to_string(), Gate::And, "y00".to_string()),
        ),
    ];

    for (index, (x_input, y_input)) in x_inputs
        .into_iter()
        .zip(y_inputs.into_iter())
        .enumerate()
        .skip(1)
    {
        let prev_index_string = format!("{:0>2}", index - 1);
        let prev_carry = format!("c{prev_index_string}");

        let index_string = format!("{index:0>2}");
        let temporary_sum = format!("s{index_string}");
        let temporary_carry_1 = format!("a{index_string}");
        let temporary_carry_2 = format!("b{index_string}");
        let output = format!("z{index_string}");

        // The last carry is just the last z.
        let carry = if index == total_input_bits - 1 {
            let next_index_string = format!("{:0>2}", index + 1);
            format!("s{next_index_string}")
        } else {
            format!("c{index_string}")
        };

        adder.push((
            temporary_sum.clone(),
            (y_input.clone(), Gate::Xor, x_input.clone()),
        ));
        adder.push((
            output,
            (prev_carry.clone(), Gate::Xor, temporary_sum.clone()),
        ));

        adder.push((
            temporary_carry_1.clone(),
            (y_input.clone(), Gate::And, x_input.clone()),
        ));
        adder.push((
            temporary_carry_2.clone(),
            (prev_carry.clone(), Gate::And, temporary_sum.clone()),
        ));
        adder.push((carry, (temporary_carry_2, Gate::Or, temporary_carry_1)));
    }

    adder
}

fn part_2(file_path: String) -> usize {
    let (known_registers, connected_gates) = parse_data(file_path);
    let output = calculate_output(&known_registers, &connected_gates);

    let x_inputs = known_registers
        .keys()
        .filter(|key| key.starts_with("x"))
        .sorted()
        .cloned()
        .collect_vec();

    let x = x_inputs
        .clone()
        .into_iter()
        .rev()
        .map(|key| *known_registers.get(&key).unwrap())
        .map(|value| value.to_string())
        .collect_vec()
        .join("");

    let x = usize::from_str_radix(x.as_str(), 2).unwrap();

    let y_inputs = known_registers
        .keys()
        .filter(|key| key.starts_with("y"))
        .sorted()
        .cloned()
        .collect_vec();

    let y = y_inputs
        .clone()
        .into_iter()
        .rev()
        .map(|key| *known_registers.get(&key).unwrap())
        .map(|value| value.to_string())
        .collect_vec()
        .join("");

    let y = usize::from_str_radix(y.as_str(), 2).unwrap();

    let expected_output = x + y;

    println!("x is {x:b} and y is {y:b} and we should get {expected_output:b} but get {output:b}");

    // let mut known_registers = known_registers;
    //
    // for y_input in y_inputs {
    //     if y_input == "y00" {
    //         known_registers.insert(y_input.clone(), 1);
    //     } else {
    //         known_registers.insert(y_input.clone(), 0);
    //     }
    // }
    //
    // let temp = x_inputs.len();
    //
    // for x in 0..2_usize.pow(10 as u32) {
    //     let x_binary = format!("{x:b}");
    //     // println!("{x}: {x_binary}");
    //
    //     for (index, char) in x_binary.chars().rev().enumerate() {
    //         let index_string = format!("{index:0>2}");
    //         let x_string = format!("x{index_string}");
    //         let value = char.to_digit(10).unwrap() as usize;
    //
    //         // println!("{x_string} is put on {value}");
    //         known_registers.insert(x_string, value);
    //     }
    //
    //     for index in x_binary.len()..temp {
    //         let index_string = format!("{index:0>2}");
    //         let x_string = format!("x{index_string}");
    //
    //         // println!("{x_string} is put on 0");
    //         known_registers.insert(x_string, 0);
    //     }
    //
    //     let output = calculate_output(&known_registers, &connected_gates);
    //     let expected_output = x + 1;
    //
    //     if output != expected_output {
    //         println!("When x = {x} ({x_binary}), we get {output} instead of {expected_output}");
    //         break;
    //     }
    // }

    // println!("{x_inputs:?}");
    // println!("{y_inputs:?}");
    //
    // I'm almost certain this is a standard binary adder circuit,
    // so I can program how it's supposed to look and find the mismatches.
    let adder: HashMap<String, (String, Gate, String)> = get_standard_adder(&x_inputs, &y_inputs)
        .into_iter()
        .collect();

    // println!("Expected");
    // for (key, value) in adder.iter() {
    //     println!("{key:?}: {value:?}");
    // }
    //
    // println!("Actual");
    // for (key, value) in connected_gates.iter() {
    //     println!("{key:?}: {value:?}");
    // }

    let z_outputs = adder
        .keys()
        .filter(|key| key.starts_with("z"))
        .sorted()
        .collect_vec();

    for z_output in z_outputs.iter() {
        let original = connected_gates.get(*z_output).unwrap();
        let expected = adder.get(*z_output).unwrap();

        if original.1 != expected.1 {
            println!("Mismatch {z_output:?}: {original:?} vs expected {expected:?}");
        }
    }

    let reverse_connected_gates: HashMap<(String, Gate, String), String> = connected_gates
        .iter()
        .map(|(key, value)| (value.clone(), key.clone()))
        .collect();

    let reverse_adder: HashMap<(String, Gate, String), String> = adder
        .iter()
        .map(|(key, value)| (value.clone(), key.clone()))
        .collect();

    for (x_input, y_input) in x_inputs.into_iter().zip(y_inputs.into_iter()).skip(1) {
        let xor_1 = (x_input.clone(), Gate::Xor, y_input.clone());
        let xor_2 = (y_input.clone(), Gate::Xor, x_input.clone());

        let and_1 = (x_input.clone(), Gate::And, y_input.clone());
        let and_2 = (y_input.clone(), Gate::And, x_input.clone());

        let xor_origin = reverse_connected_gates.get(&xor_1).unwrap_or_else(|| reverse_connected_gates.get(&xor_2).unwrap());
        let xor_expected = reverse_adder.get(&xor_1).unwrap_or_else(|| reverse_adder.get(&xor_2).unwrap());

        let and_origin = reverse_connected_gates.get(&and_1).unwrap_or_else(|| reverse_connected_gates.get(&and_2).unwrap());
        let and_expected = reverse_adder.get(&and_1).unwrap_or_else(|| reverse_adder.get(&and_2).unwrap());

        if z_outputs.contains(&xor_origin){
            println!("Mismatch {xor_1:?} or {xor_2:?}: {xor_origin:?} vs expected {xor_expected:?}");
            // Mismatch ("x00", Xor, "y00") or ("y00", Xor, "x00"): "z00" vs expected "z00"
        }

        if z_outputs.contains(&and_origin) {
            println!("Mismatch {and_1:?} or {and_2:?}: {and_origin:?} vs expected {and_expected:?}");
            // Mismatch ("x28", And, "y28") or ("y28", And, "x28"): "z28" vs expected "a28"
        }
    }

    for ((x, gate, y), z) in reverse_connected_gates.iter() {
        if *gate != Gate::Xor {
            continue;
        }

        let x_and_y = x.starts_with("x") && y.starts_with("y");
        let y_and_x = x.starts_with("y") && y.starts_with("x");

        if !z.starts_with("z") && !x_and_y && !y_and_x {
            println!("Mismatch for {:?}", ((x, gate, y), z));
        }
    }

    let known_problems = vec!["z08", "z28", "z39", "vvr", "mqh", "tfb"].into_iter().map(|s| s.to_string()).collect_vec();
    let mut option_checked = 0;

    for extra_problem_1 in connected_gates.keys() {
        for extra_problem_2 in connected_gates.keys() {
            let extra = vec![extra_problem_1.clone(), extra_problem_2.clone()];
            let all_problems: Vec<String> = known_problems.iter().chain(extra.iter()).cloned().collect_vec();

            if all_problems.iter().unique().count() < 8 {
                continue;
            }

            println!("Checking problems {all_problems:?}");
            return 0;

            for tuple_1 in all_problems.iter().cloned().tuple_combinations::<(_,_)>() {
                let less_problems: Vec<String> = all_problems.iter().filter(|a| **a != tuple_1.0 && **a != tuple_1.1).cloned().collect_vec();

                for tuple_2 in less_problems.iter().cloned().tuple_combinations::<(_,_)>() {
                    let even_less_problems: Vec<String> = less_problems.iter().filter(|a| **a != tuple_2.0 && **a != tuple_2.1).cloned().collect_vec();

                    for tuple_3 in even_less_problems.iter().cloned().tuple_combinations::<(_,_)>() {
                        let more_even_less_problems: Vec<String> = even_less_problems.iter().filter(|a| **a != tuple_3.0 && **a != tuple_3.1).cloned().collect_vec();

                        let tuple_4 = (more_even_less_problems[0].clone(), more_even_less_problems[1].clone());

                        option_checked += 1;
                        println!("Checking option {option_checked} tuples {tuple_1:?}, {tuple_2:?}, {tuple_3:?}, {tuple_4:?}");

                        let mut altered_connected_gates = connected_gates.clone();

                        let t_1_0 = altered_connected_gates.remove(&tuple_1.0).unwrap();
                        let t_1_1 = altered_connected_gates.remove(&tuple_1.1).unwrap();

                        altered_connected_gates.insert(tuple_1.0.clone(), t_1_1);
                        altered_connected_gates.insert(tuple_1.1.clone(), t_1_0);

                        let t_2_0 = altered_connected_gates.remove(&tuple_2.0).unwrap();
                        let t_2_1 = altered_connected_gates.remove(&tuple_2.1).unwrap();

                        altered_connected_gates.insert(tuple_2.0.clone(), t_2_1);
                        altered_connected_gates.insert(tuple_2.1.clone(), t_2_0);

                        let t_3_0 = altered_connected_gates.remove(&tuple_3.0).unwrap();
                        let t_3_1 = altered_connected_gates.remove(&tuple_3.1).unwrap();

                        altered_connected_gates.insert(tuple_3.0.clone(), t_3_1);
                        altered_connected_gates.insert(tuple_3.1.clone(), t_3_0);

                        let t_4_0 = altered_connected_gates.remove(&tuple_4.0).unwrap();
                        let t_4_1 = altered_connected_gates.remove(&tuple_4.1).unwrap();

                        altered_connected_gates.insert(tuple_4.0.clone(), t_4_1);
                        altered_connected_gates.insert(tuple_4.1.clone(), t_4_0);

                        let output = calculate_output(&known_registers, &altered_connected_gates);

                        if output == expected_output {
                            println!("{all_problems:?}");
                            return 42;
                        }
                    }
                }
            }
        }
    }

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
    #[case(false, 0)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: usize) {
        assert_eq!(expected, part_2(get_file_path(is_test, 24, None)));
    }
}
