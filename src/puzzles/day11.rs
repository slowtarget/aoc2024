use nom::{
    character::complete::{digit1, space1},
    multi::separated_list1,
    IResult,
};
use std::time::Instant;

// Parse a list of integers from the input
fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, parse_i64)(input)
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (input, num_str) = digit1(input)?;
    let val = num_str.parse::<i64>().unwrap();
    Ok((input, val))
}

// Function to apply transformation rules to a stone
fn transform_stone(stone: i64) -> Vec<i64> {
    if stone == 0 {
        vec![1]
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let mid = stone_str.len() / 2;
            let left = stone_str[..mid].parse::<i64>().unwrap();
            let right = stone_str[mid..].parse::<i64>().unwrap();
            vec![left, right]
        } else {
            vec![stone * 2024]
        }
    }
}

// Simulate a number of blinks
fn simulate_blinks(stones: &[i64], blinks: usize) -> Vec<i64> {
    let mut current_stones = stones.to_vec();
    for _ in 0..blinks {
        current_stones = current_stones
            .iter()
            .flat_map(|&stone| transform_stone(stone))
            .collect();
    }
    current_stones
}

pub fn solve(input: String) {
    let (_, stones) = parse_input(&input).unwrap();

    let start = Instant::now();
    let result_stones = simulate_blinks(&stones, 25);
    let duration = start.elapsed();

    println!("Number of stones after 25 blinks: {}", result_stones.len());
    println!("Simulation took: {} microseconds", duration.as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_stone() {
        assert_eq!(transform_stone(0), vec![1]);
        assert_eq!(transform_stone(10), vec![1, 0]);
        assert_eq!(transform_stone(99), vec![9, 9]);
        assert_eq!(transform_stone(1), vec![2024]);
    }

    #[test]
    fn test_simulate_blinks() {
        let stones = vec![125, 17];
        let after_1_blink = simulate_blinks(&stones, 1);
        assert_eq!(after_1_blink, vec![253000, 1, 7]);

        let after_2_blinks = simulate_blinks(&stones, 2);
        assert_eq!(after_2_blinks, vec![253, 0, 2024, 14168]);
    }
}
