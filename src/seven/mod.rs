use std::{collections::{HashMap, HashSet}};
use aocd::*;
use radix_fmt::radix_3;
use regex::Regex;
use rayon::prelude::*;

#[aocd(2024, 7)]
pub fn one() {
    let input = input!();
    let line_regex = Regex::new(r"(\d+): (.*)").unwrap();
    let total_valid_lines: u64 = input.lines().collect::<Vec<_>>().par_iter().map(|line| {
        let caps: Vec<_> = line_regex.captures_iter(line).collect();
        let total = caps.get(0).unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap();
        let nums: Vec<_> = caps.get(0).unwrap().get(2).unwrap().as_str().split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
        let total_combos = 2_u32.pow(nums.len() as u32 - 1);
        for i in 0..total_combos {
            let result = nums.iter().enumerate().fold(0, |acc, (j, next)| {
                if j == 0 {
                    return acc + next;
                }
                let operation_bit = (i >> (j-1)) & 1;
                if operation_bit == 0 {
                    return acc + next;
                } else {
                    return acc * next;
                }
            });
            if result == total {
                return total;
            }
        }
        return 0;
    }).sum();
    submit!(1, total_valid_lines);
}

fn is_valid(total: u64, nums: Vec<u64>, ops: &[char]) -> u64 {
    let mut t = 0;
    for op in ops {
        if op == &'+' {
            t = total + nums[0];
        } else if op == &'*' {
            t = total * nums[0];
        }
    }
    return t;
}

#[aocd(2024, 7)]
pub fn two() {
    let input = input!();
    let line_regex = Regex::new(r"(\d+): (.*)").unwrap();
    let total_valid_lines = input.lines().collect::<Vec<_>>().par_iter().enumerate().map(|(idx, line)| {
        // if !initial_valid_lines.contains(&idx) {
            let caps: Vec<_> = line_regex.captures_iter(line).collect();
            let total = caps.get(0).unwrap().get(1).unwrap().as_str().parse::<u64>().unwrap();
            let nums: Vec<_> = caps.get(0).unwrap().get(2).unwrap().as_str().split(' ').map(|n| n.parse::<u64>().unwrap()).collect();
            let total_combos = 3_u32.pow(nums.len() as u32 - 1);
            for i in 0..total_combos {
                let result = nums.iter().enumerate().fold(0, |acc, (j, next)| {
                    if j == 0 {
                        return acc + next;
                    }
                    let mut i_base_3: Vec<_> = radix_3(i).to_string().chars().collect();
                    i_base_3.reverse();
                    let operation_bit = i_base_3.get((j-1) as usize).unwrap_or(&'0');
                    if operation_bit == &'0' {
                        return acc + next;
                    } else if operation_bit == &'1' {
                        return acc * next;
                    } else {
                        let concatenated = acc.to_string() + &next.to_string();
                        return concatenated.parse::<u64>().unwrap();
                    }
                });
                if result == total {
                    return total;
                }
            }
        // }
        return 0;
    }).sum::<u64>();
    submit!(2, total_valid_lines);
}