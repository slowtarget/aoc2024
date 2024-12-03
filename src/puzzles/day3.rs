use nom::{
    bytes::complete::{tag},
    character::complete::{char, digit1},
    combinator::{map_res},
    sequence::{tuple},
    IResult,
};

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
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    solve(input.to_string()); //161
}

pub(crate) fn solve(input:String) {
    let mut result = 0;
    for line in input.lines() {
        let pairs = parse_all_muls(line);
        result += pairs.iter().map(|&(a, b)| a * b).sum::<i32>();
    }
    let part1: i32 = result;
    let part2: i32 = 0;

    println!("part1: {}, part2: {}", part1, part2);
}
