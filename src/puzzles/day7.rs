use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{map_res, opt},
    error::Error,
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::time::Instant;

fn parse_i64(input: &str) -> IResult<&str, i64, Error<&str>> {
    let (input, num_str) = digit1::<&str, Error<&str>>(input)?;
    let val = num_str.parse::<i64>().unwrap();
    Ok((input, val))
}

fn parse_nums(input: &str) -> IResult<&str, Vec<i64>, Error<&str>> {
    let (mut input, _) = space0::<&str, Error<&str>>(input)?;
    let mut result = Vec::new();
    while let Ok((new_input, num_val)) =
        map_res(digit1::<&str, Error<&str>>, |s: &str| s.parse::<i64>())(input)
    {
        result.push(num_val);
        input = new_input;
        let (new_input, _) = space0::<&str, Error<&str>>(input)?;
        input = new_input;
    }
    Ok((input, result))
}

fn parse_line(input: &str) -> IResult<&str, (i64, Vec<i64>), Error<&str>> {
    let (input, (test_val, nums_line)) =
        nom::sequence::separated_pair(parse_i64, tag(":"), parse_nums)(input)?;
    Ok((input, (test_val, nums_line)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(i64, Vec<i64>)>, Error<&str>> {
    many1(terminated(parse_line, opt(tag("\n"))))(input)
}

fn can_make_target_part1(target: i64, nums: &[i64]) -> bool {
    if nums.is_empty() {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == target;
    }
    fn recurse(target: i64, nums: &[i64]) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }
        let last = nums[nums.len() - 1];
        let add_candidate = target - last;
        if add_candidate >= 0 && recurse(add_candidate, &nums[..nums.len() - 1]) {
            return true;
        }
        if last != 0 && target % last == 0 {
            let mul_candidate = target / last;
            if recurse(mul_candidate, &nums[..nums.len() - 1]) {
                return true;
            }
        }
        false
    }
    recurse(target, nums)
}

fn part1(data: &[(i64, Vec<i64>)]) -> i64 {
    data.iter()
        .filter(|(t, nums)| can_make_target_part1(*t, nums))
        .map(|(t, _)| t)
        .sum()
}

// Part 2: Now we also have the concatenation operator. We will try all combinations of +, *, and concatenation.
fn can_make_target_with_concatenation(target: i64, nums: &[i64]) -> bool {
    if nums.is_empty() {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] == target;
    }

    fn recurse(target: i64, nums: &[i64]) -> bool {
        if nums.len() == 1 {
            return nums[0] == target;
        }

        let last = nums[nums.len() - 1];

        // Case 1: Try addition
        if target >= last && recurse(target - last, &nums[..nums.len() - 1]) {
            return true;
        }

        // Case 2: Try multiplication
        if last != 0 && target % last == 0 && recurse(target / last, &nums[..nums.len() - 1]) {
            return true;
        }

        // Case 3: Try concatenation
        let last_digits = last.to_string();
        let target_digits = target.to_string();
        if target_digits.ends_with(&last_digits) {
            let remaining_target = target_digits[..target_digits.len() - last_digits.len()]
                .parse::<i64>()
                .unwrap_or(0);
            if recurse(remaining_target, &nums[..nums.len() - 1]) {
                return true;
            }
        }

        false
    }

    recurse(target, nums)
}
fn part2(data: &[(i64, Vec<i64>)]) -> i64 {
    data.iter()
        .filter(|(target, nums)| can_make_target_with_concatenation(*target, nums))
        .map(|(target, _)| target)
        .sum()
}

pub fn solve(input: String) {
    let start = Instant::now();
    let (_, data) = parse_input(&input).unwrap();
    let parse_duration = start.elapsed();
    let start_solve = Instant::now();
    let ans_part1 = part1(&data);
    let ans_part2 = part2(&data);
    let solve_duration = start_solve.elapsed();
    println!("Part1: {}", ans_part1);
    println!("Part2: {}", ans_part2);
    println!("Parsing took: {} microseconds", parse_duration.as_micros());
    println!("Solving took: {} microseconds", solve_duration.as_micros());
}

#[cfg(test)]
mod tests {
    mod parse {
        use crate::puzzles::day7::parse_input;

        #[test]
        fn parse_test() {
            let input = "190: 10 19\n3267: 81 40 27\n";
            let (_, data) = parse_input(input).unwrap();
            assert_eq!(data.len(), 2);
            assert_eq!(data[0], (190, vec![10, 19]));
            assert_eq!(data[1], (3267, vec![81, 40, 27]));
        }
    }
    mod part_1 {
        use crate::puzzles::day7::part1;

