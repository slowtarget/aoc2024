use nom::{
    character::complete::{digit1, space1},
    multi::separated_list1,
    IResult,
};
use std::collections::HashMap;
use std::time::Instant;

// Parse input into a vector of integers
fn parse_input(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, parse_i64)(input)
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    let (input, num_str) = digit1(input)?;
    let val = num_str.parse::<i64>().unwrap();
    Ok((input, val))
}

// Apply transformation rules to a single number and update the counts
fn transform_stone_counts(stone_counts: &HashMap<i64, usize>) -> HashMap<i64, usize> {
    let mut new_counts = HashMap::with_capacity(3811);

    for (&stone, &count) in stone_counts {
        if stone == 0 {
            *new_counts.entry(1).or_insert(0) += count;
        } else {
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let mid = stone_str.len() / 2;
                let left = stone_str[..mid].parse::<i64>().unwrap();
                let right = stone_str[mid..].parse::<i64>().unwrap();
                *new_counts.entry(left).or_insert(0) += count;
                *new_counts.entry(right).or_insert(0) += count;
            } else {
                let new_stone = stone * 2024;
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }
    }

    new_counts
}

// Simulate blinks by iterating over transformations
fn simulate_blinks(stone_counts: &HashMap<i64, usize>, blinks: usize) -> HashMap<i64, usize> {
    let mut new_counts = stone_counts.clone();
    
    for _ in 0..blinks {
        new_counts = transform_stone_counts(&new_counts);
    }

    new_counts
}

fn initialise_counts(stones: &[i64]) -> HashMap<i64, usize> {
    let mut stone_counts: HashMap<i64, usize> = HashMap::new();

    // Initialize counts from the input
    for &stone in stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }
    stone_counts
}

pub fn solve(input: String) {
    let start = Instant::now();
    let (_, stones) = parse_input(&input).unwrap();
    let stone_counts = initialise_counts(&stones);
    println!("Parsing took: {:?}", start.elapsed());

    let start = Instant::now();
    let part_1 = simulate_blinks(&stone_counts, 25);

    println!("Total unique stones after 25 blinks: {}", part_1.len());
    println!("Total number of stones after 25 blinks: {}", part_1.values().sum::<usize>());
    println!("Part 1 took: {:?}", start.elapsed());

    let start = Instant::now();
    let part_2 = simulate_blinks(&part_1, 50);

    println!("Total unique stones after 75 blinks: {}", part_2.len());
    println!("Total number of stones after 75 blinks: {}", part_2.values().sum::<usize>());
    println!("Part 2 took: {:?}", start.elapsed());
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_stone_counts() {
        let mut counts = HashMap::new();
        counts.insert(0, 1);
        counts.insert(10, 1);
        counts.insert(99, 1);
        counts.insert(1, 1);

        let new_counts = transform_stone_counts(&counts);

        let mut expected_counts = HashMap::new();
        expected_counts.insert(1, 2); // 0 becomes 1, 1 remains as 2024
        expected_counts.insert(0, 1); // 10 splits into 1, 0
        expected_counts.insert(9, 2); // 99 splits into two 9s
        expected_counts.insert(2024, 1); // 1 becomes 2024

        assert_eq!(new_counts, expected_counts);
    }

    #[test]
    fn test_simulate_blinks() {
        let stones = vec![125, 17];
        let counts_after_1_blink = simulate_blinks(&initialise_counts(&stones), 1);
        let total_after_1_blink: usize = counts_after_1_blink.values().sum();
        assert_eq!(total_after_1_blink, 3);

        let counts_after_2_blinks = simulate_blinks(&initialise_counts(&stones), 2);
        let total_after_2_blinks: usize = counts_after_2_blinks.values().sum();
        assert_eq!(total_after_2_blinks, 4);
    }
}
