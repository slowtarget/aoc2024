use nom::{
    bytes::complete::take_while1,
    character::complete::newline,
    multi::separated_list1,
    IResult,
};
use std::collections::HashSet;
use std::time::Instant;

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(
        newline,
        take_while1(|c: char| c.is_alphanumeric() || c == '.'),
    )(input)
        .map(|(next_input, rows)| {
            (
                next_input,
                rows.iter().map(|row| row.chars().collect()).collect(),
            )
        })
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn find_antennas(grid: &[Vec<char>]) -> Vec<(Point, char)> {
    let mut antennas = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas.push((Point { x, y }, cell));
            }
        }
    }
    antennas
}
fn calculate_antinodes(antennas: &[(Point, char)], grid_width: usize, grid_height: usize) -> usize {
    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (p1, freq1) = antennas[i];
            let (p2, freq2) = antennas[j];

            if freq1 != freq2 {
                continue;
            }

            // Check if the antennas are collinear
            if p1.x == p2.x || p1.y == p2.y || (p2.x as isize - p1.x as isize).abs()
                == (p2.y as isize - p1.y as isize).abs()
            {
                let dx = p2.x as isize - p1.x as isize;
                let dy = p2.y as isize - p1.y as isize;

                // Compute antinodes on either side
                for multiplier in [-1, 1] {
                    let ax = p1.x as isize - dx * multiplier;
                    let ay = p1.y as isize - dy * multiplier;

                    if ax >= 0 && ay >= 0 && (ax as usize) < grid_width && (ay as usize) < grid_height
                    {
                        antinodes.insert(Point {
                            x: ax as usize,
                            y: ay as usize,
                        });
                    }
                }
            }
        }
    }

    antinodes.len()
}
pub fn part1(input: &str) -> usize {
    let (_, grid) = parse(input).unwrap();
    let antennas = find_antennas(&grid);
    let grid_width = grid[0].len();
    let grid_height = grid.len();

    calculate_antinodes(&antennas, grid_width, grid_height)
}
pub fn solve(input: String) {
    let start = Instant::now();

    let parse_duration = start.elapsed();
    let start_solve = Instant::now();
    let ans_part1 = part1(&*input);
    let ans_part2 = 0;
    let solve_duration = start_solve.elapsed();
    println!("Part1: {}", ans_part1);
    println!("Part2: {}", ans_part2);
    println!("Parsing took: {} microseconds", parse_duration.as_micros());
    println!("Solving took: {} microseconds", solve_duration.as_micros());
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input ="..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";
        assert_eq!(part1(input), 2);
    }
    #[test]
    fn test_part1() {
        let input = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part1(input), 14);
    }
}
