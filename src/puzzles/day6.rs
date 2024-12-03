use nom::{
    IResult,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
};
fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |digit_str: &str| digit_str.parse::<i32>())(input)
}
fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, parse_number)(input)
}


pub(crate) fn test() {
    let input = "";
    solve(input.to_string());
}

pub(crate) fn solve(input:String) {
    let mut result = 0;
    for line in input.lines() {
        let (_, numbers) = parse_line(line).expect("Failed to parse line");
        result += numbers.iter().sum::<i32>();
    }
    let part1: i32 = result;
    let part2: i32 = 0;

    println!("part1: {}, part2: {}", part1, part2);
}
