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
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    solve(input.to_string());
    solve("100 1 2 3 4 5 6".to_string());
    solve("1 1 2 3 4 5 6".to_string());
    solve("1 2 3 3 4 5 6".to_string());
    solve("1 2 3 4 5 6 0".to_string());
    solve("1 2 3 7 5 6".to_string());

}

pub(crate) fn solve(input:String) {

    let mut count_part1 = 0;
    let mut count_part2 = 0;
    for line in input.lines() {
        let (_, numbers) = parse_line(line).expect("Failed to parse line");
        if safe_part1(&numbers) {
            count_part1 +=1;
        }
        if safe_part2(&numbers) {
            count_part2 +=1;
        }

    }

    let part1: i32 = count_part1;
    let part2: i32 = count_part2;

    println!("part1: {}, part2: {}", part1, part2);
}

fn safe_part1(numbers: &Vec<i32>) -> bool {
    let mut index = 1;

    let mut difference = (numbers[index] - numbers[index-1]).abs();
    let increasing = list_increasing(&numbers);
    let mut direction = numbers[index] > numbers[index-1];
    while difference <= 3 && difference > 0 && direction == increasing && index < numbers.len() {
        difference = (numbers[index] - numbers[index-1]).abs();
        direction = numbers[index] > numbers[index-1];
        index += 1;
    }
    let is_safe = difference <= 3 && difference > 0 && direction == increasing;
    is_safe
}

fn safe_part2(numbers: &Vec<i32>) -> bool {

    if safe_part1(&numbers) {
        return true;
    }

    for i in 0..numbers.len() {
        let mut modified_numbers = numbers.clone();
        modified_numbers.remove(i);
        if safe_part1(&modified_numbers) {
            return true;
        }
    }

    println!("unsafe: {:?}", numbers);
    false
}

fn list_increasing(numbers: &Vec<i32>) -> bool {
    let mut increasing = 0;
    let mut decreasing = 0;
    let mut i = 1;
    while i < numbers.len() && increasing < 2 && decreasing < 2 {
        if numbers[i] > numbers[i-1] {
            increasing += 1;
        } else {
            decreasing += 1;
        }
        i += 1;
    }
    increasing > 1
}

