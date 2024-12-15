use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::time::Instant;

#[derive(Debug)]
struct Machine {
    a_dx: i64,
    a_dy: i64,
    b_dx: i64,
    b_dy: i64,
    target_x: i64,
    target_y: i64,
}

// Helper function to parse a single integer
fn parse_value(input: &str) -> IResult<&str, i64> {
    map_res(digit1, str::parse::<i64>)(input)
}

// Helper function to parse a button's configuration
fn parse_button(prefix: &'static str) -> impl Fn(&str) -> IResult<&str, (i64, i64)> {
    move |input: &str| {
        preceded(
            tag(prefix),
            separated_pair(preceded(tag("X+"), parse_value), tag(", Y+"), parse_value),
        )(input)
    }
}

// Parse the prize's location
fn parse_prize(input: &str) -> IResult<&str, (i64, i64)> {
    preceded(
        tag("Prize: X="),
        separated_pair(parse_value, tag(", Y="), parse_value),
    )(input)
}

// Parse a single machine
fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a_dx, a_dy)) = parse_button("Button a: ")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, (b_dx, b_dy)) = parse_button("Button b: ")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, (target_x, target_y)) = parse_prize(input)?;

    Ok((
        input,
        Machine {
            a_dx,
            a_dy,
            b_dx,
            b_dy,
            target_x,
            target_y,
        },
    ))
}

// Parse the entire input
fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(tag("\n\n"), parse_machine)(input.trim())
}

// Add 10^13 to the prize coordinates
fn update_prize_coordinates(machine: &mut Machine) {
    let offset = 10_000_000_000_000;
    machine.target_x += offset;
    machine.target_y += offset;
}

// Calculate the minimum cost to win the prize for one machine
fn min_cost_to_win(machine: &Machine) -> Option<i64> {
    let (a_dx, a_dy, b_dx, b_dy, target_x, target_y) = (
        machine.a_dx,
        machine.a_dy,
        machine.b_dx,
        machine.b_dy,
        machine.target_x,
        machine.target_y,
    );

    // Brute-force solution since the max presses are limited to 100
    let mut min_cost: Option<i64> = None; // Explicit type annotation
    for a_presses in 0..=100 {
        for b_presses in 0..=100 {
            let x_reached = a_presses * a_dx + b_presses * b_dx;
            let y_reached = a_presses * a_dy + b_presses * b_dy;

            if x_reached == target_x && y_reached == target_y {
                let cost = a_presses * 3 + b_presses;
                min_cost = Some(min_cost.map_or(cost, |c: i64| c.min(cost))); // Explicit closure type
            }
        }
    }
    min_cost
}

// Part 1: Find the total cost to win all possible prizes
fn part1(machines: &[Machine]) -> i64 {
    machines.iter().filter_map(min_cost_to_win).sum()
}

