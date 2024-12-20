// --- Day 16: Reindeer Maze ---

use std::collections::HashSet;
use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;
#[derive(Debug, Clone, PartialEq, Default)]
struct Path {
    distance: [Option<i32>; 4],
    location: Point,
    start: bool,
    end: bool,
    neighbours: [Option<Point>; 4],
}
impl Path {
    fn new(location: Point) -> Self {
        Self {
            distance: [None,None,None,None],
            location,
            start: false,
            end: false,
            neighbours: [None, None, None, None],
        }
    }
    fn start(location: Point) -> Self {
        Self {
            distance: [Some(1000),Some(0),Some(1000),Some(2000)],
            location,
            start: true,
            end: false,
            neighbours: [None, None, None, None],
        }
    }
    fn end(location: Point) -> Self {
        Self {
            distance: [None,None,None,None],
            location,
            start: false,
            end: true,
            neighbours: [None, None, None, None],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default)]
#[derive(Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, grid) = separated_list1(line_ending, many1(one_of("#.ES")))(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, grid))
}

fn map_input(input: Vec<Vec<char>>) -> (Vec<Vec<Option<Path>>>, Point, Point) {
    let mut paths = Vec::new();
    let mut start: Point = Default::default();
    let mut end: Point = Default::default();
    for (y, row) in input.iter().enumerate() {
        let mut path_row = Vec::new();
        for (x, cell) in row.iter().enumerate() {
            let path = match cell {
                '#' => None,
                '.' => Some(Path::new(Point::new(x, y))),
                'E' => Some(Path::end(Point::new(x, y))),
                'S' => Some(Path::start(Point::new(x, y))),
                _ => panic!("Invalid cell"),
            };
            if path.is_some() && path.as_ref().unwrap().start {
                start = path.as_ref().unwrap().location.clone();
            }
            if path.is_some() && path.as_ref().unwrap().end {
                end = path.as_ref().unwrap().location.clone();
            }
            path_row.push(path);
        }
        paths.push(path_row);
    }
    (paths, start, end)
}
fn populate_neighbours(maze: &mut Vec<Vec<Option<Path>>>) {
    let neighbours: Vec<(isize, isize)> = vec![(0,-1),(1,0),(0,1),(-1,0)]; // N,E,S,W
    let maze_copy = maze.clone();
    for (y, row) in maze.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            if let Some(cell) = cell {
                let neighbour_cells = neighbours
                    .iter()
                    .map(|(dx, dy)|&maze_copy[(y as isize + dy) as usize][(x as isize + dx) as usize] )
                    .map(|cell| cell.as_ref().map(|c| c.location.clone()))
                    .collect::<Vec<Option<Point>>>();
                cell.neighbours = neighbour_cells.try_into().unwrap();
            }
        }
    }
}
fn part_1(input: &Vec<Vec<Option<Path>>>, start: &Point, end: &Point) -> (i32, Vec<Vec<Option<Path>>>) {
    let mut queue: std::vec::Vec<(usize, Point)> = Vec::new();
    let mut maze = input.clone();
    // to turn from my current direction to the new one will take TURNS[current_direction][direction] 90 degree TURNS

    queue.push((1, start.clone()));
    while let Some((current_direction, current)) = queue.pop() {
        // Extract what we need in a block so current_path is dropped at the end of the block.
        let (current_distance, neighbours) = {
            let current_path = maze[current.y][current.x].as_ref().unwrap();
            (current_path.distance.clone(), current_path.neighbours.clone())
        };
        for (direction, neighbour) in neighbours.iter().enumerate() {
            if let Some(neighbour) = neighbour {
                // currently heading in current_direction from current
                // the cost to get to current, headed in current_direction is current_distance[current_direction]
                // the cost to get to neighbour, is the cost to turn towards that neighbour and then move 1 space
                // so it's current_distance[current_direction] + 1000 * TURNS[current_direction][direction] + 1

                let distance = current_distance[current_direction].unwrap() + 1 + TURNS[current_direction][direction] * 1_000;

                let neighbour_cell = maze[neighbour.y][neighbour.x].as_mut().unwrap();
                let neighbour_distance = neighbour_cell.distance[direction].unwrap_or(i32::MAX);

                if neighbour_distance > distance {
                    neighbour_cell.distance[direction] = Some(distance);
                    // what about the other directions? could we have found the cheapest way to arrive here and be facing in one of the other directions?
                    for d in 0..4 {
                        if d == direction {
                            continue;
                        }
                        let nd = neighbour_cell.distance[d].unwrap_or(i32::MAX);
                        let dist = distance + TURNS[direction][d] * 1_000;
                        if nd > dist {
                            neighbour_cell.distance[d] = Some(dist);
                        }
                    }
                    queue.push((direction, neighbour.clone()));
                }
                neighbour_cell.distance[direction] = Some(neighbour_cell.distance[direction].unwrap().min(distance));
            }
        }
    }
    let min_distance = maze[end.y][end.x]
        .as_ref().unwrap()
        .distance
        .iter()
        .filter_map(|&d| d) // Convert &Option<i32> to Option<i32> and filter out None
        .min(); // Returns Option<i32>
    (
        min_distance.unwrap(),
        maze
    )
}

