use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

fn parse_rule(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (a, b)) = separated_pair(parse_number, tag("|"), parse_number)(input)?;
    Ok((input, (a, b)))
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(line_ending, parse_rule)(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), parse_number)(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(line_ending, parse_update)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let (input, rules) = parse_rules(input)?;
    let (input, _) = line_ending(input)?;
    let (input, updates) = parse_updates(input)?;
    Ok((input, (rules, updates)))
}

fn is_valid_update(rules: &[(u32, u32)], update: &[u32]) -> bool {
    let index_map: HashMap<u32, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &page)| (page, i))
        .collect();

    for &(before, after) in rules {
        if let (Some(&before_index), Some(&after_index)) =
            (index_map.get(&before), index_map.get(&after))
        {
            if before_index > after_index {
                return false;
            }
        }
    }
    true
}

fn middle_page(update: &[u32]) -> u32 {
    let mid_index = update.len() / 2;
    update[mid_index]
}

fn part1(input: &str) -> u32 {
    let (_, (rules, updates)) = parse_input(input).unwrap();

    updates
        .iter()
        .filter(|update| is_valid_update(&rules, update))
        .map(|update| middle_page(update))
        .sum()
}

pub fn solve(input: String) {
    let result = part1(&input);
    println!("Sum of middle pages of valid updates: {}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "\
47|53
97|13

75,47,61,53,29
97,61,53,29,13";
        let (_, (rules, updates)) = parse_input(input).unwrap();
        assert_eq!(rules, vec![(47, 53), (97, 13)]);
        assert_eq!(
            updates,
            vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13],]
        );
    }

    #[test]
    fn test_is_valid_update() {
        let rules = vec![(47, 53), (75, 29)];
        let update = vec![75, 47, 53, 29];
        assert!(is_valid_update(&rules, &update));
        let invalid_update = vec![75, 53, 47, 29];
        assert!(!is_valid_update(&rules, &invalid_update));
    }

    #[test]
    fn test_middle_page() {
        let update = vec![75, 47, 61, 53, 29];
        assert_eq!(middle_page(&update), 61);
    }

    #[test]
    fn test_part1() {
        let input = "47|53\n97|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n";
        assert_eq!(part1(input), 143);
    }
}
