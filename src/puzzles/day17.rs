use crate::puzzles::day14;
use day14::parse_unsigned;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space0},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use num::pow;

/// Parse one line of the form: `Register X: 1234`
fn parse_register_line<'a>(input: &'a str, reg_name: &str) -> IResult<&'a str, i32> {
    let (input, _) = tag("Register ")(input)?;
    let (input, _) = tag(reg_name)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, val) = parse_unsigned(input)?;
    Ok((input, val as i32))
}

fn parse_registers(input: &str) -> IResult<&str, (i32, i32, i32)> {
    let (input, a) = parse_register_line(input, "A")?;
    let (input, _) = line_ending(input)?;

    let (input, b) = parse_register_line(input, "B")?;
    let (input, _) = line_ending(input)?;

    let (input, c) = parse_register_line(input, "C")?;
    Ok((input, (a, b, c)))
}

fn parse_program_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, _) = tag("\nProgram: ")(input)?;

    // separated_list1 will parse a list of items separated by a comma
    let (input, nums) = separated_list1(
        tag(","),                         // delimiter
        preceded(space0, parse_unsigned), // each integer, potentially preceded by optional spaces
    )(input)?;

    Ok((input, nums.iter().map(|&x| x as i32).collect()))
}

// -----------------------------------------------------------
// Define our data structures for the final parse result
// -----------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
struct Register {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Computer {
    store: Register,
    program: Vec<i32>,
}

use Instruction::*;
enum Instruction {
    Adv(i32),
    Bxl(i32),
    Bst(i32),
    Jnz(i32),
    Bxc(()),
    Out(i32),
    Bdv(i32),
    Cdv(i32),
}
impl Instruction {
    fn new(instruction: i32, operand: i32) -> Self {
        match instruction {
            0 => Adv(operand),
            1 => Bxl(operand),
            2 => Bst(operand),
            3 => Jnz(operand),
            4 => Bxc(()),
            5 => Out(operand),
            6 => Bdv(operand),
            7 => Cdv(operand),
            _ => panic!("Unknown instruction: {}", instruction),
        }
    }

    fn get_combo(operand: i32, register: &Register) -> i32 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => register.a,
            5 => register.b,
            6 => register.c,
            7 => panic!("Invalid operand: {}", operand),
            _ => panic!("Unknown instruction: {}", operand),
        }
    }

    fn act(&self, register: &Register, pointer: &usize) -> (Register, usize, Vec<i32>) {
        match self {
            Adv(operand) => (
                Register {
                    a: register.a / pow(2_i32, Self::get_combo(*operand, register) as usize),
                    b: register.b,
                    c: register.c,
                },
                pointer + 2,
                Vec::new(),
            ),
            Bxl(operand) => (
                Register {
                    a: register.a,
                    b: register.b ^ *operand,
                    c: register.c,
                },
                pointer + 2,
                Vec::new(),
            ),
            Bst(operand) => (
                Register {
                    a: register.a,
                    b: Self::get_combo(*operand, register) % 8,
                    c: register.c,
                },
                pointer + 2,
                Vec::new(),
            ),
            Jnz(operand) => (
                Register {
                    a: register.a,
                    b: register.b,
                    c: register.c,
                },
                {
                    if register.a != 0 {
                        *operand as usize
                    } else {
                        pointer + 2
                    }
                },
                Vec::new(),
            ),

            Bxc(_) => (
                Register {
                    a: register.a,
                    b: register.b ^ register.c,
                    c: register.c,
                },
                pointer + 2,
                Vec::new(),
            ),
            Out(operand) => (
                Register {
                    a: register.a,
                    b: register.b,
                    c: register.c,
                },
                pointer + 2,
                vec![Self::get_combo(*operand, register) % 8],
            ),
            Bdv(operand) => (
                Register {
                    a: register.a,
                    b: register.a / pow(2, Self::get_combo(*operand, register) as usize),
                    c: register.c,
                },
                pointer + 2,
                Vec::new(),
            ),
            Cdv(operand) => (
                Register {
                    a: register.a,
                    b: register.b,
                    c: register.a / pow(2, Self::get_combo(*operand, register) as usize),
                },
                pointer + 2,
                Vec::new(),
            ),
        }
    }
}

