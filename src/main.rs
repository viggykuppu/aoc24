use aoc24::*;
use std::time::Instant;

fn main() {
    let mut now = Instant::now();
    day23::one();
    let part_one_time = now.elapsed().as_millis();
    println!("Part 1 solved in {part_one_time} ms");
    now = Instant::now();
    day23::two();
    let part_two_time = now.elapsed().as_millis();
    println!("Part 2 solved in {part_two_time} ms");
}
