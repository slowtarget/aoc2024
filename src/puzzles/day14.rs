use std::str::FromStr;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize
}
#[derive(Debug, PartialEq)]
struct Velocity {
    x: isize,
    y: isize
}

#[derive(Debug, PartialEq)]
struct Robot {
    start: Point,
    velocity: Velocity
}
// Helper function to parse a single unsigned integer
fn parse_unsigned(input: &str) -> IResult<&str, usize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        usize::from_str(s)
    })(input)?;

    Ok((i, number))
}
// Helper function to parse a single signed integer
fn parse_signed(input: &str) -> IResult<&str, isize> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| {
        isize::from_str(s)
    })(input)?;

    Ok((i, number))
}
fn parse_point(input: &str) -> IResult<&str, Point> {
    let (i, pair) = separated_pair(parse_unsigned, tag(","), parse_unsigned)(input)?;
    Ok((i, Point {x: pair.0, y: pair.1}))
}
fn parse_velocity(input: &str) -> IResult<&str, Velocity> {
    let (i, pair) = separated_pair(parse_signed, tag(","), parse_signed)(input)?;
    Ok((i, Velocity {x: pair.0, y: pair.1}))
}
//p=0,4 v=3,-3
fn parse_robot(input: &str) -> IResult<&str, Robot>  {
    let (i, result) = preceded(
        tag("p="), separated_pair(parse_point, tag(" v="), parse_velocity))(input)?;
    Ok((i, Robot {start: result.0, velocity: result.1}))
}
fn parse(input: String) -> Vec<Robot> {
    separated_list1(tag("\n"), parse_robot)(input.trim()).unwrap().1
}
fn robot_move(robot: &Robot, width: usize, height: usize, time: usize) -> Point {
    Point {
        x: (robot.start.x + time * (robot.velocity.x + width as isize) as usize) % width,
        y: (robot.start.y + time * (robot.velocity.y + height as isize) as usize) % height
    }
}
fn get_quadrant(point: &Point, width: usize, height: usize) -> Option<usize> {
    let x = point.x;
    let y = point.y;
    
    let mid_x = width / 2;
    let mid_y = height / 2;
    
    if x == mid_x || y == mid_y {
        None
    } else {
        let quad_x = if x < mid_x { 0 } else { 1 };
        let quad_y = if y < mid_y { 0 } else { 1 };
        Some(quad_x + quad_y * 2)
    }
}