/// Parse the entire multi-line input into a `Computer`.
fn parse_input(input: &str) -> IResult<&str, Computer> {
    // 1) Parse the three registers
    let (input, (a, b, c)) = parse_registers(input)?;
    // There should be a line break after `Register C: <val>`
    let (input, _) = line_ending(input)?;
    println!("Registers: {a}, {b}, {c}");
    
    // 2) Parse the "Program: ..." line
    let (input, program) = parse_program_line(input)?;
    println!("Program: {program:?}");
    // Construct the final Computer object
    let computer = Computer {
        store: Register { a, b, c },
        program,
    };
    Ok((input, computer))
}

fn part_2(computer: &Computer) -> i32 {

    let program = computer.program.clone();
    let expected = program
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("Expected: {expected}");
    let mut a = 0;
    while expected != part_1(&Computer { store: Register { a, b: 0, c: 0 }, program: program.clone() }).0  {
        a += 1;
    }
    println!("A: {a}");
    a
}

fn part_1(computer: &Computer) -> (String, Register) {
    let mut ptr = 0;
    let mut output: Vec<i32> = Vec::new();
    let mut register = computer.store.clone();
    let program = computer.program.clone();
    while ptr < program.len() {
        let instruction_output: Vec<i32>;
        let instruction = Instruction::new(program[ptr], program[ptr + 1]);
        (register, ptr, instruction_output) = instruction.act(&register, &ptr);
        output.extend(instruction_output);
    }
    (
        output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(","),
        register,
    )
}

