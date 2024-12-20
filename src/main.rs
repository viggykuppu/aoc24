#![allow(dead_code)]

use std::time::Instant;
use aoc24::*;

fn main() {
    let mut now = Instant::now();
    day20::one();
    let part_one_time = now.elapsed().as_millis();
    println!("Part 1 solved in {part_one_time} ms");
    now = Instant::now();
    day20::two();
    let part_two_time = now.elapsed().as_millis();
    println!("Part 2 solved in {part_two_time} ms");
}
