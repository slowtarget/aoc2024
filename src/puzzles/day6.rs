use std::time::Instant;

pub(crate) fn solve(input: String) {
    let start = Instant::now();

    let (width,height, obstruction, guard_x, guard_y, direction) = parse_map(&input);
    let ( part1_result, visited) = part1(&width, &height, &obstruction, &guard_x, &guard_y, &direction);
    let part2_result = part2(width, height, obstruction, visited, guard_x, guard_y, direction);
    let duration = start.elapsed();
    println!("Execution time: {} microseconds", duration.as_micros());

    println!("part1: {}, part2: {}", part1_result, part2_result);
}

fn parse_map(input: &String) -> (usize, usize, Vec<bool>, usize, usize, usize) {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let mut obstruction: Vec<bool> = vec![false; lines.len() * lines[0].len()];
    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;
    let mut direction: usize = 0; // 0=North, 1=East, 2=South, 3=West
    let guard_direction = ['^','>','v','<'];
    let width = lines[0].len();
    let height = lines.len();
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {}
                '#' => {
                    obstruction[y * width + x] = true;
                }
                _ => {
                    let d = guard_direction.iter().enumerate().find(|(_, x)| **x == ch).map(|(dir, _)| dir);
                    if d.is_some() {
                        direction = d.unwrap();
                        guard_x = x;
                        guard_y = y;
                    }
                }
            }
        }

    }
    (width, height, obstruction, guard_x, guard_y, direction)
}

fn part1(width: &usize, height: &usize, obstruction: &Vec<bool>, guard_x: &usize, guard_y: &usize, direction: &usize) -> (usize, Vec<bool>) {
    let mut visited = vec![false; width * height];
    let mut guard_x = *guard_x;
    let mut guard_y = *guard_y;
    let mut direction = *direction;

    // Movement deltas for (N, E, S, W)
    let deltas = [(0isize, -1isize), (1, 0), (0, 1), (-1, 0)];

    // Mark starting position visited
    visited[guard_y * width + guard_x] = true;

    // Simulation loop
    loop {
        let (dx, dy) = deltas[direction as usize];
        let nx = guard_x as isize + dx;
        let ny = guard_y as isize + dy;
        if nx < 0 || ny < 0 || nx >= *width as isize || ny >= *height as isize {
            // Guard leaves the map
            break;
        }
        let nxu = nx as usize;
        let nyu = ny as usize;

        if obstruction[nyu * *width + nxu] {
            // Turn right
            direction = (direction + 1) % 4;
        } else {
            // Move forward
            guard_x = nxu;
            guard_y = nyu;
            visited[guard_y * width + guard_x] = true;
        }
    }

    let count = visited.iter().filter(|&&x| x).count();
    (count, visited)
}

fn part2(width: usize, height: usize, mut obstruction: Vec<bool>,visited: Vec<bool>, guard_x: usize, guard_y: usize, guard_dir:usize) -> i32 {

    // We want to find how many positions cause a loop if we place an obstruction there.
    // The new obstruction can't be placed at the guard's starting position.

    //The new obstruction will only make an impact if placed somewhere on the original route

    let mut count = 0;

    // We'll consider placing '#' on any cell that is '.' and not the start cell and has been visited
    // After placing it, we run causes_loop and see if we get a loop.
    for y in 0..height {
        for x in 0..width {
            // Can't place at start position
            if x == guard_x && y == guard_y {
                continue;
            }
            if !visited[y * width + x] {
                continue;
            }
            if !obstruction[y * width + x] {
                // Place an obstruction
                obstruction[y * width + x] = true;

                if causes_loop(&width,&height, &obstruction, guard_x, guard_y, guard_dir) {
                    count += 1;
                }

                // Remove obstruction
                obstruction[y * width + x] = false;
            }
        }
    }

    count
}

fn causes_loop(width: &usize, height: &usize, obstruction: &[bool], start_x: usize, start_y: usize, start_dir: usize) -> bool {

    // We'll track visited states as visited_states[(y * width + x) * 4 + dir]
    // If we ever revisit the same state, we have a loop.
    let mut visited_states = vec![false; 4 * width * height];

    let mut x = start_x;
    let mut y = start_y;
    let mut dir = start_dir;

    // Movement deltas for (N, E, S, W)
    let deltas = [(0isize, -1isize), (1,0), (0,1), (-1,0)];

    // Mark the initial state as visited
    visited_states[(y * width + x) * 4 + dir] = true;

    loop {
        let (dx, dy) = deltas[dir];
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        // Check bounds
        if nx < 0 || ny < 0 || nx >= *width as isize || ny >= *height as isize {
            // Guard leaves the map, no loop
            return false;
        }
        let nxu = nx as usize;
        let nyu = ny as usize;

        if obstruction[nyu * *width + nxu] {
            // Turn right
            let new_dir = (dir + 1) % 4;
            // We haven't moved, just turned. Check if this state was visited:
            if visited_states[(y * width + x) * 4 + new_dir] {
                // Loop detected
                return true;
            }
            visited_states[(y * width + x) * 4 + new_dir] = true;
            dir = new_dir;
        } else {
            // Move forward
            let new_x = nxu;
            let new_y = nyu;
            let new_dir = dir;

            if visited_states[(new_y * width + new_x) * 4 + new_dir] {
                // Loop detected
                return true;
            }

            visited_states[(new_y * width + new_x) * 4 + new_dir] = true;

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
        let (width, height, obstruction, gx, gy, d) = parse_map(&input.to_string());
        let (part1,_) = part1(&width, &height, &obstruction, &gx, &gy, &d);
        assert_eq!(part1, 41);
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
        let (width, height, obstruction, gx, gy, d) = parse_map(&input.to_string());
        let (_part1, visited) = part1(&width, &height, &obstruction, &gx, &gy, &d);
        assert_eq!(part2(width,height,obstruction, visited ,gx, gy, d), 6);
    }
}