        #[test]
        fn provided() {
            let data = vec![
                (190, vec![10, 19]),
                (3267, vec![81, 40, 27]),
                (83, vec![17, 5]),
                (156, vec![15, 6]),
                (7290, vec![6, 8, 6, 15]),
                (161011, vec![16, 10, 13]),
                (192, vec![17, 8, 14]),
                (21037, vec![9, 7, 18, 13]),
                (292, vec![11, 6, 16, 20]),
            ];
            let ans = part1(&data);
            assert_eq!(ans, 3749);
        }

        #[test]
        fn simpler() {
            let data = vec![
                (10, vec![5, 2]), // 5*2=10 -> true
                (15, vec![5, 3]), // 5*3=15 -> true
                (11, vec![5, 3]), // no
            ];
            assert_eq!(part1(&data), 25);
        }
    }
    mod part_2 {
        use crate::puzzles::day7::{can_make_target_with_concatenation, part2};
        use crate::puzzles::day7_test_util;

        #[test]
        fn provided() {
            // 6 * 8 = 48, 48 || 6 = 486, 486 * 15 = 7290
            assert!(can_make_target_with_concatenation(7290, &[6, 8, 6, 15]));

            // 15 || 6 = 156
            assert!(can_make_target_with_concatenation(156, &[15, 6]));

            // 17 || 8 + 14 = 192
            assert!(can_make_target_with_concatenation(192, &[17, 8, 14]));

            // Valid case: 12 || 3 = 123
            assert!(can_make_target_with_concatenation(123, &[12, 3]));

            // Concatenation needed for success
            assert!(can_make_target_with_concatenation(
                5693516,
                &[5, 692, 6, 83, 833]
            ));

            // Invalid case
            assert!(!can_make_target_with_concatenation(999, &[10, 20, 30])); // No way to make 999
        }
        #[test]
        fn simpler() {
            // With concatenation:
            // 12: 1 2 can form 1||2=12 -> matches 12
            // 11: 1 1 can form 1||1=11 -> matches 11
            // 25: 5 2 no standard ops produce 25; check concatenation: 5||2=52, 5+2=7, 5*2=10. No match.
            let data = vec![(12, vec![1, 2]), (11, vec![1, 1]), (25, vec![5, 2])];
            // 12 + 11 = 23 no 25
            let ans = part2(&data);
            assert_eq!(ans, 23);
        }
        #[test]
        fn verify_successful_sequences() {
            let vec = day7_test_util::get_test_cases();
            for (line, nums, ops, expected_result) in vec {
                assert!(
                    can_make_target_with_concatenation(expected_result, &nums),
                    "Line {}: nums = {:?}, ops = {:?}, expected = {}",
                    line,
                    nums,
                    ops,
                    expected_result
                );
            }
        }

        #[test]
        fn pipe_plus() {
            let data = vec![(192, vec![17, 8, 14])];
            let ans = part2(&data);
            assert_eq!(ans, 192);
        }
        #[test]
        fn test_concatenation_and_arithmetic_complex() {
            // This is from the puzzle's example for part two:
            // 7290: 6 8 6 15
            //
            // Correct sequence:
            // 6 * 8 = 48
            // 48 || 6 = 486
            // 486 * 15 = 7290
            //
            let data = vec![(7290, vec![6, 8, 6, 15])];
            assert_eq!(part2(&data), 7290);
        }

        #[test]
        fn test_concatenation_only() {
            // Only concatenation makes sense:
            // 123: 1 2 3 -> 1||2||3 = 123
            assert_eq!(part2(&vec![(123, vec![1, 2, 3])]), 123);
        }

        // Test concatenation + addition:
        #[test]
        fn test_concatenation_plus_addition() {
            // 25: 2 5 -> can be 2||5=25 or 2+5=7 or 2*5=10
            // Here concatenation should yield 25.
            let data = vec![(31, vec![2, 5, 6])];
            let ans = part2(&data);
            assert_eq!(ans, 31);
        }

        // Test concatenation + multiplication:
        #[test]
        fn test_concatenation_plus_multiplication() {
            // 102: 10 2 -> can be 10||2=102, 10+2=12, 10*2=20
            let data = vec![(510, vec![10, 2, 5])];
            let ans = part2(&data);
            assert_eq!(ans, 510);
        }

        // Test a sequence where concatenation and addition are needed:
        // 17 || 0 * 1 + 10 = 180
        #[test]
        fn test_concat_with_leading_zero() {
            let data = vec![(180, vec![17, 0, 1, 10])];
            let ans = part2(&data);
            assert_eq!(ans, 180);
        }

        // Test a more complex sequence where multiple concatenations are possible:
        // 1234: 1 23 4 can yield 1||23=123; 123||4=1234
        #[test]
        fn test_multiple_concats() {
            let data = vec![(1234, vec![1, 23, 4])];
            let ans = part2(&data);
            assert_eq!(ans, 1234);
        }
    }
}
