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

fn part_1(file_path: String) -> usize {
    let (mut known_registers, connected_gates) = parse_data(file_path);

    let all_registers: HashSet<String> = connected_gates
        .iter()
        .flat_map(|(key, value)| vec![key.clone(), value.0.clone(), value.2.clone()])
        .collect();

    // println!("{:?}", all_registers);

    while known_registers.len() != all_registers.len() {
        let known_register_map: HashSet<String> = known_registers.keys().cloned().collect();
        let unknown_registers = all_registers.difference(&known_register_map).collect_vec();
        // println!("I have {} unknown registers left", unknown_registers.len());

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

    let output = output_keys
        .into_iter()
        .map(|key| *known_registers.get(key).unwrap())
        .map(|value| value.to_string())
        .collect_vec()
        .join("");

    // println!("{output:?}");
    usize::from_str_radix(output.as_str(), 2).unwrap()
}

fn get_standard_adder(
    x_inputs: &Vec<&String>,
    y_inputs: &Vec<&String>,
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

    let x_inputs = known_registers
        .keys()
        .filter(|key| key.starts_with("x"))
        .sorted()
        .collect_vec();

    let y_inputs = known_registers
        .keys()
        .filter(|key| key.starts_with("y"))
        .sorted()
        .collect_vec();

    // println!("{x_inputs:?}");
    // println!("{y_inputs:?}");

    // I'm almost certain this is a standard binary adder circuit,
    // so I can program how it's supposed to look and find the mismatches.
    let adder: HashMap<String, (String, Gate, String)> = get_standard_adder(&x_inputs, &y_inputs)
        .into_iter()
        .collect();

    println!(
        "input {connected_gates:?} has length {}",
        connected_gates.len()
    );
    println!("expected {adder:?} has length {}", adder.len());

    let z_outputs = adder
        .keys()
        .filter(|key| key.starts_with("z"))
        .sorted()
        .collect_vec();

    let mut name_mapping: HashMap<String, String> = HashMap::new();

    for z_output in z_outputs.iter() {
        let original = connected_gates.get(*z_output).unwrap();
        let expected = adder.get(*z_output).unwrap();

        if original.1 != expected.1 {
            println!("Mismatch {z_output:?}: {original:?} vs expected {expected:?}");
            // Mismatch "z28": ("y28", And, "x28") vs expected ("c27", Xor, "s28")
            // Mismatch "z08": ("kwv", Or, "ctv") vs expected ("c07", Xor, "s08")
            // Mismatch "z39": ("thk", And, "wnk") vs expected ("c38", Xor, "s39")
        }
        else {
            println!("Matching-maybe {z_output:?}: {original:?} vs expected {expected:?}");
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
