use std::fs;
mod puzzles;
use crate::puzzles::*;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => panic!("Please provide a day number"),
        _ => {
            let day = args[1].as_str();
            let test = args.get(2).map(|x| x.as_str()) == Some("test");
            let input = fs::read_to_string(format!("input/day{}.txt", day)).expect("Could not read file");
            match day {
                "1" => if test {day1::test()} else {  day1::solve(input)},
                "2" => if test {day2::test()} else {  day2::solve(input)},
                "3" => if test {day3::test()} else {  day3::solve(input)},
                "4" => {day4::solve(input);},
                "5" => {day5::solve(input);},
                "6" => {day6::solve(input);},
                "7" => {day7::solve(input);},
                "8" => {day8::solve(input);},
                "9" => {day9::solve(input);},
                "10" => {day10::solve(input);},
                _ => panic!("Day {} not found", day),
            }
        }
    }
}