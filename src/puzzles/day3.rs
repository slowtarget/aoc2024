use nom::{
    branch::alt,
    bytes::complete::{tag},
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_mul(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("mul")(input)?;
    let (input, _) = char('(')(input)?;
    let (input, (a, _, b)) = tuple((
        parse_integer,
        char(','),
        parse_integer,
    ))(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, (a, b)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        parse_do,
        parse_dont,
        map(parse_mul, |(a, b)| Instruction::Mul(a, b)),
    ))(input)
}

fn parse_all_instructions(input: &str) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    let mut remaining_input = input;
    let mut mul_enabled = true;

    while !remaining_input.is_empty() {
        let instruction = parse_instruction(remaining_input);
        match instruction {
            Ok((next_input, instruction)) => {
                match instruction {
                    Instruction::Do => {
                        mul_enabled = true;
                    }
                    Instruction::Dont => {
                        mul_enabled = false;
                    }
                    Instruction::Mul(a, b) => {
                        if mul_enabled {
                            res.push((a, b));
                        }
                    }
                }
                remaining_input = next_input;
            }
            Err(_) => {
                // Consume one character and continue parsing
                remaining_input = &remaining_input[1..];
            }
        }
    }

    res
}

fn parse_all_muls(input: &str) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    let mut remaining_input = input;

    while !remaining_input.is_empty() {
        match parse_mul(remaining_input) {
            Ok((next_input, tuple)) => {
                res.push(tuple);
                remaining_input = next_input;
            },
            Err(_) => {
                // Consume one character and continue parsing
                remaining_input = &remaining_input[1..];
            },
        }
    }

    res
}


pub(crate) fn test() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    solve(input.to_string()); //161
}

pub(crate) fn solve(input:String) {
    let part1: i32 = input.lines().flat_map(parse_all_muls).map(|(a, b)| a * b).sum::<i32>();
    let part2: i32 = parse_all_instructions(&*input).iter().map(|(a, b)| a * b).sum::<i32>();

    println!("part1: {}, part2: {}", part1, part2);
}
