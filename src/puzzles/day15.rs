use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Debug, Clone, PartialEq, Default)]
struct Path {
    distance: [Option<i32>; 4],
    location: Point,
    start: bool,
    bx: bool,
    neighbours: [Option<Point>; 4],
}
impl Path {
    fn new(location: Point) -> Self {
        Self {
            distance: [None, None, None, None],
            location,
            start: false,
            bx: false,
            neighbours: [None, None, None, None],
        }
    }
    fn start(location: Point) -> Self {
        Self {
            distance: [Some(1000), Some(0), Some(1000), Some(2000)],
            location,
            start: true,
            bx: false,
            neighbours: [None, None, None, None],
        }
    }
    fn bx(location: Point) -> Self {
        Self {
            distance: [None, None, None, None],
            location,
            start: false,
            bx: true,
            neighbours: [None, None, None, None],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn neighbour(&self) -> usize {
        match self {
            Direction::Up => 0,    // N
            Direction::Right => 1, // E
            Direction::Down => 2,  // S
            Direction::Left => 3,  // W
        }
    }
}
fn parse_arrows(input: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    separated_list1(
        line_ending,
        many1(map(one_of("<>^v"), |c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!(),
        })),
    )(input)
}
fn parse_map(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    separated_list1(line_ending, many1(one_of("#.@O")))(input)
}
fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Direction>)> {
    let (input, (grid, moves)) = separated_pair(parse_map, tag("\n\n"), parse_arrows)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, (grid, moves.into_iter().flatten().collect())))
}
fn map_input(input: Vec<Vec<char>>) -> (Vec<Vec<Option<Path>>>, Point) {
    let mut paths = Vec::new();
    let mut start: Point = Default::default();
    for (y, row) in input.iter().enumerate() {
        let mut path_row = Vec::new();
        for (x, cell) in row.iter().enumerate() {
            let path = match cell {
                '#' => None,
                '.' => Some(Path::new(Point::new(x, y))),
                '@' => Some(Path::start(Point::new(x, y))),
                'O' => Some(Path::bx(Point::new(x, y))),
                _ => panic!("Invalid cell"),
            };
            if path.is_some() && path.as_ref().unwrap().start {
                start = path.as_ref().unwrap().location.clone();
            }
            path_row.push(path);
        }
        paths.push(path_row);
    }
    (paths, start)
}
fn populate_neighbours(maze: &mut Vec<Vec<Option<Path>>>) {
    let neighbours: Vec<(isize, isize)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)]; // N,E,S,W
    let maze_copy = maze.clone();
    for (y, row) in maze.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            if let Some(cell) = cell {
                let neighbour_cells = neighbours
                    .iter()
                    .map(|(dx, dy)| {
                        &maze_copy[(y as isize + dy) as usize][(x as isize + dx) as usize]
                    })
                    .map(|cell| cell.as_ref().map(|c| c.location.clone()))
                    .collect::<Vec<Option<Point>>>();
                cell.neighbours = neighbour_cells.try_into().unwrap();
            }
        }
    }
}
fn part_1(input: &String) -> usize {
    let (_, (grid, moves)) = parse_input(&input).unwrap();
    let (mut paths, start) = map_input(grid);
    populate_neighbours(&mut paths);

    let mut current = start;
    for move_ in moves {
        let (target, next, pushing) = {
            let mut position = current.clone();
            let mut path = paths[position.y][position.x].as_ref().unwrap();
            let mut pushing = false;
            let next = {
                if path.neighbours[move_.neighbour()].is_some() {
                    Some(path.neighbours[move_.neighbour()].as_ref().unwrap().clone())
                } else {
                    None
                }
            };
            while next.is_some() && path.neighbours[move_.neighbour()].is_some() && (path.bx || position == current) {
                // while we can move in the direction and we are on a box
                position = path.neighbours[move_.neighbour()].as_ref().unwrap().clone();
                path = paths[position.y][position.x].as_ref().unwrap();
                pushing = pushing || path.bx;
            }

            if path.bx || position == current {
                // if we are still on a box then we cannot move
                (None, next, false)
            } else {
                (Some(position), next, pushing)
            }
        };
        if target.is_none() {
            continue;
        }
        current = next.unwrap();
        if pushing {
            paths[current.y][current.x].as_mut().unwrap().bx = false;
            let Point { x, y } = target.unwrap();
            paths[y][x].as_mut().unwrap().bx = true;
        }
    }
    let mut sum = 0;
    for row in paths.iter() {
        for cell in row.iter().flatten() {
            if cell.bx {
                let Point { x, y } = cell.location;
                sum += y * 100 + x;
            }
        }
    }
    sum
}

pub(crate) fn solve(input: String) -> (i32, i32) {
    (part_1(&input) as i32, 0)
}



#[cfg(test)]
mod tests {
    use super::*;

    mod parse_tests {
        use super::*;

        #[test]
        fn parse_input_test() {
            let input = get_smaller_input();
            let result = parse_input(&input);
            assert!(result.is_ok());
            let (remaining, (grid, moves)) = result.unwrap();
            assert_eq!(remaining, "");
            assert_eq!(
                grid,
                vec![
                    vec!['#', '#', '#', '#', '#', '#', '#', '#'],
                    vec!['#', '.', '.', 'O', '.', 'O', '.', '#'],
                    vec!['#', '#', '@', '.', 'O', '.', '.', '#'],
                    vec!['#', '.', '.', '.', 'O', '.', '.', '#'],
                    vec!['#', '.', '#', '.', 'O', '.', '.', '#'],
                    vec!['#', '.', '.', '.', 'O', '.', '.', '#'],
                    vec!['#', '.', '.', '.', '.', '.', '.', '#'],
                    vec!['#', '#', '#', '#', '#', '#', '#', '#'],
                ]
            );
            assert_eq!(
                moves,
                vec![
                    Direction::Left,
                    Direction::Up,
                    Direction::Up,
                    Direction::Right,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Down,
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Left
                ]
            );
        }
    }
    fn get_smaller_input() -> String {
        "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>
v<<
"
        .to_string()
    }
    fn get_input() -> String {
        "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"
        .to_string()
    }
}
