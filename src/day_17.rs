use itertools::Itertools;

// TODO: real input from file!

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> String {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Operator {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Operator {
    fn from_op_code(op_code: u64) -> Operator {
        match op_code {
            0 => Operator::Adv,
            1 => Operator::Bxl,
            2 => Operator::Bst,
            3 => Operator::Jnz,
            4 => Operator::Bxc,
            5 => Operator::Out,
            6 => Operator::Bdv,
            7 => Operator::Cdv,
            _ => panic!("Op code {op_code} shouldn't be possible!"),
        }
    }
}

struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    program: Vec<u64>,
    instruction_pointer: usize,
    pub outputs: Vec<u64>,
}

impl Computer {
    fn new(a: u64, b: u64, c: u64, program: Vec<u64>) -> Self {
        Self {
            register_a: a,
            register_b: b,
            register_c: c,
            program,
            instruction_pointer: 0,
            outputs: vec![],
        }
    }

    fn get_operand(&self, operand: u64) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => panic!("7 is reserved!"),
            _ => panic!("How did you get here?"),
        }
    }

    fn run_instruction(&mut self) {
        // println!("Running instructions at {}", self.instruction_pointer);

        let operator = Operator::from_op_code(self.program[self.instruction_pointer]);
        let literal_operand = self.program[self.instruction_pointer + 1];
        let combo_operand = self.get_operand(literal_operand);

        // println!("Operator {operator:?}, literal operand {literal_operand}, combo operand {combo_operand}");

        match operator {
            Operator::Adv => {
                let result = self.divide(combo_operand);
                self.register_a = result;
            }
            Operator::Bxl => {
                let result = self.register_b ^ literal_operand;
                self.register_b = result;
            }
            Operator::Bst => {
                let result = combo_operand % 8;
                self.register_b = result;
            }
            Operator::Jnz => {
                if self.register_a != 0 {
                    self.instruction_pointer = literal_operand as usize;
                    return;
                }
            }
            Operator::Bxc => {
                let result = self.register_b ^ self.register_c;
                self.register_b = result;
            }
            Operator::Out => {
                let result = combo_operand % 8;
                self.outputs.push(result);
            }
            Operator::Bdv => {
                let result = self.divide(combo_operand);
                self.register_b = result;
            }
            Operator::Cdv => {
                let result = self.divide(combo_operand);
                self.register_c = result;
            }
        }

        self.instruction_pointer += 2;
    }

    fn divide(&self, operand: u64) -> u64 {
        let numerator = self.register_a;
        numerator >> operand
    }

    fn run_to_end(&mut self) -> Vec<u64> {
        while self.instruction_pointer < self.program.len() {
            self.run_instruction();
        }

        self.outputs.clone()
    }
}

const TEST_CASE: (u64, &[u64]) = (729, &[0, 1, 5, 4, 3, 0]);
const SECOND_TEST_CASE: (u64, &[u64]) = (2024, &[0, 3, 5, 4, 3, 0]);
const REAL_CASE: (u64, &[u64]) = (66171486, &[2, 4, 1, 6, 7, 5, 4, 6, 1, 4, 5, 5, 0, 3, 3, 0]);

fn part_1(file_path: String) -> String {
    let case_data = if file_path.contains("test") {
        TEST_CASE
    } else {
        REAL_CASE
    };

    let mut computer = Computer::new(case_data.0, 0, 0, case_data.1.to_vec());

    let output = computer.run_to_end();

    output
        .into_iter()
        .map(|n| n.to_string())
        .collect_vec()
        .into_iter()
        .join(",")
}

fn part_2(file_path: String) -> String {
    let is_test = file_path.contains("test");
    let case_data = if is_test { SECOND_TEST_CASE } else { REAL_CASE };

    let needed_output_values = case_data.1.iter().copied().rev().collect_vec();

    let mut possible_solutions = vec![0];

    for needed_output_value in needed_output_values.into_iter() {
        let mut new_possible_solutions = vec![];

        for possible_solution in possible_solutions {
            let starting_value = possible_solution * 8;

            for offset in 0..8 {
                let a = starting_value + offset;

                if a == 0 {
                    // Don't need to bother with an actual 0.
                    continue;
                }

                let mut computer = Computer::new(a, 0, 0, case_data.1.to_vec());
                let output = computer.run_to_end();

                // println!(
                //     "For input {a} ({a:b}), compare program {:?} to output {output:?}",
                //     case_data.1.to_vec(),
                // );

                if output[0] == needed_output_value {
                    // println!("{a} ({a:b}) counts as a possible solution!");
                    new_possible_solutions.push(a);
                }
            }
        }

        possible_solutions = new_possible_solutions;
    }

    possible_solutions[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    #[rstest]
    #[case(true, "4,6,3,5,6,3,5,2,1,0")]
    #[case(false, "2,3,6,2,1,6,1,2,1")]
    fn test_part_1(#[case] is_test: bool, #[case] expected: String) {
        assert_eq!(expected, part_1(get_file_path(is_test, 17, None)));
    }

    #[rstest]
    #[case(true, "117440")]
    #[case(false, "90938893795561")]
    fn test_part_2(#[case] is_test: bool, #[case] expected: String) {
        assert_eq!(expected, part_2(get_file_path(is_test, 17, None)));
    }
}
