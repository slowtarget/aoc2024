use nom::character::complete::{line_ending, one_of};
use nom::multi::{many1, separated_list1};
use nom::IResult;
use num::abs;

#[derive(Debug, Clone, PartialEq, Default)]
struct Path {
    distance: i32,
    location: Point,
    start: bool,
    end: bool,
    neighbours: [Option<Point>; 4],
}
impl Path {
    fn new(location: Point) -> Self {
        Self {
            distance: i32::MAX,
            location,
            start: false,
            end: false,
            neighbours: [None, None, None, None],
        }
    }
    fn start(location: Point) -> Self {
        Self {
            distance: 0,
            location,
            start: true,
            end: false,
            neighbours: [None, None, None, None],
        }
    }
    fn end(location: Point) -> Self {
        Self {
            distance: i32::MAX,
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
fn find_shortest_path( grid: &mut Vec<Vec<Option<Path>>>, start:Point, end :Point) -> i32 {
    // let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: Vec<Point> = Vec::new();
    queue.push(start);
    while let Some(current) = queue.pop() {
        
        let ( neighbours, current_distance) = {
            let current_path = &grid[current.y][current.x].as_ref().unwrap();
            (current_path.neighbours.clone(),
             current_path.distance)
        };
        for neighbour in neighbours.into_iter().flatten() {
            let neighbour_path = &mut grid[neighbour.y][neighbour.x].as_mut().unwrap();
            let distance = current_distance + 1;
            if  neighbour_path.distance > distance {
                neighbour_path.distance = distance;
                queue.push(neighbour.clone());
            }
        }
    }
    grid[end.y][end.x].as_ref().unwrap().distance
}


fn part_1_deprecated(maze: &[Vec<Option<Path>>], required_saving: i32) -> i32 {
    let neighbours: Vec<(isize, isize)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)]; // N,E,S,W
    let width = maze[0].len() as isize;
    let height = maze.len() as isize;
    let mut count = 0;
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_none() {
                for i in 0..3 {
                    for j in (i + 1)..4 {
                        let ny = y as isize + neighbours[i].1;
                        let nx = x as isize + neighbours[i].0;
                        let ny2 = y as isize + neighbours[j].1;
                        let nx2 = x as isize + neighbours[j].0;

                        if ny >= 0 && nx >= 0 && ny < height && nx < width && ny2 >= 0 && nx2 >= 0 && ny2 < height && nx2 < width {
                            let cell1 = &maze[ny as usize][nx as usize];
                            let cell2 = &maze[ny2 as usize][nx2 as usize];
                            if cell1.is_some() && cell2.is_some() {
                                let distance1 = &maze[ny as usize][nx as usize].as_ref().unwrap().distance;
                                let distance2 = &maze[ny2 as usize][nx2 as usize].as_ref().unwrap().distance;
                                if abs(distance1 - distance2) > required_saving + 1{
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
fn part_1(maze: &[Vec<Option<Path>>], required_saving: i32) -> i32 {
    part_2(maze, required_saving, 2)
}
fn part_2(maze: &[Vec<Option<Path>>], required_saving: i32, cheats: isize) -> i32 {
    let width = maze[0].len();
    let height = maze.len();
    let mut visited = vec!{vec!{false; width}; height};
    let mut count = 0;
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_some() {
                let path = cell.as_ref().unwrap();
                visited[y][x] = true;
                for dx in -cheats..=cheats {
                    for dy in -cheats..=cheats {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if dx.abs() + dy.abs() > cheats {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                            let cell = &maze[ny as usize][nx as usize];
                            if cell.is_some() {
                                if visited[ny as usize][nx as usize] {
                                    continue;
                                }
                                
                                let distance = cell.as_ref().unwrap().distance;
                                let saving = abs(distance - path.distance);
                                let cost: i32 = (dx.abs() + dy.abs()) as i32;
                                if saving - cost >= required_saving {
                                    count += 1;
                                    // println!("{} saving: {} cost: {} dx: {} dy: {} x: {} y: {} nx: {} ny: {}", count, saving, cost, dx, dy, x, y, nx, ny);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
fn prep_input(input: &str) -> Vec<Vec<Option<Path>>> {
    let (_, grid) = parse_input(&input).unwrap();
    let (maze, start, end) = map_input(grid);
    let mut maze = maze;
    populate_neighbours(&mut maze);
    println!("shortest: {}",find_shortest_path(&mut maze, start, end));
    maze
}
pub fn solve(input: String) -> (i32, i32) {

    let maze = prep_input(&input);
    let part_1_result = part_1(&maze, 100);
    let part_2_result = part_2(&maze, 100, 20); // 7 647 is too low. // 582991 is too low. // 555710 :(

    (part_1_result, part_2_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    mod part_1_tests {
        use super::*;
        #[test]
        fn provided_shortest_path_test() {
            let input = get_input();
            let mut grid;
            let start:Point;
            let end:Point;
            let (_, chars) = parse_input(input).unwrap();
            (grid, start, end) = map_input( chars);
            populate_neighbours(&mut grid);
            let shortest_path= find_shortest_path(&mut grid, start, end);
            assert_eq!(shortest_path, 84);
        }
        #[test]
        fn provided_1_64_test() {
            let maze = prep_input(&get_input());
            let part_1_result = part_1(&maze, 64);
            assert_eq!(part_1_result, 1);
        }

        #[test]
        fn provided_2_40_test() {
            let maze = prep_input(&get_input());
            let part_1_result = part_1(&maze, 40);
            assert_eq!(part_1_result, 2);
        }

        #[test]
        fn provided_5_20_test() {
            let maze = prep_input(&get_input());
            let part_1_result = part_1(&maze, 20);
            assert_eq!(part_1_result, 5);
        }

        #[test]
        fn provided_10_10_test() {
            let maze = prep_input(&get_input());
            let part_1_result = part_1(&maze, 10);
            assert_eq!(part_1_result, 10);
        }

        #[test]
        fn provided_16_6_test() {
            let maze = prep_input(&get_input());
            let part_1_result = part_1(&maze, 6);
            assert_eq!(part_1_result, 16);
        }
        // There are 14 cheats that save 2 picoseconds.
        // There are 14 cheats that save 4 picoseconds.
        // There are 2 cheats that save 6 picoseconds.
        // There are 4 cheats that save 8 picoseconds.
        // There are 2 cheats that save 10 picoseconds.
        // There are 3 cheats that save 12 picoseconds.
        // There is one cheat that saves 20 picoseconds.
        // There is one cheat that saves 36 picoseconds.
        // There is one cheat that saves 38 picoseconds.
        // There is one cheat that saves 40 picoseconds.
        // There is one cheat that saves 64 picoseconds.
        // 14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1 = 44

        #[test]
        fn provided_lots_4_test() {
            let maze = prep_input(&get_input());
            assert_eq!(part_1(&maze, 4), 30); // 63?
        }
        
        #[test]
        fn provided_lots_2_test() {
            let maze = prep_input(&get_input());
            assert_eq!(part_1(&maze, 2), 44); // 63?
        }
    }
    mod part_2_tests {
        use super::*;
        #[test]
        fn provided_76_6_1_test() { // This six-picosecond cheat saves 76 picoseconds
            let maze = prep_input(&get_input());
            assert_eq!(part_2(&maze, 76, 6), 1);
        }
        #[test]
        fn provided_76_20_3_test() { 
            let maze = prep_input(&get_input());
            assert_eq!(part_2(&maze, 76, 20), 3);
        }
        #[test]
        fn provided_74_20_7_test() { 
            let maze = prep_input(&get_input());
            assert_eq!(part_2(&maze, 74, 20), 3 + 4);
        }
        #[test]
        fn provided_72_20_29_test() { 
            let maze = prep_input(&get_input());
            assert_eq!(part_2(&maze, 72, 20), 3 + 4 + 22);
        }
        #[test]
        fn provided_70_20_41_test() {
            let maze = prep_input(&get_input());
            assert_eq!(part_2(&maze, 70, 20), 3 + 4 + 22 + 12);
        }
        #[test]
        fn provided_68_20_test() {
            let maze = prep_input(&get_input());
            assert_eq!(part_2(&maze, 68, 20), 3 + 4 + 22 + 12 + 14);
        }
        #[test]
        fn provided_66_20_test() {
            let maze = prep_input(&get_input());
            assert_eq!(part_2(&maze, 66, 20), 3 + 4 + 22 + 12 + 14 + 12);
        }
    }
    

    fn get_input() -> &'static str {
        "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
"
    }
}