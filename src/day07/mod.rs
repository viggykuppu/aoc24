use aocd::*;
use regex::Regex;
use rayon::prelude::*;
use itertools::Itertools;

#[aocd(2024, 7)]
pub fn one() {
    let input = input!();
    let line_regex = Regex::new(r"(\d+): (.*)").unwrap();
    let operators = ['+', '*'];
    let total_valid_lines: u64 = input.lines().collect::<Vec<_>>().par_iter().map(|line| {
        let caps: Vec<_> = line_regex.captures_iter(line).collect();
        let total = caps.first().unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap();
        let nums: Vec<_> = caps.first().unwrap().get(2).unwrap().as_str().split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
        if check_if_valid(total, &nums, &operators) {
            return total;
        }
        0
    }).sum();
    submit!(1, total_valid_lines);
}

#[aocd(2024, 7)]
pub fn two() {
    let input = input!();
    let line_regex = Regex::new(r"(\d+): (.*)").unwrap();
    let old_operators = ['+', '*'];
    let operators = ['+', '*', '|'];
    let total_valid_lines: u64 = input.lines().collect::<Vec<_>>().par_iter().map(|line| {
        let caps: Vec<_> = line_regex.captures_iter(line).collect();
        let total = caps.first().unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap();
        let nums: Vec<_> = caps.first().unwrap().get(2).unwrap().as_str().split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
        if check_if_valid(total, &nums, &old_operators) || check_if_valid(total, &nums, &operators) {
            return total;
        }
        0
    }).sum();
    submit!(2, total_valid_lines);
}

fn check_if_valid(total: u64, nums: &[u64], operators: &[char]) -> bool {
    let combos: Vec<_> = itertools::repeat_n(operators, nums.len()-1).multi_cartesian_product().collect();
    for combo in combos.iter() {
        let result = nums.iter().enumerate().fold(0, |acc, (i, next)| {
            if i == 0 {
                return *next;
            }
            let op = *combo[i-1];
            if op == '+' {
                return acc + next;
            } else if op == '*' {
                return acc * next;
            } else if op == '|' {
                let concatenated = acc.to_string() + &next.to_string();
                return concatenated.parse::<u64>().unwrap();
            }
            0
        });
        if result == total {
            return true;
        }
    }
    false
}