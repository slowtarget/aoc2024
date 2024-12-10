use nom::{
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        map_res(digit1, str::parse),
        space1,
        map_res(digit1, str::parse),
    )(input)
}

pub(crate) fn test() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";
    solve(input.to_string()); // 11
}

pub(crate) fn solve(input: String) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let (_, (a, b)) = parse_line(line).expect("Failed to parse line");
        left.push(a);
        right.push(b);
    }

    left.sort();
    right.sort();

    let part1: i32 = sum_of_differences(&left, &right);
    let part2: i32 = similarity_score(&left, &right);

    println!("part1: {}, part2: {}", part1, part2);
}

fn sum_of_differences(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();
    for value in right {
        *frequency_map.entry(value).or_insert(0) += 1;
    }
    left.iter()
        .map(|&value| value * frequency_map.get(&value).unwrap_or(&0))
        .sum()
}
