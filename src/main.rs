#![allow(dead_code)]

use std::time::Instant;


mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;
mod eleven;

fn main() {
    let mut now = Instant::now();
    eleven::one();
    let part_one_time = now.elapsed().as_millis();
    println!("Part 1 solved in {part_one_time} ms");
    now = Instant::now();
    eleven::two();
    let part_two_time = now.elapsed().as_millis();
    println!("Part 2 solved in {part_two_time} ms");
}
