use itertools::Itertools;

#[allow(dead_code)]
pub fn run(file_path: String, part: i32) -> String {
    match part {
        1 => part_1(file_path),
        2 => part_2(file_path),
        _ => panic!("... nope."),
    }
}

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
            _ => panic!("Op code {op_code} shouldn't be possible!")
        }
    }
}

struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    program: Vec<u64>,
    instruction_pointer: usize,
    outputs: Vec<u64>,
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
        let operator = Operator::from_op_code(self.program[self.instruction_pointer]);
        let operand = self.get_operand(self.program[self.instruction_pointer + 1]);

        match operator {
            Operator::Adv => {
                let result = self.divide(operand);
                self.register_a = result;
            },
            Operator::Bxl => {
                let result = self.register_b ^ operand;
                self.register_b = result;
            },
            Operator::Bst => {
                let result = operand % 8;
                self.register_b = result;
            },
            Operator::Jnz => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand as usize;
                    return;
                }
            },
            Operator::Bxc => {
                let result = self.register_b ^ self.register_c;
                self.register_b = result;
            },
            Operator::Out => {
                let result = operand % 8;
                self.outputs.push(result);
            },
            Operator::Bdv => {
                let result = self.divide(operand);
                self.register_b = result;
            },
            Operator::Cdv => {
                let result = self.divide(operand);
                self.register_c = result;
            },
        }

        self.instruction_pointer += 2;
    }

    fn divide(&self, operand: u64) -> u64 {
        let numerator = self.register_a;
        let denominator = 2_u64.pow(operand as u32);
        let result = numerator / denominator;

        result
    }

    fn run_to_end(&mut self) -> Vec<u64> {
        while self.instruction_pointer < self.program.len() {
            self.run_instruction();
        }

        self.outputs.clone()
    }
}

const TEST_CASE: (u64, u64, u64, &[u64]) = (729, 0, 0, &[0,1,5,4,3,0]);
const REAL_CASE: (u64, u64, u64, &[u64]) = (66171486, 0, 0, &[2,4,1,6,7,5,4,6,1,4,5,5,0,3,3,0]);

fn part_1(file_path: String) -> String {
    let case_data = if file_path.contains("test") {
        TEST_CASE
    } else {
        REAL_CASE
    };

    let mut computer = Computer::new(case_data.0, case_data.1, case_data.2, case_data.3.to_vec());

    let output = computer.run_to_end();

    output.into_iter().map(|n| n.to_string()).collect_vec().into_iter().join(",")
}

fn part_2(file_path: String) -> String {
    let case_data = if file_path.contains("test") {
        TEST_CASE
    } else {
        REAL_CASE
    };

    let mut _computer = Computer::new(case_data.0, case_data.1, case_data.2, case_data.3.to_vec());

    "Not yet!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_utilities::get_file_path;
    use rstest::rstest;

    // #[rstest]
    // #[case(vec![125, 17], 6, 22)]
    // fn test_blink_many_times(
    //     #[case] stones: Vec<u128>,
    //     #[case] times: usize,
    //     #[case] expected: usize,
    // ) {
    //     assert_eq!(expected, blink_many_times(stones, times));
    // }

    #[rstest]
    #[case(true, 55312)]
    #[case(false, 183435)]
    fn test_part_1(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_1(get_file_path(is_test, 11, None)));
    }

    #[rstest]
    #[case(false, 218279375708592)]
    fn test_part_2(#[case] is_test: bool, #[case] expected: u64) {
        assert_eq!(expected, part_2(get_file_path(is_test, 11, None)));
    }
}
