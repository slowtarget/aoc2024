extern crate core;

use std::time::Instant;
use std::fs;
use timing_util::measure_time;

mod puzzles;
use crate::puzzles::*;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => panic!("Please provide a day number"),
        _ => {
            let day = args[1].as_str();
            let test = args.get(2).map(|x| x.as_str()) == Some("test");
            let input =
                fs::read_to_string(format!("input/day{}.txt", day)).expect("Could not read file");
            match day {
                "1" => {
                    if test {
                        day1::test()
                    } else {
                        day1::solve(input)
                    }
                }
                "2" => {
                    if test {
                        day2::test()
                    } else {
                        day2::solve(input)
                    }
                }
                "3" => {
                    if test {
                        day3::test()
                    } else {
                        day3::solve(input)
                    }
                }
                "4" => {
                    day4::solve(input);
                }
                "5" => {
                    day5::solve(input);
                }
                "6" => {
                    day6::solve(input);
                }
                "7" => {
                    day7::solve(input);
                }
                "8" => {
                    day8::solve(input);
                }
                "9" => {
                    day9::solve(input);
                }
                "10" => {
                    day10::solve(input);
                }
                "11" => {
                    day11::solve(input);
                }
                "12" => {
                    println!("{:?}", day12::solve(&*input));
                }
                "13" => {
                    println!("{:?}", day13::solve(input));
                }
                "14" => {
                    println!("{:?}", measure_time!(day14::solve(input)));
                }
                "15" => {
                    println!("{:?}", measure_time!(day15::solve(input)));
                }
                "16" => {
                    println!("{:?}", measure_time!(day16::solve(input)));
                }
                "17" => {
                    println!("{:?}", measure_time!(day17::solve(input)));
                }
                "18" => {
                    println!("{:?}", measure_time!(day18::solve(input)));
                }
                "20" => {
                    println!("{:?}", measure_time!(day20::solve(input)));
                }
                "21" => {
                    println!("{:?}", measure_time!(day21::solve(input)));
                }
                "22" => {
                    println!("{:?}", measure_time!(day22::solve(input)));
                }
                "23" => {
                    println!("{:?}", measure_time!(day23::solve(input)));
                }
                "24" => {
                    println!("{:?}", measure_time!(day24::solve(input)));
                }
                _ => panic!("Day {} not found", day),
            }
        }
    }
}
