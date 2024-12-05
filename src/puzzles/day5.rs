use std::collections::{HashMap, HashSet};
use std::fs;

// Function to parse the ordering rules and updates from the input string
fn parse_input(input: &str) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let mut lines = input.lines();

    // Read ordering rules until an empty line is encountered
    let mut ordering_rules = HashSet::new();
    for line in &mut lines {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            let x = parts[0].parse::<u32>().unwrap();
            let y = parts[1].parse::<u32>().unwrap();
            ordering_rules.insert((x, y));
        }
    }

    // Read updates
    let mut updates = Vec::new();
    for line in &mut lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let pages: Vec<u32> = line
            .split(',')
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();
        updates.push(pages);
    }

    (ordering_rules, updates)
}

// Function to check if an update is in correct order
fn is_update_correct(ordering_rules: &HashSet<(u32, u32)>, update: &Vec<u32>) -> bool {
    // Create a map from page number to its position in the update
    let mut position_map = HashMap::new();
    for (index, &page) in update.iter().enumerate() {
        position_map.insert(page, index);
    }

    // For each ordering rule, check if it is violated in the update
    for &(x, y) in ordering_rules {
        if let (Some(&pos_x), Some(&pos_y)) = (position_map.get(&x), position_map.get(&y)) {
            if pos_x >= pos_y {
                // Rule violated
                return false;
            }
        }
    }

    // All rules satisfied
    true
}

// Function to find the middle page number of an update
fn middle_page_number(update: &Vec<u32>) -> u32 {
    let len = update.len();
    update[len / 2]
}

pub (crate) fn solve(input: String) {
    let (ordering_rules, updates) = parse_input(&input);

    let mut total = 0;
    let mut correct_updates = Vec::new();

    for update in updates {
        if is_update_correct(&ordering_rules, &update) {
            let middle_page = middle_page_number(&update);
            total += middle_page;
            correct_updates.push((update, middle_page));
        }
    }

    // Output the result
    println!("Total sum of middle page numbers: {}", total);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_provided() {
        let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        solve(input.to_string());
    }

}