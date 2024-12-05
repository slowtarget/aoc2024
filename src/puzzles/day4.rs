use nom::{
    IResult,
    character::complete::anychar,
    multi::many1,
};

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    many1(anychar)(input)
}

pub(crate) fn solve(input: String) -> (u32, u32) {
    // First, build the grid
    let mut grid: Vec<Vec<Point>> = Vec::new();

    // Parse the input into a 2D grid of Points
    for (y, line) in input.lines().enumerate() {
        let (_, chars) = parse_line(line).expect("Failed to parse line");
        let mut row = Vec::new();
        for (x, &ch) in chars.iter().enumerate() {
            row.push(Point {
                value: ch,
                x,
                y,
                neighbours: Vec::new(),
            });
        }
        grid.push(row);
    }

    // Set up neighbours for each point in fixed order
    let height = grid.len();
    let width = grid[0].len();
    let directions = [
        (-1, -1), // 0: Northwest
        (0, -1),  // 1: North
        (1, -1),  // 2: Northeast
        (-1, 0),  // 3: West
        (1, 0),   // 4: East
        (-1, 1),  // 5: Southwest
        (0, 1),   // 6: South
        (1, 1),   // 7: Southeast
    ];

    for y in 0..height {
        for x in 0..width {
            let mut neighbours = Vec::new();
            for &(dx, dy) in &directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                    neighbours.push(Some((nx as usize, ny as usize)));
                } else {
                    neighbours.push(None);
                }
            }
            grid[y][x].neighbours = neighbours;
        }
    }

    // Part One
    let part_one_total = solve_part_one(&grid);
    println!("Part One - Total occurrences of 'XMAS': {}", part_one_total);

    // Part Two
    let part_two_total = solve_part_two(&grid); // 1961 was too high - but tests pass
    println!("Part Two - Total occurrences of 'X-MAS': {}", part_two_total);
    (part_one_total, part_two_total)
}

struct Point {
    value: char,
    x: usize,
    y: usize,
    neighbours: Vec<Option<(usize, usize)>>, // Fixed order of neighbors
}

// Function to solve Part One
fn solve_part_one(grid: &Vec<Vec<Point>>) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            total += search_all_directions(grid, x, y);
        }
    }
    total
}

// Function to search for "XMAS" starting from a point in all directions
fn search_all_directions(grid: &Vec<Vec<Point>>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for dir in 0..8 {
        count += search_in_direction(grid, x, y, dir, 0);
    }
    count
}

// Recursive function to search for "XMAS" in a specific direction
fn search_in_direction(
    grid: &Vec<Vec<Point>>,
    x: usize,
    y: usize,
    dir: usize,
    index: usize,
) -> u32 {
    let word = ['X', 'M', 'A', 'S'];
    if grid[y][x].value != word[index] {
        return 0;
    }
    if index == word.len() - 1 {
        return 1;
    }
    if let Some(Some((nx, ny))) = grid[y][x].neighbours.get(dir) {
        // Proceed to the neighbor in the same direction
        search_in_direction(grid, *nx, *ny, dir, index + 1)
    } else {
        0
    }
}

// Function to solve Part Two
fn solve_part_two(grid: &Vec<Vec<Point>>) -> u32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut total = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x].value == 'A' {
                total += check_x_mas(&grid, x, y);
            }
        }
    }

    total
}

// Function to check for 'X-MAS' centered at a given 'A'
fn check_x_mas(grid: &Vec<Vec<Point>>, x: usize, y: usize) -> u32 {
    let mut count = 0;

    // Diagonal orientation: pairs are (NW, SE) and (NE, SW)
    if check_orientation(grid, x, y, &[(0, 7), (2, 5)]) {
        count += 1;
    }

    // Cardinal orientation: pairs are (N, S) and (W, E)
    // if check_orientation(grid, x, y, &[(1, 6), (3, 4)]) {
    //     count += 1;
    // }

    count
}

// Function to check if both pairs in an orientation satisfy the condition
fn check_orientation(
    grid: &Vec<Vec<Point>>,
    x: usize,
    y: usize,
    pairs: &[(usize, usize)],
) -> bool {
    for &(dir1, dir2) in pairs {
        if !check_pair(grid, x, y, dir1, dir2) {
            return false;
        }
    }
    true
}

// Function to check if the characters in the two directions are 'M' and 'S' in any order
fn check_pair(
    grid: &Vec<Vec<Point>>,
    x: usize,
    y: usize,
    dir1: usize,
    dir2: usize,
) -> bool {
    let pair = [dir1, dir2].map(|dir| {
        grid[y][x].neighbours.get(dir)
            .and_then(|&pos| pos.map(|(nx, ny)| grid[ny][nx].value)).unwrap_or_default()
    }).iter().collect::<String>();
    pair == "MS" || pair == "SM"

}

#[cfg(test)]
mod tests_xmas {
    use super::*;
    #[test]
    fn test_provided() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let (part1, part2) = solve(input.to_string());
        assert_eq!(part1, 18);
        assert_eq!(part2, 9);
    }
    mod part_one {
        use super::*;

        #[test]
        fn test_xmas_e() {
            let input = "XMAS";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 1);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_xmas_w() {
            let input = "SAMX";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 1);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_xmas_s() {
            let input = "X
M
A
S";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 1);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_xmas_n() {
            let input = "S
A
M
X";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 1);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_xmas_ne() {
            let input = "S...
.A..
..M.
...X";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 1);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_xmas_se() {
            let input = "...X
..M.
.A..
S...";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 1);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_xmas_sw() {
            let input = "X...
.M..
..A.
...S";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 1);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_xmas_nw() {
            let input = "...S
..A.
.M..
X...";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 1);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_xmas_none() {
            let input = "X...
..M.
..A..
...S";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 0);
            assert_eq!(part2, 0);
        }
    }

    mod part_two {
        use super::*;

        #[test]
        fn test_x_mas_cardinal() {
            let input = ".M.
MAS
.S.";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 0);
            assert_eq!(part2, 0);
        }
        #[test]
        fn test_x_mas_diagonal() {
            let input = "M.M
.A.
S.S";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 0);
            assert_eq!(part2, 1);
        }
        #[test]
        fn test_x_mas_both() {
            let input = "MMM
MAS
SSS";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 0);
            assert_eq!(part2, 1);
        }
        #[test]
        fn test_x_mas_none() {
            let input = "MMM
AAA
MMM";
            let (part1, part2) = solve(input.to_string());
            assert_eq!(part1, 0);
            assert_eq!(part2, 0);
        }
    }

}