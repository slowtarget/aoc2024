use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

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

// Function to correct an update according to the ordering rules
fn correct_update(ordering_rules: &HashSet<(u32, u32)>, update: &Vec<u32>) -> Vec<u32> {
    // Extract relevant ordering rules
    let pages_in_update: HashSet<u32> = update.iter().cloned().collect();
    let mut relevant_rules = Vec::new();
    for &(x, y) in ordering_rules {
        if pages_in_update.contains(&x) && pages_in_update.contains(&y) {
            relevant_rules.push((x, y));
        }
    }

    // Build the dependency graph
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new(); // Adjacency list
    let mut in_degree: HashMap<u32, usize> = HashMap::new(); // Incoming edge counts

    // Initialize in_degree for all pages in the update
    for &page in &pages_in_update {
        in_degree.insert(page, 0);
    }

    // Build graph and compute in-degrees
    for &(x, y) in &relevant_rules {
        graph.entry(x).or_default().push(y);
        *in_degree.get_mut(&y).unwrap() += 1;
    }

    // Use a max-heap to select nodes with the highest page number
    let mut heap = BinaryHeap::new();

    // Add nodes with zero in-degree to the heap
    for (&page, &deg) in &in_degree {
        if deg == 0 {
            heap.push(Reverse(page)); // Reverse to create a min-heap (since BinaryHeap is a max-heap)
        }
    }

    // Perform topological sort
    let mut sorted_pages = Vec::new();
    while let Some(Reverse(page)) = heap.pop() {
        sorted_pages.push(page);

        if let Some(neighbors) = graph.get(&page) {
            for &neighbor in neighbors {
                let deg = in_degree.get_mut(&neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    heap.push(Reverse(neighbor));
                }
            }
        }
    }

    // Check if we have a valid ordering
    if sorted_pages.len() != pages_in_update.len() {
        panic!("Cycle detected in the ordering rules!");
    }

    sorted_pages
}

pub fn solve(input: String) -> u32 {
    let (ordering_rules, updates) = parse_input(&input);

    let mut total_corrected = 0;
    let mut incorrect_updates = Vec::new();

    for update in &updates {
        if !is_update_correct(&ordering_rules, update) {
            // Incorrectly ordered update, correct it
            let corrected_update = correct_update(&ordering_rules, update);
            let middle_page = middle_page_number(&corrected_update);
            total_corrected += middle_page;
            incorrect_updates.push((corrected_update, middle_page));
        }
    }

    // Output the result
    println!(
        "Total sum of middle page numbers after correcting updates: {}",
        total_corrected
    );
    total_corrected
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn provided() {

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

        assert_eq!(solve(input.to_string()),123);
    }
}
