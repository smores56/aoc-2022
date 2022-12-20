#![feature(
    slice_group_by,
    iter_array_chunks,
    array_chunks,
    hash_drain_filter,
    array_windows
)]

use std::env;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day18;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod util;

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
        1 => Box::new(day1::Day1::from_lines(lines)),
        2 => Box::new(day2::Day2::from_lines(lines)),
        3 => Box::new(day3::Day3::from_lines(lines)),
        4 => Box::new(day4::Day4::from_lines(lines)),
        5 => Box::new(day5::Day5::from_lines(lines)),
        6 => Box::new(day6::Day6::from_lines(lines)),
        7 => Box::new(day7::Day7::from_lines(lines)),
        8 => Box::new(day8::Day8::from_lines(lines)),
        9 => Box::new(day9::Day9::from_lines(lines)),
        10 => Box::new(day10::Day10::from_lines(lines)),
        11 => Box::new(day11::Day11::from_lines(lines)),
        12 => Box::new(day12::Day12::from_lines(lines)),
        13 => Box::new(day13::Day13::from_lines(lines)),
        14 => Box::new(day14::Day14::from_lines(lines)),
        15 => Box::new(day15::Day15::from_lines(lines)),
        18 => Box::new(day18::Day18::from_lines(lines)),
        20 => Box::new(day20::Day20::from_lines(lines)),
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