fn print_grid(points: &Vec<Point>, width: usize, height: usize) {
    let mut grid = vec!{0; height * width};
    points.iter().for_each(| Point {x,y}| grid[*y * width + *x] +=1);
    for y in 0..height {
        println!(
            "{}",
            grid[y * width..(y + 1) * width]
                .iter()
                .map(|num: &i32| {
                    if *num == 0 {
                        " ".to_owned()
                    } else {
                        num.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        );
    }
}
fn get_safety_factor(input: &Vec<usize>) -> i32 {
    let mut factors = vec!{0; 4};
    for quadrant in input {
            factors[*quadrant] += 1;
    }
    factors.iter().product()
}

fn part_1(robots: &Vec<Robot>, width: usize, height: usize, time: usize) -> i32 {
    get_safety_factor(  &robots.iter().map(|robot| robot_move(robot, width, height, time))
        .filter_map(|point: Point |get_quadrant(&point,width,height))
                            .collect())    
}

fn part_2(robots: &Vec<Robot>, width: usize, height: usize) -> usize {
    let mut time = 0;
    let mut min_tree = 243724976;
    let mut min_tree_time = 0;

    loop {
        let points = robots.iter().map(|robot| robot_move(robot, width, height, time));
        let tree = get_safety_factor(  &points
            .filter_map(|point: Point |get_quadrant(&point,width,height))
            .collect());

        if tree < min_tree {
            min_tree = tree;
            min_tree_time = time;
            println!("min found: {} {} {}", robots.len(), time, min_tree);
            print_grid(&robots.iter().map(|robot| robot_move(robot, width, height, time)).collect(), width as usize, height as usize);
        }
        time += 1;
        if time > 10000 {
            println!("returning out of time {}", time );
            return min_tree_time;
        }
    }
}

pub(crate) fn solve(input: String) -> (i32, i32) {
    let robots = parse(input);
    println!(" {}", &robots.len());

    let part_2_result = part_2(&robots, 101, 103);
    print_grid(&robots.iter().map(|robot| robot_move(robot, 101, 103, part_2_result)).collect(), 101, 103);
    println!("part 2: {}", part_2_result);
    
    (part_1(&robots, 101, 103, 100), part_2_result as i32)
}
#[cfg(test)]
mod tests {
    use super::*;
    mod parse_tests {
        use super::*;
        mod parse_value_tests {
            use super::*;
            #[test]
            fn parse_value_test() {
                assert_eq!(parse_unsigned("123"), Ok(("", 123)));
            }
            #[test]
            fn parse_value_with_trailing_bob_test() {
                assert_eq!(parse_unsigned("123 bob"), Ok((" bob", 123)));
            }
            #[test]
            fn parse_value_signed_test() {
                assert_eq!(parse_signed("-123"), Ok(("", -123)));
            }
            #[test]
            fn parse_value_error_test() {
                let expected_error = Err(nom::Err::Error(nom::error::Error::new("abc", nom::error::ErrorKind::Digit)));
                assert_eq!(parse_unsigned("abc"), expected_error);
            }
        }
        mod parse_point_tests {
            use super::*;
            #[test]
            fn parse_point_test() {
                let expected = Ok(("", Point {x: 0, y: 4}));
                assert_eq!(parse_point("0,4"), expected);
            }
            #[test]
            fn parse_point_2_test() {
                let expected = Ok(("", Point {x: 1, y: 5}));
                assert_eq!(parse_point("1,5"), expected);
            }
        }
        mod parse_robot_tests {
            use super::*;
            #[test]
            fn parse_robot_test() {
                let expected = Ok(("",Robot {start: Point {x: 0, y: 4}, velocity: Velocity {x: 3, y: -3}}));
                assert_eq!(parse_robot("p=0,4 v=3,-3"), expected);
            }
            #[test]
            fn parse_robot_2_test() {
                let expected = Ok(("",Robot {start: Point {x: 1, y: 5}, velocity: Velocity {x: 4, y: -4}}));
                assert_eq!(parse_robot("p=1,5 v=4,-4"), expected);
            }

        }
        mod parse_test {
            use super::*;
            #[test]
            fn parse_simple_test() {
                let input = "\
p=0,4 v=3,-3
p=1,5 v=4,-4
";
                let expected = vec![
                    Robot {start: Point {x: 0, y: 4}, velocity: Velocity {x: 3, y: -3}},
                    Robot {start: Point {x: 1, y: 5}, velocity: Velocity {x: 4, y: -4}}
                ];
                assert_eq!(parse(input.to_string()), expected);
            }
        }
    }
    mod part_1_tests {
        use super::*;
        mod robot_move_tests {
            use super::*;
            fn get_robot() -> Robot {
                Robot { start: Point { x: 2, y: 4 }, velocity: Velocity { x: 2, y: -3 } }
            }
            #[test]
            fn provided_1() {
                assert_eq!(robot_move(&get_robot(), 11, 7, 1), Point {x: 4, y: 1});
            }
            #[test]
            fn provided_2() {
                assert_eq!(robot_move(&get_robot(), 11, 7, 2), Point {x: 6, y: 5});
            }
            #[test]
            fn provided_3() {
                assert_eq!(robot_move(&get_robot(), 11, 7, 3), Point {x: 8, y: 2});
            }
            #[test]
            fn provided_4() {
                assert_eq!(robot_move(&get_robot(), 11, 7, 4), Point {x: 10, y: 6});
            }
            #[test]
            fn provided_5() {
                assert_eq!(robot_move(&get_robot(), 11, 7, 5), Point {x: 1, y: 3});
            }
        }
        mod get_quadrant_tests {
            use super::*;
            #[test]
            fn get_quadrant_none_1_test() {
                assert_eq!(get_quadrant(&Point {x: 5, y: 0}, 11, 7), None);
            }
            #[test]
            fn get_quadrant_none_2_test() {
                assert_eq!(get_quadrant(&Point {x: 0, y: 3}, 11, 7), None);
            }
            #[test]
            fn get_quadrant_none_3_test() {
                assert_eq!(get_quadrant(&Point {x: 5, y: 3}, 11, 7), None);
            }
            #[test]
            fn get_quadrant_some_1_test() {
                assert_eq!(get_quadrant(&Point {x: 0, y: 0}, 11, 7), Some(0));
            }
            #[test]
            fn get_quadrant_some_2_test() {
                assert_eq!(get_quadrant(&Point {x: 6, y: 2}, 11, 7), Some(1));
            }
            #[test]
            fn get_quadrant_some_3_test() {
                assert_eq!(get_quadrant(&Point {x: 0, y: 4}, 11, 7), Some(2));
            }
            #[test]
            fn get_quadrant_some_4_test() {
                assert_eq!(get_quadrant(&Point {x: 6, y: 6}, 11, 7), Some(3));
            }
            
        }
        mod get_safety_factor_tests {
            use super::*;
            #[test]
            fn get_safety_factor_test() {
                let quadrants = vec![
                    0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2
                ];
                assert_eq!(get_safety_factor(&quadrants), 3 * 3 * 3 * 2);
            }
        }
    }
}