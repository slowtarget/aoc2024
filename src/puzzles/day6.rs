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


pub(crate) fn solve(input:String) {

    let part1: i32 = part1(&input);
    let part2: i32 = 0;

    println!("part1: {}, part2: {}", part1, part2);
}
use std::io::{self, BufRead};

#[derive(Clone)]
struct Point {
    value: char,
    visited: bool,
}

fn part1(input: &String) -> i32 {
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
        // If line shorter than width, fill remaining with '.'
        while row.len() < width {
            row.push(Point { value: '.', visited: false });
        }
        grid.push(row);
    }

    // Movement deltas for (N, E, S, W)
    let deltas = [ (0isize, -1isize), (1, 0), (0, 1), (-1, 0) ];

    // Mark starting position visited
    grid[guard_y as usize][guard_x as usize].visited = true;

    // Simulation loop
    loop {
        // Compute the coordinates in front of the guard
        let (dx, dy) = deltas[direction];
        let nx = guard_x as isize + dx;
        let ny = guard_y as isize + dy;
        if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
            // Guard left the map
            break;
        }
        // Check if forward is blocked or out of bounds
        let blocked = {
            let nxu = nx as usize;
            let nyu = ny as usize;
            grid[nyu][nxu].value == '#'
        };

        if blocked {
            // Turn right
            direction = (direction + 1) % 4;
        } else {
            // Move forward
            guard_x = nx as usize;
            guard_y = ny as usize;

            // Mark visited
            grid[guard_y][guard_x].visited = true;
        }
    }

    // Count visited positions
    let mut count = 0;
    for row in &grid {
        for p in row {
            if p.visited {
                count += 1;
            }
        }
    }

    count
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
        assert_eq!(part1(&input.to_string()), 41);

    }
}