static TURNS: [[i32; 4]; 4]  = [[0, 1, 2, 1], [1, 0, 1, 2], [2, 1, 0, 1], [1, 2, 1, 0]];
    // TURNS needed        N  E  S  W
    // current direction : 0  1  2  3
    // direction         0 0  1  2  1
    //                   1 1  0  1  2
    //                   2 2  1  0  1
    //                   3 1  2  1  0
    // I'm sure that there's a formula, but I cannot see it!
    // to turn from my current direction to the new one will take TURNS[current_direction][direction] 90 degree TURNS

fn part_2(input: &Vec<Vec<Option<Path>>>, _start: &Point, end: &Point, best: &i32) -> i32 {
    // walk back from the end visiting all the cells on the least cost paths
    // nope - needs some more thought - calculate the cost of the move
    // and its on the least cost path if they match
    static REVERSE: fn(usize) -> usize = |x:usize| (x + 2) % 4;
    let mut queue: std::vec::Vec<(usize, Point)> = Vec::new();
    let maze = input.clone();
    let mut visited: HashSet<Point> = HashSet::new();
    let (end_distance, _neighbours) = {
        let current_path = maze[end.y][end.x].as_ref().unwrap();
        (current_path.distance.clone(), current_path.neighbours.clone())
    };
    end_distance.iter().enumerate()
        .filter_map(|(i,&d)|if d.is_some() && d.unwrap() == *best {Some(i)} else {None})
        .for_each(|direction| {
            queue.push((direction, end.clone()));
    });
    
    while let Some( (current_direction, current)) = queue.pop() {

        let (current_distance, neighbours) = {
            let current_path = maze[current.y][current.x].as_ref().unwrap();
            (current_path.distance[current_direction].unwrap(), current_path.neighbours.clone())
        };

        visited.insert(current.clone());

        neighbours
            .iter()
            .enumerate()
            .filter_map(|(direction, neighbour):(usize, &Option<Point>)| if neighbour.is_some() {Some((REVERSE(direction.clone()), neighbour.clone().unwrap()))} else {None})
            .filter(|(reverse_direction, neighbour): &(usize, Point)| {
                let neighbour_cell = maze[neighbour.y][neighbour.x].as_ref().unwrap();
                let distance = neighbour_cell.distance[*reverse_direction].unwrap();

                distance + 1 + 1000 * TURNS[current_direction][*reverse_direction] == current_distance
            })
            .for_each(|neighbour| {
                queue.push(neighbour.clone());
            });
    }
    visited.len() as i32
}
pub fn solve(input: String) -> (i32, i32) {
    let (_, grid) = parse_input(&input).unwrap();
    let (maze, start, end) = map_input(grid);
    let mut maze = maze;
    populate_neighbours(&mut maze);
    let (part_1_result, maze) = part_1(&maze, &start, &end);
    let part_2_result = part_2(&maze, &start, &end, &part_1_result);

    (part_1_result, part_2_result)
}
#[cfg(test)]
mod tests {
    use super::*;
    mod parse_input_tests {
        use super::*;
        #[test]
        fn parse_input_simple_test() {
            let input = "#.ES#\n";
            let result = parse_input(input);
            assert_eq!(result, Ok(("", vec![vec!['#', '.', 'E', 'S', '#']])));
        }
        #[test]
        fn parse_input_multiple_rows_test() {
            let input = "#.ES#\n#.#.#\n";
            let result = parse_input(input);
            assert_eq!(
                result,
                Ok((
                    "",
                    vec![vec!['#', '.', 'E', 'S', '#'], vec!['#', '.', '#', '.', '#']]
                ))
            );
        }
    }
    mod map_input_tests {
        use super::*;
        #[test]
        fn map_input_simple_test() {
            let input = vec![vec!['#', '.', 'E', 'S', '#']];
            let result = map_input(input);
            assert_eq!(
                result,
                (
                    vec![vec![
                        None,
                        Some(Path::new(Point::new(1, 0))),
                        Some(Path::end(Point::new(2, 0))),
                        Some(Path::start(Point::new(3, 0))),
                        None
                    ]],
                    Point::new(3, 0),
                    Point::new(2, 0)
                )
            );
        }
        #[test]
        fn map_input_multiple_rows_test() {
            let input = vec![vec!['#', '.', 'E', 'S', '#'], vec!['#', '.', '#', '.', '#']];
            let result = map_input(input);
            assert_eq!(
                result,
                (
                    vec![
                        vec![
                            None,
                            Some(Path::new(Point::new(1, 0))),
                            Some(Path::end(Point::new(2, 0))),
                            Some(Path::start(Point::new(3, 0))),
                            None
                        ],
                        vec![
                            None,
                            Some(Path::new(Point::new(1, 1))),
                            None,
                            Some(Path::new(Point::new(3, 1))),
                            None
                        ]
                    ],
                    Point::new(3, 0),
                    Point::new(2, 0)
                )
            );
        }
    }
    mod point_tests {
        use super::*;
        #[test]
        fn point_new_test() {
            let p = Point::new(1, 2);
            assert_eq!(p, Point { x: 1, y: 2 });
        }
        #[test]
        fn point_deconstruct_test() {
            let p = Point { x: 1, y: 2 };
            assert_eq!(p.x, 1);
            assert_eq!(p.y, 2);
        }
        #[test]
        fn point_clone_test() {
            let p = Point { x: 1, y: 2 };
            let p2 = p.clone();
            assert_eq!(p, p2);
        }
        #[test]
        fn point_default_test() {
            let p = Point::default();
            assert_eq!(p, Point { x: 0, y: 0 });
        }
        #[test]
        fn point_spread_test() {
            let p = Point { x: 1, y: 2 };
            let Point { x, y } = p;
            assert_eq!(x, 1);
            assert_eq!(y, 2);
        }
    }
    mod path_tests {
        use super::*;
        #[test]
        fn path_new_test() {
            let p = Path::new(Point::new(1, 2));
            assert_eq!(
                p,
                Path {
                    distance: [Default::default(),Default::default(),Default::default(),Default::default()],
                    location: Point { x: 1, y: 2 },
                    start: false,
                    end: false,
                    neighbours: [None, None, None, None]
                }
            );
        }
        #[test]
        fn path_start_test() {
            let p = Path::start(Point::new(1, 2));
            assert_eq!(
                p,
                Path {
                    distance: [Some(1000),Some(0),Some(1000),Some(2000)],
                    location: Point { x: 1, y: 2 },
                    start: true,
                    end: false,
                    neighbours: [None, None, None, None]
                }
            );
        }
        #[test]
        fn path_end_test() {
            let p = Path::end(Point::new(1, 2));
            assert_eq!(
                p,
                Path {
                    distance: [Default::default(),Default::default(),Default::default(),Default::default()],
                    location: Point { x: 1, y: 2 },
                    start: false,
                    end: true,
                    neighbours: [None, None, None, None]
                }
            );
        }
    }
    mod part_1_tests {
        use super::*;
        #[test]
        fn part_1_simple_test() {
            let start = Point::new(1, 1);
            let end = Point::new(2, 1);
            let mut maze = vec![
                vec![None,None,None,None],
                vec![None, Some(Path::start(start.clone())), Some(Path::end(end.clone())), None],
                vec![None,None,None,None],
            ];

            populate_neighbours(&mut maze);

            let (result,_) = part_1(&maze, &start, &end);
            assert_eq!(result, 1);
        }
        #[test]
        fn part_1_simple_east_test() {
            let start = Point::new(1, 1);
            let end = Point::new(4, 1);
            let mut maze = vec![
                vec![None,None,None,None,None,None],
                vec![None, Some(Path::start(start.clone())), Some(Path::new(Point::new(2,1))),Some(Path::new(Point::new(3,1))) , Some(Path::end(end.clone())), None],
                vec![None,None,None,None,None,None],
            ];
            populate_neighbours(&mut maze);
            let (result,_) = part_1(&maze, &start, &end);
            assert_eq!(result, 3);
        }
    }
    mod integration {
        use super::*;
        #[test]
        fn provided() {
           let input = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
            let (_, grid) = parse_input(input).unwrap();
            let (maze, start, end) = map_input(grid);
            let mut maze = maze;
            populate_neighbours(&mut maze);
            let (result,maze) = part_1(&maze, &start, &end);
            assert_eq!(result, 7036);
            let result = part_2(&maze, &start, &end, &result);
            assert_eq!(result, 45);
        }
        #[test]
        fn provided_another() {
           let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
            let (_, grid) = parse_input(input).unwrap();
            let (maze, start, end) = map_input(grid);
            let mut maze = maze;
            populate_neighbours(&mut maze);
            let (result,maze) = part_1(&maze, &start, &end);
            assert_eq!(result, 11048);
            let result = part_2(&maze, &start, &end, &result);
            assert_eq!(result, 64);
        }
        #[test]
        fn simple() {
           let input = "\
#####
#...#
#S#E#
#...#
#####
";
            let (_, grid) = parse_input(input).unwrap();
            let (maze, start, end) = map_input(grid);
            let mut maze = maze;
            populate_neighbours(&mut maze);
            let (result,maze) = part_1(&maze, &start, &end);
            assert_eq!(result, 3004);
            print_grid(&maze);
            let result = part_2(&maze, &start, &end, &result);
            assert_eq!(result, 8);
        }
    }

    fn print_grid(maze: &Vec<Vec<Option<Path>>>) {
        for row in maze.iter() {
            for d in 0..4 {
                for cell in row.iter() {
                    if let Some(cell) = cell {
                        print!("{} ", cell.distance[d].unwrap_or(0));
                    } else {
                        print!("     ");
                    }
                }
                println!();
            }
            println!();
        }
    }

}