fn solve_machine(machine: &Machine) -> Option<i64> {
    let Machine {
        a_dx, a_dy, b_dx, b_dy, target_x, target_y
    } = *machine;

    // Compute determinant
    let d = a_dx * b_dy - a_dy * b_dx;
    if d == 0 {
        // If D = 0, either no solutions or infinite solutions if proportional.
        // Check if the equations are multiples of each other.
        // If no solution, return None.
        // If infinite solutions, you'd need to check the target alignment.
        // For simplicity, just return None here as a common case:
        println!("Determinant is zero : No solution for machine: {:?}", machine);
        return None;
    }

    // Check if the solution divides evenly
    if ((target_x * b_dy - target_y * b_dx) % d != 0) ||
        ((a_dx * target_y - a_dy * target_x) % d != 0) {
        println!("solution divides evenly : No solution for machine: {:?}", machine);
        return None; // No integer solution.
    }
    //println!("Determinant: {}", d);
    // Particular solution
    let a_0 = (target_x * b_dy - target_y * b_dx) / d;
    let b_0 = (a_dx * target_y - a_dy * target_x) / d;
    
    if a_0 * a_dx + b_0 * b_dx == target_x && a_0 * a_dy + b_0 * b_dy == target_y && a_0 >= 0 && b_0 >= 0 {
        println!("Particular solution accepted: {:?}, A: {}, B: {}, Cost: {}", machine, a_0, b_0, 3 * a_0 + b_0);
        // Compute cost
        let cost = 3 * a_0 + b_0;
        return Some(cost);
    }
    None
}
// Solve for all machines
fn part2(machines: &[Machine]) -> i64 {
    machines.iter().filter_map(solve_machine).sum()
}
// Parse and solve the puzzle
pub fn solve(input: String) -> (i64, i64) {
    let start = Instant::now();
    let (_, mut machines) = parse_input(&input).unwrap();
    let ans_part1 = part1(&machines); // Assuming part1 is already implemented
    machines.iter_mut().for_each(update_prize_coordinates);
    let parse_duration = start.elapsed();

    let start_solve = Instant::now();

    let ans_part2 = part2(&machines);
    let solve_duration = start_solve.elapsed();

    println!("Part 1: {}", ans_part1);
    println!("Part 2: {}", ans_part2);
    println!("Parsing took: {} microseconds", parse_duration.as_micros());
    println!("Solving took: {} microseconds", solve_duration.as_micros());

    (ans_part1, ans_part2)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "Button a: X+94, Y+34\nButton b: X+22, Y+67\nPrize: X=8400, Y=5400\n";
        let (_, machine) = parse_machine(input).unwrap();
        assert_eq!(machine.a_dx, 94);
        assert_eq!(machine.a_dy, 34);
        assert_eq!(machine.b_dx, 22);
        assert_eq!(machine.b_dy, 67);
        assert_eq!(machine.target_x, 8400);
        assert_eq!(machine.target_y, 5400);
    }

    mod part_1_test {
        use super::*;
        #[test]
        fn test_first_machine() {
            let input = "\
Button a: X+94, Y+34
Button b: X+22, Y+67
Prize: X=8400, Y=5400";
            let (_, machines) = parse_input(input).unwrap();
            assert_eq!(part1(&machines), 280);
        }
        #[test]
        fn test_third_machine() {
            let input = "\
Button a: X+17, Y+86
Button b: X+84, Y+37
Prize: X=7870, Y=6450";
            let (_, machines) = parse_input(input).unwrap();
            assert_eq!(part1(&machines), 200);
        }
        #[test]
        fn test_part_provided() {
            let input = "\
Button a: X+94, Y+34
Button b: X+22, Y+67
Prize: X=8400, Y=5400

Button a: X+26, Y+66
Button b: X+67, Y+21
Prize: X=12748, Y=12176

Button a: X+17, Y+86
Button b: X+84, Y+37
Prize: X=7870, Y=6450

Button a: X+69, Y+23
Button b: X+27, Y+71
Prize: X=18641, Y=10279";
            let (_, machines) = parse_input(input).unwrap();
            assert_eq!(part1(&machines), 480);
        }
    }
    mod part_2_test {
        use super::*;

        mod solve_machine_test {
            use super::*;

            #[test]
            fn test_solve_machine_simple() {
                // Simple case where exact matches are possible
                let machine = Machine {
                    a_dx: 2,
                    a_dy: 3,
                    b_dx: 1,
                    b_dy: 1,
                    target_x: 9,
                    target_y: 12,
                };

                let result = solve_machine(&machine);
                assert_eq!(result, Some(12)); // 3 presses of A (3 * 2 = 6, 3 * 3 = 9) : cost : 9
                                              // 3 presses of B (3 * 1 = 3, 3 * 1 = 3) : cost : 3 (total 12)
                                              //                         9          12 === target 
            }
            #[test]
            fn test_solve_machine_second() {
                // Button a: X+26, Y+66
                // Button b: X+67, Y+21
                // Prize: X=10000000012748, Y=10000000012176
                let machine = Machine {
                    a_dx: 26,
                    a_dy: 66,
                    b_dx: 67,
                    b_dy: 21,
                    target_x: 12748 + 10_000_000_000_000,
                    target_y: 12176 + 10_000_000_000_000,
                };

                let result = solve_machine(&machine);
                assert!(result.is_some()); // 3 presses of A (3 * 2 = 6, 3 * 3 = 9) : cost : 9
                                              // 3 presses of B (3 * 1 = 3, 3 * 1 = 3) : cost : 3 (total 12)
                                              //                         9          12 === target 
            }            #[test]
            fn test_solve_machine_fourth() {
            // Button a: X+69, Y+23
            // Button b: X+27, Y+71
            // Prize: X=18641, Y=10279
                let machine = Machine {
                    a_dx: 69,
                    a_dy: 23,
                    b_dx: 27,
                    b_dy: 71,
                    target_x: 18641 + 10_000_000_000_000,
                    target_y: 10279 + 10_000_000_000_000,
                };

                let result = solve_machine(&machine);
                assert!(result.is_some()); // 3 presses of A (3 * 2 = 6, 3 * 3 = 9) : cost : 9
                                              // 3 presses of B (3 * 1 = 3, 3 * 1 = 3) : cost : 3 (total 12)
                                              //                         9          12 === target 
            }

            #[test]
            fn test_solve_machine_no_solution() {
                // Case where no solution exists
                let machine = Machine {
                    a_dx: 2,
                    a_dy: 3,
                    b_dx: 4,
                    b_dy: 6,
                    target_x: 5,
                    target_y: 8,
                };

                let result = solve_machine(&machine);
                assert_eq!(result, None); // No solution possible
            }

            #[test]
            fn test_solve_machine_large_target() {
                // Large target values
                let machine = Machine {
                    a_dx: 94, 
                    a_dy: 34,
                    b_dx: 22,
                    b_dy: 67,
                    target_x: 9422000000, // 94 * 100_000_000 + 22 * 1_000_000
                    target_y: 3467000000, // 34 * 100_000_000 + 67 * 1_000_000
                };
                // cost is 100_000_000 * 3 + 1_000_000 = 300_000_000 + 1_000_000 = 301_000_000
                let result = solve_machine(&machine);
                assert_eq!(result, Some(301_000_000));
                println!("Cost: {:?}", result.unwrap());
            }
        }
        #[test]
        fn test_part2() {
            let input = "\
Button a: X+94, Y+34
Button b: X+22, Y+67
Prize: X=8400, Y=5400

Button a: X+26, Y+66
Button b: X+67, Y+21
Prize: X=12748, Y=12176

Button a: X+17, Y+86
Button b: X+84, Y+37
Prize: X=7870, Y=6450

Button a: X+69, Y+23
Button b: X+27, Y+71
Prize: X=18641, Y=10279";
            let (_, machines) = parse_input(input).unwrap();
            assert!(part2(&machines) > 0);
        }
    }
}
