#![allow(dead_code)]

use std::time::Instant;


mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    let mut now = Instant::now();
    day12::one();
    let part_one_time = now.elapsed().as_millis();
    println!("Part 1 solved in {part_one_time} ms");
    now = Instant::now();
    day12::two();
    let part_two_time = now.elapsed().as_millis();
    println!("Part 2 solved in {part_two_time} ms");
}