pub(crate) fn solve(input: String) -> (String, i32) {
    match parse_input(&*input) {
        Ok((remaining, computer)) => {
            println!("Parsed Computer: {computer:?}");
            println!("Remaining: {remaining:?}");

            (part_1(&computer).0, part_2(&computer)) //part_1 4,6,5,4,4,3,7,5,3 is not correct...
        }
        Err(err) => {
            eprintln!("Error parsing input: {err}");
            ("".to_string(), 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //     Adv(i32),
    #[test]
    fn adv_combo_test() {
        let instruction = Instruction::new(0, 5);
        let register = Register { a: 64, b: 5, c: 9 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 2, b: 5, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, Vec::new());
    }
    #[test]
    fn adv_literal_test() {
        let instruction = Instruction::new(0, 3);
        let register = Register { a: 64, b: 5, c: 9 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 8, b: 5, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, Vec::new());
    }
    //     Bxl(i32),
    #[test]
    fn bxl_test() {
        let instruction = Instruction::new(1, 6);
        let register = Register { a: 64, b: 6, c: 9 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 64, b: 0, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, Vec::new());
    }
    //     Bst(i32),
    #[test]
    fn bst_test() {
        let instruction = Instruction::new(2, 6);
        let (register, pointer, output) = instruction.act(&Register { a: 4, b: 0, c: 9 }, &0);
        assert_eq!(register, Register { a: 4, b: 1, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, Vec::new());
    }
    //     Jnz(i32),
    #[test]
    fn jnz_test() {
        let instruction = Instruction::new(3, 6);
        let register = &Register { a: 4, b: 0, c: 9 };
        let (register, pointer, output) = instruction.act(register, &0);
        assert_eq!(register, Register { a: 4, b: 0, c: 9 });
        assert_eq!(pointer, 6);
        assert_eq!(output, Vec::new());
    }
    #[test]
    fn jnz_0_test() {
        let instruction = Instruction::new(3, 6);
        let register = &Register { a: 0, b: 0, c: 9 };
        let (register, pointer, output) = instruction.act(register, &2);
        assert_eq!(register, Register { a: 0, b: 0, c: 9 });
        assert_eq!(pointer, 4);
        assert_eq!(output, Vec::new());
    }
    //     Bxc(()),
    #[test]
    fn bxc_test() {
        let instruction = Instruction::new(4, 6);
        let register = &Register { a: 0, b: 16, c: 14 };
        let (register, pointer, output) = instruction.act(register, &2);
        assert_eq!(register, Register { a: 0, b: 30, c: 14 });
        assert_eq!(pointer, 4);
        assert_eq!(output, Vec::new());
    }
    #[test]
    fn bxc_2_test() {
        let instruction = Instruction::new(4, 6);
        let register = &Register { a: 0, b: 12, c: 10 };
        let (register, pointer, output) = instruction.act(register, &2);
        assert_eq!(register, Register { a: 0, b: 6, c: 10 });
        assert_eq!(pointer, 4);
        assert_eq!(output, Vec::new());
    }
    //     Out(i32),
    #[test]
    fn out_combo_test() {
        let instruction = Instruction::new(5, 6);
        let register = Register { a: 12, b: 5, c: 64 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 12, b: 5, c: 64 });
        assert_eq!(pointer, 2);
        assert_eq!(output, vec![0]);
    }
    #[test]
    fn out_literal_test() {
        let instruction = Instruction::new(5, 3);
        let register = Register { a: 64, b: 5, c: 9 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 64, b: 5, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, vec![3]);
    }
    //     Bdv(i32),
    #[test]
    fn bdv_combo_test() {
        let instruction = Instruction::new(6, 6);
        let register = Register { a: 64, b: 10, c: 5 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 64, b: 2, c: 5 });
        assert_eq!(pointer, 2);
        assert_eq!(output, Vec::new());
    }
    #[test]
    fn bdv_literal_test() {
        let instruction = Instruction::new(6, 3);
        let register = Register { a: 64, b: 64, c: 9 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 64, b: 8, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, Vec::new());
    }
    //     Cdv(i32),
    #[test]
    fn cdv_combo_test() {
        let instruction = Instruction::new(7, 5);
        let register = Register { a: 64, b: 5, c: 64 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 64, b: 5, c: 2 });
        assert_eq!(pointer, 2);
        assert_eq!(output, Vec::new());
    }
    #[test]
    fn cdv_literal_test() {
        let instruction = Instruction::new(7, 3);
        let register = Register { a: 64, b: 5, c: 64 };
        let (register, pointer, output) = instruction.act(&register, &0);
        assert_eq!(register, Register { a: 64, b: 5, c: 8 });
        assert_eq!(pointer, 2);
        assert_eq!(output, Vec::new());
    }

    //     If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn computer_test() {
        let computer = Computer {
            store: Register { a: 10, b: 0, c: 0 },
            program: vec![5, 0, 5, 1, 5, 4],
        };
        let (output, _register) = part_1(&computer);
        assert_eq!(output, "0,1,2");
    }
    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    #[test]
    fn computer_2_test() {
        let computer = Computer {
            store: Register {
                a: 2024,
                b: 0,
                c: 0,
            },
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let (output, register) = part_1(&computer);
        assert_eq!(output, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(register.a, 0);
    }
    // If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn computer_3_test() {
        let computer = Computer {
            store: Register { a: 0, b: 29, c: 0 },
            program: vec![1, 7],
        };
        let (output, register) = part_1(&computer);
        assert_eq!(output, "");
        assert_eq!(register.b, 26);
    }
    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    #[test]
    fn computer_4_test() {
        let computer = Computer {
            store: Register {
                a: 0,
                b: 2024,
                c: 43690,
            },
            program: vec![4, 0],
        };
        let (output, register) = part_1(&computer);
        assert_eq!(output, "");
        assert_eq!(register.b, 44354);
    }
    // Register A: 729
    // Register B: 0
    // Register C: 0
    //
    // Program: 0,1,5,4,3,0
    #[test]
    fn provided_test() {
        let computer = Computer {
            store: Register { a: 729, b: 0, c: 0 },
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let (output, _register) = part_1(&computer);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }
    // Register A: 2024
    // Register B: 0
    // Register C: 0
    // 
    // Program: 0,3,5,4,3,0
    // This program outputs a copy of itself if register A is instead initialized to 117440. (The original initial value of register A, 2024, is ignored.)
    #[test]
    fn testing_part_1_using_part_2_provided_test() {
        let computer = Computer {
            store: Register { a: 117440, b: 0, c: 0 },
            program: vec![0,3,5,4,3,0],
        };
        let (output, _register) = part_1(&computer);
        assert_eq!(output, "0,3,5,4,3,0");
    }
    #[test]
    fn part_2_provided_test() {
        let computer = Computer {
            store: Register { a: 2024, b: 0, c: 0 },
            program: vec![0,3,5,4,3,0],
        };
        let result = part_2(&computer);
        assert_eq!(result, 117440);
    }
}
