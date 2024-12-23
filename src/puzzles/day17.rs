use crate::puzzles::day14;
use day14::parse_unsigned;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space0},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use std::time::{Duration, Instant};
use timing_util::measure_time;

/// Parse one line of the form: `Register X: 1234`
fn parse_register_line<'a>(input: &'a str, reg_name: &str) -> IResult<&'a str, u64> {
    let (input, _) = tag("Register ")(input)?;
    let (input, _) = tag(reg_name)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, val) = parse_unsigned(input)?;
    Ok((input, val as u64))
}

fn parse_registers(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (input, a) = parse_register_line(input, "A")?;
    let (input, _) = line_ending(input)?;

    let (input, b) = parse_register_line(input, "B")?;
    let (input, _) = line_ending(input)?;

    let (input, c) = parse_register_line(input, "C")?;
    Ok((input, (a, b, c)))
}

fn parse_program_line(input: &str) -> IResult<&str, Vec<u8>> {
    let (input, _) = tag("\nProgram: ")(input)?;

    // separated_list1 will parse a list of items separated by a comma
    let (input, nums) = separated_list1(
        tag(","),                         // delimiter
        preceded(space0, parse_unsigned), // each integer, potentially preceded by optional spaces
    )(input)?;

    Ok((input, nums.iter().map(|&x| x as u8).collect()))
}

// -----------------------------------------------------------
// Define our data structures for the final parse result
// -----------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
struct Register {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, PartialEq, Clone)]
struct Computer {
    store: Register,
    program: Vec<u8>,
}

use Instruction::*;
enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc(()),
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}
type InstrFn = Box<dyn Fn(&Register, usize) -> (Register, usize, u8)>;

