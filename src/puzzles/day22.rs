use crate::puzzles::day14;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::{HashMap};
use timing_util::measure_time;
use std::time::Instant;
const MASK: i32 = 16777216 - 1;

fn parse_line(input: &str) -> IResult<&str, usize> {
    let (i, result) = day14::parse_unsigned(input)?;
    Ok((i, result))
}
fn parse(input: String) -> Vec<usize> {
    separated_list1(tag("\n"), parse_line)(input.trim())
        .unwrap()
        .1
}
fn prune(input: i32) -> i32 {
    input & MASK
}

fn mix(secret: i32, number: i32) -> i32 {
    number ^ secret
}
fn next(current: i32) -> i32 {
    let mut result = prune(mix(current, current << 6));
    // println!(
    //     "current {:b} current * 64 {:b} xor: {:b} mixed and pruned {}",
    //     current,
    //     current * 64,
    //     current ^ (current * 64),
    //     result
    // );
    result = prune(mix(result, result >> 5));
    prune(mix(result, result << 11))
}
fn two_thousandth(input: i32) -> i32 {
    let mut result = input;

    for _ in 0..2000 {
        result = next(result);
    }
    result
}
fn get_sequences(input: i32) -> HashMap<[i32; 4], usize> {
    let mut secret = input;
    let mut changes = Vec::with_capacity(4);
    let mut previous = secret % 10;
    let mut sequences: HashMap<[i32; 4], usize> = HashMap::with_capacity(2000);
    for _ in 0..2000 {
        secret = next(secret);
        let digit = secret % 10;
        changes.push(digit - previous);
        previous = digit;

        if changes.len() == 4 {
            let sequence: [i32; 4] = changes[..].try_into().unwrap();
            changes.remove(0);

            if sequences.contains_key(&sequence) {
                continue;
            }
            sequences.insert(sequence, digit as usize);
        }
    }
    sequences
}
pub(crate) fn solve(input: String) -> (i32, i32) {
    let numbers = parse(input);
    let part_1 = numbers
        .iter()
        .fold(0, |sum, number| sum + two_thousandth(*number as i32));

    (part_1, measure_time!( part_2(numbers))) // 225 is too low
}

fn part_2(numbers: Vec<usize>) -> i32 {
    let mut sequence_totals: HashMap<[i32; 4], usize> = HashMap::with_capacity(10000);
    for number in numbers {
        let sequences = get_sequences(number as i32);
        for (sequence, value) in sequences {
            let sum = sequence_totals.get(&sequence).unwrap_or(&0) + value;
            sequence_totals.insert(sequence, sum);
        }
    }
    *sequence_totals.values().max().unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_tests {
        use super::*;

        #[test]
        fn parse_input_test() {
            assert_eq!(parse(get_input()), vec![1, 10, 100, 2024]);
        }

        #[test]
        fn prune_test() {
            println!("{:b}", 100000000);
            println!("{:b}", 16777216);
            println!("{:b}", 16113920);

            println!("{:b}", MASK);

            assert_eq!(prune(100000000), 16113920);
            assert_eq!(100000000 & MASK, 16113920);
        }

        #[test]
        fn mix_test() {
            assert_eq!(mix(42, 15), 37);
        }

        #[test]
        fn rounding_test() {
            assert_eq!(33 / 32, 1);
            assert_eq!(48 / 32, 1);
            assert_eq!(63 / 32, 1);
            assert_eq!(65 / 32, 2);
            assert_eq!(33 >> 5, 1);
            assert_eq!(48 >> 5, 1);
            assert_eq!(63 >> 5, 1);
            assert_eq!(65 >> 5, 2);
        }

        #[test]
        fn next_test() {
            let numbers = [
                123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
                7753432, 5908254,
            ];
            for (i, number) in numbers.iter().enumerate() {
                if i == numbers.len() - 1 {
                    break;
                }
                println!("{} {}", number, numbers[i + 1]);
                assert_eq!(next(*number), numbers[i + 1]);
            }
        }

        #[test]
        fn part_2_test() {
            assert_eq!(part_2([1, 2, 3, 2024].to_vec()),23);
        }
        fn get_input() -> String {
            "\
1
10
100
2024"
                .to_string()
        }
    }
}
