// --- Day 18: Reindeer Maze ---

use core::fmt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map_res, opt, recognize};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::str::FromStr;

#[derive(Eq, Hash,Debug, Clone, PartialEq, Default)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
struct Path {
    distance: Option<i32>,
    location: Point,
    start: bool,
    end: bool,
    neighbours: [Option<Point>; 4],
}
impl Path {
    fn new(location: Point) -> Self {
        Self {
            distance: None,
            location,
            start: false,
            end: false,
            neighbours: [None, None, None, None],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
struct Grid (Vec<Vec<Option<Path>>>, Point, Point, Point);
impl Grid {
    fn new(input: &[Point], width: usize, height: usize) -> Self {
        println!("length of input {:?}", input.len());
        println!("last {:?}", input.last().unwrap());
        let start = Point::new(0, 0);
        let end = Point::new(width - 1, height - 1);
        let mut grid: Vec<Vec<Option<Path>>> = vec! {vec! {Default::default(); width}; height};
        for y in 0..height {
            for x in 0..width {
                let point = Point::new(x, y);
                let is_start = point == start;
                let is_end = point == end;
                let mut cell = Path::new(point);
                cell.distance = if is_start { Some(0) } else { Some(i32::MAX)};
                cell.start = is_start;
                cell.end = is_end;
                grid[y][x] = Some(cell);
            }
        }
        input.iter().for_each(|point| {
            grid[point.y][point.x] = None;
        });
        
        Self::populate_neighbours(&mut grid);
        Self(grid, start, end, input.last().unwrap().clone())
    }
    fn populate_neighbours(maze: &mut Vec<Vec<Option<Path>>>) {
        let neighbours: Vec<(isize, isize)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)]; // N,E,S,W
        let maze_copy = maze.clone();
        let width = maze_copy[0].len() as isize;
        let height = maze_copy.len() as isize;
        for (y, row) in maze.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if let Some(cell) = cell {
                    let neighbour_cells: Vec<Option<Point>> = neighbours
                        .iter()
                        .map(|(dx, dy)| {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if nx >= 0 && ny >= 0 && nx < width && ny < height && maze_copy[ny as usize][nx as usize].is_some() {
                                Some(Point::new(nx as usize, ny as usize))
                            } else {
                                None
                            }
                        })
                        .collect();// Trait `FromIterator<()>` is not implemented for `Vec<Option<Point>>` 
                    cell.neighbours = neighbour_cells.try_into().unwrap();
                }
            }
        }
    }
    fn find_shortest_path(&mut self) -> i32 {
        // let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: Vec<Point> = Vec::new();
        let end = { self.2.clone() };
        let grid = &mut self.0;
        queue.push(self.1.clone());
        while let Some(current) = queue.pop() {

            // if visited.contains(&current) {
            //     continue;
            // }
            // visited.insert(current.clone());
            
            let ( neighbours, current_distance) = {
                let current_path = &grid[current.y][current.x].as_ref().unwrap();
                (current_path.neighbours.clone(),
                 current_path.distance)
            };
            for neighbour in neighbours.into_iter().flatten() {
                let neighbour_path = &mut grid[neighbour.y][neighbour.x].as_mut().unwrap();
                let distance = current_distance.unwrap_or(0) + 1;
                if  neighbour_path.distance.unwrap_or(i32::MAX) > distance {
                    neighbour_path.distance = Some(distance);
                    queue.push(neighbour.clone());
                }
            }
        }
        grid[end.y][end.x].as_ref().unwrap().distance.unwrap()
    }
    fn print(&self) {
        for row in &self.0 {
            for cell in row {
                if let Some(cell) = cell {
                    if cell.start {
                        print!("S");
                    } else if cell.end {
                        print!("E");
                    } else {
                        print!(".");
                    }
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}

fn parse_unsigned(input: &str) -> IResult<&str, usize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        usize::from_str(s)
    })(input)?;

    Ok((i, number))
}
fn parse_point(input: &str) -> IResult<&str, Point> {
    let (i, pair) = separated_pair(parse_unsigned, tag(","), parse_unsigned)(input)?;
    Ok((i, Point {x: pair.0, y: pair.1}))
}
fn parse_input(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, grid) = separated_list1(line_ending, parse_point)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, grid))
}

pub(crate) fn solve(input: String) -> (i32, String) {
    let (_, input_bytes) = parse_input(&*input).unwrap();
    let width = 71;
    let height = 71;
    let mut grid = Grid::new(&input_bytes[..1024], width, height);
    let last = grid.3.to_string();
    let part_1_result = grid.find_shortest_path();

    let mut result = 0;
    let mut bytes = 1025;
    let mut last = String::new();
    while result < i32::MAX {
        grid = Grid::new(&input_bytes[..bytes], width, height);
        last = grid.3.to_string();
        result = grid.find_shortest_path();
        bytes += 1;
    }
    (part_1_result, last)
}
#[cfg(test)]
mod tests {
    use super::*;

    mod integration {
        use super::*;
        #[test]
        fn test_provided() {
            let input = get_input();
            let (_, grid) = parse_input(&*input).unwrap();
            let width = 7;
            let height = 7;
            let mut grid = Grid::new(&grid[..12], width, height);
            let result = grid.find_shortest_path();
            grid.print();
            assert_eq!(result,22);
        }
        #[test]
        fn test_provided_2() {
            let input = get_input();
            let (_, input_bytes) = parse_input(&*input).unwrap();
            let width = 7;
            let height = 7;
            let mut grid = Grid::default();
            let mut result = 0;
            let mut bytes = 13;
            while result < i32::MAX {
                grid = Grid::new(&input_bytes[..bytes], width, height);
                result = grid.find_shortest_path();
                bytes += 1;
            }
            grid.print();
            assert_eq!(grid.3, Point::new(6, 1));
        }

        fn get_input() -> String {
            "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
".to_string()
        }
    }
}