impl Instruction {
    #[inline(always)]
    fn new(instruction: u8, operand: u8) -> Self {
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

    fn get_combo(operand: u8, register: &Register) -> u64 {
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
    #[inline(always)]
    fn act(&self) -> InstrFn {
        match self {
            Adv(operand) => {
                let op = *operand;
                Box::new(move |reg, ptr| {
                    (
                        Register {
                            a: reg.a / (1 << Instruction::get_combo(op, reg) as usize),
                            b: reg.b,
                            c: reg.c,
                        },
                        ptr + 2,
                        8,
                    )
                })
            }
            Bxl(operand) => {
                let op = *operand;
                Box::new(move |reg, ptr| {
                    (
                        Register {
                            a: reg.a,
                            b: reg.b ^ op as u64,
                            c: reg.c,
                        },
                        ptr + 2,
                        8,
                    )
                })
            }
            Bst(operand) => {
                let op = *operand;
                Box::new(move |reg, ptr| {
                    (
                        Register {
                            a: reg.a,
                            b: Self::get_combo(op, reg) & 7,
                            c: reg.c,
                        },
                        ptr + 2,
                        8,
                    )
                })
            }
            Jnz(operand) => {
                let op = *operand;
                Box::new(move |reg, ptr| {
                    (
                        Register {
                            a: reg.a,
                            b: reg.b,
                            c: reg.c,
                        },
                        {
                            if reg.a != 0 {
                                op as usize
                            } else {
                                ptr + 2
                            }
                        },
                        8,
                    )
                })
            }

            Bxc(_) => Box::new(move |reg, ptr| {
                (
                    Register {
                        a: reg.a,
                        b: reg.b ^ reg.c,
                        c: reg.c,
                    },
                    ptr + 2,
                    8,
                )
            }),
            Out(operand) => {
                let op = *operand;
                Box::new(move |reg, ptr| {
                    (
                        Register {
                            a: reg.a,
                            b: reg.b,
                            c: reg.c,
                        },
                        ptr + 2,
                        (Instruction::get_combo(op, reg) & 7) as u8,
                    )
                })
            }
            Bdv(operand) => {
                let op = *operand;
                Box::new(move |reg, ptr| {
                    (
                        Register {
                            a: reg.a,
                            b: reg.a / (1 << Self::get_combo(op, reg) as usize),
                            c: reg.c,
                        },
                        ptr + 2,
                        8,
                    )
                })
            }
            Cdv(operand) => {
                let op = *operand;
                Box::new(move |reg, ptr| {
                    (
                        Register {
                            a: reg.a,
                            b: reg.b,
                            c: reg.a / (1 << Self::get_combo(op, reg) as usize),
                        },
                        ptr + 2,
                        8,
                    )
                })
            }
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
// A helper to format a Duration in a friendlier, more readable way:
fn format_friendly_duration(dur: Duration) -> String {
    let secs = dur.as_secs();
    let millis = dur.subsec_millis();

    // For multi-component output, break it down:
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    let secs = secs % 60;

    if hours > 0 {
        format!("{:02}h {:02}m {:02}.{:03}s", hours, mins, secs, millis)
    } else if mins > 0 {
        format!("{:02}m {:02}.{:03}s", mins, secs, millis)
    } else {
        format!("{}.{:03}s", secs, millis)
    }
}
//inline

fn run_program(program: &[u8], r: &mut Register) -> Vec<u8> {
    let mut ptr = 0;
    let mut output = Vec::new();

    while ptr < program.len() && program.len() >= output.len() &&  program[0..output.len()] == output[..] {
        let opcode = program[ptr];
        let operand = program[ptr + 1];
        match opcode {
            0 => { // ADV
                let shift = Instruction::get_combo(operand, &r);
                r.a >>= shift;
                ptr += 2;
            }
            1 => { // BXL
                r.b ^= operand as u64;
                ptr += 2;
            }
            2 => { // BST
                r.b = Instruction::get_combo(operand, &r) & 7;
                ptr += 2;
            }
            3 => { // JNZ
                if r.a != 0 {
                    ptr = operand as usize;
                } else {
                    ptr += 2;
                }
            }
            4 => { // BXC
                r.b ^= r.c;
                ptr += 2;
            }
            5 => {
                // OUT
                let val = Instruction::get_combo(operand, &r) & 7;
                output.push(val as u8);
                ptr += 2;
            }
            6 => { // BDV
                let shift = Instruction::get_combo(operand, &r);
                r.b = r.a >> shift;
                ptr += 2;
            }
            7 => { // CDV
                let shift = Instruction::get_combo(operand, &r);
                r.c = r.a >> shift;
                ptr += 2;
            }
            // etc...
            _ => {}
        }
    }

    output
}

/// Returns all possible `a` values at iteration i=0 that yield `digits` 
/// by the time we get to i=N (N digits).
fn backward_solve(digits: &[u8]) -> Vec<u64> {
    // We'll keep a "set of possible a-values" after the i-th digit (counting from the right).
    // Start with i = N: after all digits have been produced, a might be anything, 
    // but let's say we start with {0} as a baseline if the loop ended with a=0 
    // (or we might allow a range).
    //
    // Actually, if the loop ended naturally, `a` could be zero (the while condition breaks). 
    // But if you want an exact chain of length N, you can allow any final `a`.
    // Let’s just do a BFS or back-propagation for one possibility: final a = 0.
    //
    use std::collections::HashSet;

    let mut possible_as = HashSet::new();
    // Suppose the final a is 0 after producing digits[N-1].
    // You could also store multiple possible final `a`s if needed.
    possible_as.insert(0u64);

    // We'll go backwards from i = N-1 down to i = 0
    let operations = &get_operations(digits);
    for (i,&digit) in digits.iter().enumerate().rev() {
        let program = digits[i..].to_vec();
        println!("{} Program: {program:?}",possible_as.len());


        let mut new_possible_as = HashSet::new();
        for &a_next in &possible_as {
            // We'll find all a_current that produce `digit` and yield `a_next` after >>3.
            // a_current must be in [a_next<<3 .. a_next<<3+7].
            let base = a_next << 3;
            let top = 1 << 3;
            // println!("Base: {base}, Top: {top} = {}", base+top);
            for low3 in 0..top {
                let a_candidate = base + low3;
                let (output, _register) =
                    part_1(&program, &Register { a: a_candidate, b: 0, c: 0 }, operations, true);
                if output == program {
                    new_possible_as.insert(a_candidate);
                    // println!("Match!: {program:?} -> {a_candidate} ({} -> {})", possible_as.len(), new_possible_as.len());
                }
            }
        }
        possible_as = new_possible_as;
        if possible_as.is_empty() {
            // No solutions for this chain
            break;
        }
    }

    // After the loop, `possible_as` contains all a-values that, if we run the forward loop,
    // produce exactly `digits`.
    possible_as.into_iter().collect()
}

fn run_my_program(program: &[u8], r: &mut Register) -> Vec<u8> {
    let mut output = Vec::with_capacity(program.len());
    let Register {mut a, mut b, mut c} = r;
    while a != 0 && program.len() >= output.len() &&  program[0..output.len()] == output[..] {
        b = a & 7 ^ 7;
        c = a >> b;
        a >>= 3;
        b ^= c ^ 7;
        output.push((b & 7) as u8);
    }
    output
}
fn part_2(computer: &Computer) -> Vec<u64> {
    let program = computer.program.clone();
    let start = Instant::now();
    let expected = program.clone();
    println!("Expected: {expected:?}");
    // let mut a =  86167781376;
    // while expected != run_my_program(&computer.program,  &mut Register { a, b: 0, c: 0 }) {
    //     if a & 0xFFFFFFFF == 0 {
    //         let elapsed = Instant::now().duration_since(start);
    //         println!("{} A: {a}", format_friendly_duration(elapsed));
    //     }
    //     a += 1;
    // }
    let a = backward_solve(&expected);
    println!("A: {a:?}");
    a
}
fn get_operations(program: &[u8]) -> Vec<InstrFn> {
    let mut operations: Vec<InstrFn> = Vec::new();
    let mut ptr = 0;
    while ptr < program.len() {
        let instruction = Instruction::new(program[ptr], program[ptr + 1]);
        operations.push(instruction.act());
        ptr += 2;
    }
    operations
}
fn part_1(program: &[u8], register: &Register, operations: &Vec<InstrFn>, match_program: bool) -> (Vec<u8>, Register) {
    let mut ptr = 0;
    let mut output: Vec<u8> = Vec::new();
    let mut register = register.clone();
    while ptr < operations.len() * 2 && (!match_program || (program.len() >= output.len() &&  program[0..output.len()] == output[..])) {
        let instruction_output: u8;
        (register, ptr, instruction_output) = operations[ptr / 2](&register, ptr);
        if instruction_output != 8 {
            output.push(instruction_output);
        }
    }
    (output, register)
}

pub(crate) fn solve(input: String) -> (String, u64) {
    match parse_input(&*input) {
        Ok((remaining, computer)) => {
            println!("Parsed Computer: {computer:?}");
            println!("Remaining: {remaining:?}");
            let mut part_2 = measure_time!(part_2(&computer));
            part_2.sort();
            (
                part_1(
                    &computer.program,
                    &computer.store,
                    &get_operations(&computer.program),
                    false,
                )
                .0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
                part_2[0],
            ) //part_1 4,6,5,4,4,3,7,5,3 is not correct...
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
    use timing_util::measure_time;
    //     Adv(i32),
    #[test]
    fn adv_combo_test() {
        let instruction = Instruction::new(0, 5);
        let register = Register { a: 64, b: 5, c: 9 };
        let (register, pointer, output) = instruction.act()(&register, 0);
        assert_eq!(register, Register { a: 2, b: 5, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 8);
    }
    #[test]
    fn adv_literal_test() {
        let instruction = Instruction::new(0, 3);
        let register = Register { a: 64, b: 5, c: 9 };
        let (register, pointer, output) = instruction.act()(&register, 0);
        assert_eq!(register, Register { a: 8, b: 5, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 8);
    }
    //     Bxl(i32),
    #[test]
    fn bxl_test() {
        let instruction = Instruction::new(1, 6);
        let register = Register { a: 64, b: 6, c: 9 };
        let (register, pointer, output) = instruction.act()(&register, 0);

        assert_eq!(register, Register { a: 64, b: 0, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 8);
    }
    //     Bst(i32),
    #[test]
    fn bst_test() {
        let instruction = Instruction::new(2, 6);
        let register = Register { a: 4, b: 1, c: 9 };
        let (register, pointer, output) = instruction.act()(&register, 0);
        assert_eq!(register, Register { a: 4, b: 1, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 8);
    }
    //     Jnz(i32),
    #[test]
    fn jnz_test() {
        let instruction = Instruction::new(3, 6);
        let register = &Register { a: 4, b: 0, c: 9 };
        let (register, pointer, output) = instruction.act()(&register, 0);

        assert_eq!(register, Register { a: 4, b: 0, c: 9 });
        assert_eq!(pointer, 6);
        assert_eq!(output, 8);
    }
    #[test]
    fn jnz_0_test() {
        let instruction = Instruction::new(3, 6);
        let register = &Register { a: 0, b: 0, c: 9 };
        let (register, pointer, output) = instruction.act()(&register, 2);

        assert_eq!(register, Register { a: 0, b: 0, c: 9 });
        assert_eq!(pointer, 4);
        assert_eq!(output, 8);
    }
    //     Bxc(()),
    #[test]
    fn bxc_test() {
        let instruction = Instruction::new(4, 6);
        let register = &Register { a: 0, b: 16, c: 14 };
        let (register, pointer, output) = instruction.act()(&register, 2);

        assert_eq!(register, Register { a: 0, b: 30, c: 14 });
        assert_eq!(pointer, 4);
        assert_eq!(output, 8);
    }
    #[test]
    fn bxc_2_test() {
        let instruction = Instruction::new(4, 6);
        let register = &Register { a: 0, b: 12, c: 10 };
        let (register, pointer, output) = instruction.act()(&register, 2);

        assert_eq!(register, Register { a: 0, b: 6, c: 10 });
        assert_eq!(pointer, 4);
        assert_eq!(output, 8);
    }
    //     Out(i32),
    #[test]
    fn out_combo_test() {
        let instruction = Instruction::new(5, 6);
        let register = Register { a: 12, b: 5, c: 64 };
        let (register, pointer, output) = instruction.act()(&register, 0);

        assert_eq!(register, Register { a: 12, b: 5, c: 64 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 0);
    }
    #[test]
    fn out_literal_test() {
        let instruction = Instruction::new(5, 3);
        let register = Register { a: 64, b: 5, c: 9 };
        let (register, pointer, output) = instruction.act()(&register, 0);

        assert_eq!(register, Register { a: 64, b: 5, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 3);
    }
    //     Bdv(i32),
    #[test]
    fn bdv_combo_test() {
        let instruction = Instruction::new(6, 6);
        let register = Register { a: 64, b: 10, c: 5 };
        let (register, pointer, output) = instruction.act()(&register, 0);

        assert_eq!(register, Register { a: 64, b: 2, c: 5 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 8);
    }
    #[test]
    fn bdv_literal_test() {
        let instruction = Instruction::new(6, 3);
        let register = Register { a: 64, b: 64, c: 9 };
        let (register, pointer, output) = instruction.act()(&register, 0);

        assert_eq!(register, Register { a: 64, b: 8, c: 9 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 8);
    }
    //     Cdv(i32),
    #[test]
    fn cdv_combo_test() {
        let instruction = Instruction::new(7, 5);
        let register = Register { a: 64, b: 5, c: 64 };
        let (register, pointer, output) = instruction.act()(&register, 0);

        assert_eq!(register, Register { a: 64, b: 5, c: 2 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 8);
    }
    #[test]
    fn cdv_literal_test() {
        let instruction = Instruction::new(7, 3);
        let register = Register { a: 64, b: 5, c: 64 };
        let (register, pointer, output) = instruction.act()(&register, 0);

        assert_eq!(register, Register { a: 64, b: 5, c: 8 });
        assert_eq!(pointer, 2);
        assert_eq!(output, 8);
    }

    //     If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn computer_test() {
        let computer = Computer {
            store: Register { a: 10, b: 0, c: 0 },
            program: vec![5, 0, 5, 1, 5, 4],
        };
        let (output, _register) =
            part_1(&computer.program, &computer.store, &get_operations(&computer.program), false);
        assert_eq!(output, [0, 1, 2]);
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
        let (output, register) =
            part_1(&computer.program, &computer.store, &get_operations(&computer.program), false);
        assert_eq!(output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(register.a, 0);
    }
    // If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn computer_3_test() {
        let computer = Computer {
            store: Register { a: 0, b: 29, c: 0 },
            program: vec![1, 7],
        };
        let (output, register) =
            part_1(&computer.program, &computer.store, &get_operations(&computer.program), false);
        assert_eq!(output, []);
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
        let (output, register) =
            part_1(&computer.program, &computer.store, &get_operations(&computer.program), false);
        assert_eq!(output, []);
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
        let (output, _register) =
            part_1(&computer.program, &computer.store, &get_operations(&computer.program), false);
        assert_eq!(output, [4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
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
            store: Register {
                a: 117440,
                b: 0,
                c: 0,
            },
            program: vec![0, 3, 5, 4, 3, 0],
        };
        let (output, _register) =
            part_1(&computer.program, &computer.store, &get_operations(&computer.program), false);
        assert_eq!(output, [0, 3, 5, 4, 3, 0]);
    } 
    
    fn testing_part_1_using_part_2_2_provided_test() {
        let computer = Computer {
            store: Register {
                a: 238991,
                b: 0,
                c: 0,
            },
            program: vec![0, 3, 5, 4, 3, 0],
        };
        let (output, _register) =
            part_1(&computer.program, &computer.store, &get_operations(&computer.program), false);
        assert_eq!(output, [0, 3, 5, 4, 3, 0]);
    }
    #[test]
    fn part_2_provided_test() {
        let computer = Computer {
            store: Register {
                a: 2024,
                b: 0,
                c: 0,
            },
            program: vec![0, 3, 5, 4, 3, 0],
        };
        let mut result = measure_time!(part_2(&computer));
        result.sort();
        assert_eq!(result[0], 117440);
    }
}
