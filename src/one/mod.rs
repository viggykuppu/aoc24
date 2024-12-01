use num::abs;
use regex::Regex;
use std::collections::HashMap;
use std::iter::zip;

use aocd::*;

#[aocd(2024, 1)]
pub fn one() {
    let input = input!();
    let number_regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut l1 = Vec::<i32>::new();
    let mut l2 = Vec::<i32>::new();

    input.lines().enumerate().for_each(|(i, line)| {
        let caps: Vec<_> = number_regex.captures_iter(line).collect();
        l1.push(caps.get(0).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap());
        l2.push(caps.get(0).unwrap().get(2).unwrap().as_str().parse::<i32>().unwrap());
    });

    l1.sort();
    l2.sort();

    let sum: u32 = zip(l1,l2).map(|(x,y)| {
        x.abs_diff(y)
    }).sum();
    submit!(1, sum);
}

#[aocd(2024, 1)]
pub fn two() {
    let input = input!();
    let number_regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut l1 = Vec::<i32>::new();
    let mut frequency_map = HashMap::<i32, i32>::new();
    input.lines().enumerate().for_each(|(i, line)| {
        let caps: Vec<_> = number_regex.captures_iter(line).collect();
        l1.push(caps.get(0).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap());
        let right = caps.get(0).unwrap().get(2).unwrap().as_str().parse::<i32>().unwrap();
        if let Some(value) = frequency_map.get(&right) {
            frequency_map.insert(right, value+1);
        } else {
            frequency_map.insert(right, 1);
        }
    });

    let similarity: i32 = l1.iter().map(|num| {
        let a = num;
        if let Some(value) = frequency_map.get(&a) {
            value*a
        } else {
            0
        }
    }).sum();
    
    submit!(2, similarity);
}
