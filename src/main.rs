#![feature(slice_group_by)]

use std::env;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

use day1::Day1;
use day2::Day2;

mod day1;
mod day2;

/// Reads the lines from the input file into a relevant
/// model of the data for the day's solution.
trait FromInput {
    fn from_lines(lines: impl Iterator<Item = String>) -> Self;
}

/// Solutions for a day of Advent of Code.
trait DaySolution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

/// Reads the input for a day from the `.input` directory.
fn load_input(day: usize) -> impl Iterator<Item = String> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .open(format!(".input/{day}.txt"))
        .expect("Failed to access data for day");
    let buffered_file = BufReader::new(file);

    buffered_file
        .lines()
        .map(|line| line.expect("Failed to read line from data file"))
}

/// Gets the solution for the given day as a trait object.
fn get_day_solution(day: usize, lines: impl Iterator<Item = String>) -> Box<dyn DaySolution> {
    match day {
        1 => Box::new(Day1::from_lines(lines)),
        2 => Box::new(Day2::from_lines(lines)),
        _other => panic!("Day hasn't been solved yet"),
    }
}

/// Times the execution of a function.
fn time_execution(work: impl FnOnce() -> String) -> (String, Duration) {
    let start = Instant::now();
    let result = work();
    let duration = start.elapsed();

    (result, duration)
}

fn main() {
    let day = env::args()
        .nth(1)
        .expect("Must provide a day to solve")
        .parse::<usize>()
        .expect("Provided day wasn't a valid integer");

    let input = load_input(day);
    let solution = get_day_solution(day, input);
    println!("Solving day {day}...");

    let (part_one, duration) = time_execution(|| solution.part_one());
    println!("Part 1: {part_one} ({} seconds)", duration.as_secs_f32());

    let (part_two, duration) = time_execution(|| solution.part_two());
    println!("Part 2: {part_two} ({} seconds)", duration.as_secs_f32());
}
