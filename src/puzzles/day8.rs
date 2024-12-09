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
        take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == '#'),
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
    x: isize,
    y: isize,
}

fn find_antennas(grid: &[Vec<char>]) -> Vec<(Point, char)> {
    let mut antennas = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != '.'  && cell != '#' {
                antennas.push((Point { x: x as isize, y: y as isize }, cell));
            }
        }
    }
    antennas
}
fn calculate_antinodes(antennas: &[(Point, char)], grid_width: isize, grid_height: isize) -> usize {
    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (p1, freq1) = antennas[i];
            let (p2, freq2) = antennas[j];

            if freq1 != freq2 {
                continue;
            }

            let mut candidates = HashSet::new();
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            candidates.insert(Point {
                x: p1.x - dx,
                y: p1.y - dy,
            });
            candidates.insert(Point {
                x: p2.x + dx,
                y: p2.y + dy,
            });
            // println!("candidates: {:?}", candidates);
            candidates.iter().filter(|&Point { x, y }|{x >= &0isize && y >= &0isize && x < &grid_width && y < &grid_height}).for_each(|&Point { x, y }| {
                antinodes.insert(Point {
                    x,
                    y,
                });
            });

        }
    }
    // println!("antennas: {:?}", antennas);
    // println!("antinodes: {:?}", antinodes);
    antinodes.len()
}
fn calculate_antinodes2(antennas: &[(Point, char)], grid_width: isize, grid_height: isize) -> usize {
    let mut antinodes = HashSet::new();

    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            let (p1, freq1) = antennas[i];
            let (p2, freq2) = antennas[j];

            if freq1 != freq2 {
                continue;
            }

            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let mut mult = 0;
            while p1.x - dx * mult >= 0 && p1.y - dy * mult >= 0 && p1.x - dx * mult < grid_width && p1.y - dy * mult < grid_height {
                let x = p1.x - dx * mult;
                let y = p1.y - dy * mult;
                antinodes.insert(Point {
                    x,
                    y,
                });
                mult += 1;
            }
            mult = 0;
            while p2.x + dx * mult >= 0 && p2.y + dy * mult >= 0 && p2.x + dx * mult < grid_width && p2.y + dy * mult < grid_height {
                antinodes.insert(Point {
                    x: p2.x + dx * mult,
                    y: p2.y + dy * mult,
                });
                mult += 1
            }
        }
    }
    // println!("antennas: {:?}", antennas);
    // println!("antinodes: {:?}", antinodes);
    antinodes.len()
}
pub fn part1(input: &str) -> usize {
    let (_, grid) = parse(input).unwrap();
    let antennas = find_antennas(&grid);
    let grid_width = grid[0].len() as isize;
    let grid_height = grid.len() as isize;

    let result = calculate_antinodes(&antennas, grid_width, grid_height);
    result
}
pub fn part2(input: &str) -> usize {
    let (_, grid) = parse(input).unwrap();
    let antennas = find_antennas(&grid);
    let grid_width = grid[0].len() as isize;
    let grid_height = grid.len() as isize;

    let result = calculate_antinodes2(&antennas, grid_width, grid_height);
    result
}
pub fn solve(input: String) {
    let start = Instant::now();

    let parse_duration = start.elapsed();
    let start_solve = Instant::now();
    let ans_part1 = part1(&*input);
    let ans_part2 = part2(&*input); 
    let solve_duration = start_solve.elapsed();
    println!("Part1: {}", ans_part1);
    println!("Part2: {}", ans_part2);
    println!("Parsing took: {} microseconds", parse_duration.as_micros());
    println!("Solving took: {} microseconds", solve_duration.as_micros());
}
#[cfg(test)]
mod tests {
    use super::*;
    mod part1 {
        use super::*;

        #[test]
        fn simple() {
            let input = "..........
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
        fn simpler() {
            let input = "..a.a.....";
            assert_eq!(part1(input), 2);
        }
        #[test]
        fn border() {
            let input = "a.a.....";
            assert_eq!(part1(input), 1);
        }
        #[test]
        fn collision() {
            let input = "a.a...b.b";
            assert_eq!(part1(input), 1);
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
    mod part2 {
        use crate::puzzles::day8::{part2};

        #[test]
        fn provided() {
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
            assert_eq!(part2(input), 34);
        }#[test]
        fn simple() {
            let input = "\
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........";
            assert_eq!(part2(input), 9);
        }
    }
}
