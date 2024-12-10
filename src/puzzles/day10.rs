use nom::{
    character::complete::newline,
    combinator::{map, map_res},
    multi::separated_list1,
    IResult,
};
use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use timing_util::measure_time;

/// Parse a single line of the map into a vector of integers.
fn parse_line(input: &str) -> IResult<&str, Vec<u8>> {
    map(
        nom::multi::many1(map_res(
            nom::character::complete::one_of("0123456789"),
            |c| c.to_string().parse::<u8>(),
        )),
        |vec| vec,
    )(input)
}

/// Parse the entire input into a 2D grid of integers.
fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(newline, parse_line)(input)
}

/// Get valid neighbors for a given position in the grid.
fn get_neighbors(x: usize, y: usize, grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if x + 1 < rows {
        neighbors.push((x + 1, y));
    }
    if y + 1 < cols {
        neighbors.push((x, y + 1));
    }

    neighbors
}

/// Calculate the score for a trailhead at position (x, y).
fn calculate_score(x: usize, y: usize, grid: &[Vec<u8>]) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut reachable_nines = 0;

    queue.push_back((x, y));
    visited.insert((x, y));

    while let Some((cx, cy)) = queue.pop_front() {
        for (nx, ny) in get_neighbors(cx, cy, grid) {
            if visited.contains(&(nx, ny)) {
                continue;
            }

            // Ensure the trail is valid (height must increase by 1).
            if grid[nx][ny] == grid[cx][cy] + 1 {
                visited.insert((nx, ny));
                queue.push_back((nx, ny));

                if grid[nx][ny] == 9 {
                    reachable_nines += 1;
                }
            }
        }
    }

    reachable_nines
}

/// Sum the scores of all trailheads in the grid.
fn part1(grid: &[Vec<u8>]) -> usize {
    let mut total_score = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, &value) in row.iter().enumerate() {
            if value == 0 {
                total_score += calculate_score(x, y, grid);
            }
        }
    }

    total_score
}

fn calculate_rating(x: usize, y: usize, grid: &[Vec<u8>]) -> usize {
    fn dfs(x: usize, y: usize, grid: &[Vec<u8>], visited: &mut HashSet<(usize, usize)>) -> usize {
        // Base case: End the trail if we've reached height 9
        if grid[x][y] == 9 {
            return 1;
        }

        let mut count = 0;
        let current_height = grid[x][y];
        visited.insert((x, y));

        // Explore all valid neighbors
        for (nx, ny) in get_neighbors(x, y, grid) {
            if !visited.contains(&(nx, ny)) && grid[nx][ny] == current_height + 1 {
                count += dfs(nx, ny, grid, visited);
            }
        }

        visited.remove(&(x, y)); // Backtrack
        count
    }

    let mut visited = HashSet::new();
    dfs(x, y, grid, &mut visited)
}

fn part2(grid: &[Vec<u8>]) -> usize {
    let mut total_rating = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, &value) in row.iter().enumerate() {
            if value == 0 {
                total_rating += calculate_rating(x, y, grid);
            }
        }
    }

    total_rating
}

pub fn solve(input: String) {
    let (_, grid) = measure_time!({ parse_input(&input).unwrap() });

    let total_score = measure_time!({ part1(&grid) });
    println!("Part 1: {}", total_score);

    let total_rating = measure_time!({ part2(&grid) });
    println!("Part 2: {}", total_rating);
}

#[cfg(test)]
mod tests {
    use super::*;
    mod tests_part2 {
        use super::*;

        #[test]
        fn test_example_part2() {
            let input = "\
5590559
5551598
5552557
6543456
7655987
8765555
9875555";
            let (_, grid) = parse_input(input).unwrap();
            assert_eq!(part2(&grid), 13);
        }

        #[test]
        fn test_large_example_part2() {
            let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
            let (_, grid) = parse_input(input).unwrap();
            assert_eq!(part2(&grid), 81);
        }
    }

    mod tests_part1 {
        use super::*;

        #[test]
        fn test_example_input() {
            let input = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
            let (_, grid) = parse_input(input).unwrap();
            assert_eq!(part1(&grid), 36);
        }
    }
}
