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

use std::io::{self, BufRead};

#[derive(Clone)]
struct Point {
    value: char,
    visited: bool,
}

pub(crate) fn solve(input: String) {
    let (grid, guard_x, guard_y, direction) = parse_map(&input);
    let part1_result = part1_simulation(&mut grid.clone(), guard_x, guard_y, direction);
    let part2_result = part2(&input);

    println!("part1: {}, part2: {}", part1_result, part2_result);
}

fn parse_map(input: &String) -> (Vec<Vec<Point>>, usize, usize, usize) {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    // Parse the map into a 2D grid of Points
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid: Vec<Vec<Point>> = Vec::new();
    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;
    let mut direction: usize = 0; // 0=North, 1=East, 2=South, 3=West

    for (y, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            let mut p = Point { value: ch, visited: false };
            // Identify guard
            if ch == '^' {
                guard_x = x;
                guard_y = y;
                direction = 0; // Facing North
                p.value = '.'; // Replace guard symbol with floor
            } else if ch == '>' {
                guard_x = x;
                guard_y = y;
                direction = 1; // Facing East
                p.value = '.';
            } else if ch == 'v' {
                guard_x = x;
                guard_y = y;
                direction = 2; // Facing South
                p.value = '.';
            } else if ch == '<' {
                guard_x = x;
                guard_y = y;
                direction = 3; // Facing West
                p.value = '.';
            }
            row.push(p);
        }

        grid.push(row);
    }

    (grid, guard_x, guard_y, direction)
}

fn part1_simulation(grid: &mut Vec<Vec<Point>>, guard_x: usize, guard_y: usize, direction: usize) -> i32 {
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else {0};
    let mut guard_x = guard_x;
    let mut guard_y = guard_y;
    let mut direction = direction;

    // Movement deltas for (N, E, S, W)
    let deltas = [(0isize, -1isize), (1, 0), (0, 1), (-1, 0)];

    // Mark starting position visited
    grid[guard_y][guard_x].visited = true;

    // Simulation loop
    loop {
        let (dx, dy) = deltas[direction];
        let nx = guard_x as isize + dx;
        let ny = guard_y as isize + dy;
        if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
            // Guard leaves the map
            break;
        }
        let nxu = nx as usize;
        let nyu = ny as usize;
        let blocked = grid[nyu][nxu].value == '#';

        if blocked {
            // Turn right
            direction = (direction + 1) % 4;
        } else {
            // Move forward
            guard_x = nxu;
            guard_y = nyu;
            grid[guard_y][guard_x].visited = true;
        }
    }

    // Count visited positions
    let mut count = 0;
    for row in &*grid {
        for p in row {
            if p.visited {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &String) -> i32 {
    let (mut original_grid, guard_x, guard_y, guard_dir) = parse_map(input);

    // We want to find how many positions cause a loop if we place an obstruction there.
    // The new obstruction can't be placed at the guard's starting position.
    let height = original_grid.len();
    let width = if height == 0 {0} else {original_grid[0].len()};

    let mut count = 0;

    // We'll consider placing '#' on any cell that is '.' and not the start cell.
    // After placing it, we run causes_loop and see if we get a loop.
    for y in 0..height {
        for x in 0..width {
            // Can't place at start position
            if x == guard_x && y == guard_y {
                continue;
            }
            if original_grid[y][x].value == '.' {
                // Place an obstruction
                original_grid[y][x].value = '#';

                if causes_loop(&original_grid, guard_x, guard_y, guard_dir) {
                    count += 1;
                }

                // Remove obstruction
                original_grid[y][x].value = '.';
            }
        }
    }

    count
}

fn causes_loop(grid: &Vec<Vec<Point>>, start_x: usize, start_y: usize, start_dir: usize) -> bool {
    let height = grid.len();
    if height == 0 { return false; }
    let width = grid[0].len();

    // We'll track visited states as visited_states[y][x][dir]
    // If we ever revisit the same state, we have a loop.
    let mut visited_states = vec![vec![vec![false;4];width];height];

    let mut x = start_x;
    let mut y = start_y;
    let mut dir = start_dir;

    // Movement deltas for (N, E, S, W)
    let deltas = [(0isize, -1isize), (1,0), (0,1), (-1,0)];

    // Mark the initial state as visited
    visited_states[y][x][dir] = true;

    loop {
        let (dx, dy) = deltas[dir];
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        // Check bounds
        if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
            // Guard leaves the map, no loop
            return false;
        }
        let nxu = nx as usize;
        let nyu = ny as usize;

        let blocked = grid[nyu][nxu].value == '#';
        if blocked {
            // Turn right
            let new_dir = (dir + 1) % 4;
            // We haven't moved, just turned. Check if this state was visited:
            if visited_states[y][x][new_dir] {
                // Loop detected
                return true;
            }
            visited_states[y][x][new_dir] = true;
            dir = new_dir;
        } else {
            // Move forward
            let new_x = nxu;
            let new_y = nyu;
            let new_dir = dir;

            if visited_states[new_y][new_x][new_dir] {
                // Loop detected
                return true;
            }

            visited_states[new_y][new_x][new_dir] = true;

            x = new_x;
            y = new_y;
            dir = new_dir;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let (grid, gx, gy, d) = parse_map(&input.to_string());
        let mut grid_clone = grid.clone();
        assert_eq!(part1_simulation(&mut grid_clone, gx, gy, d), 41);
    }

    #[test]
    fn part2_test() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let (grid, gx, gy, d) = parse_map(&input.to_string());
        let mut grid_clone = grid.clone();
        assert_eq!(part2(&input.to_string()), 6);
    }
}
