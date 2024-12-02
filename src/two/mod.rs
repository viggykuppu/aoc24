use regex::Regex;
use std::collections::HashMap;
use std::iter::zip;

use aocd::*;

#[aocd(2024, 2)]
pub fn one() {
    let input = input!();
    let number_regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    
    let num_safe_reports: u32 = input.lines().enumerate().map(|(i, line)| {
        let digits: Vec<i32> = line.split(" ").map(|d| d.parse::<i32>().unwrap()).collect();
        
        if big_safe_check(&digits) {
            1
        } else {
            0
        }
    }).sum();

    
    submit!(1, num_safe_reports);
}

#[aocd(2024, 2)]
pub fn two() {
    let input = input!();
    let number_regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    
    let num_safe_reports: u32 = input.lines().enumerate().map(|(i, line)| {
        let digits: Vec<i32> = line.split(" ").map(|d| d.parse::<i32>().unwrap()).collect();
        
        let mut safe = big_safe_check(&digits);
        if !safe {
            for j in 0..digits.len() {
                let mut digits_copy = digits.clone();
                digits_copy.remove(j);
                safe = big_safe_check(&digits_copy);
                if safe {
                    break;
                }
            }
        }
        
        if safe {
            1
        } else {
            0
        }
    }).sum();

    
    submit!(2, num_safe_reports);
}

fn big_safe_check(nums: &Vec<i32>) -> bool {
    let sign = if nums.get(0).unwrap() > nums.get(1).unwrap() {
        -1
    } else {
        1
    };
    for i in 0..(nums.len()-1) {
        let current = nums.get(i).unwrap();
        let next = nums.get(i+1).unwrap();
        if !safe_check(current, next, &sign) {
            return false;
        }
    }
    return true;
}

fn safe_check(current: &i32, next: &i32, sign: &i32) -> bool {
    let difference = next - current;
    let difference_magnitude = difference.abs();
    if difference_magnitude == 0 || difference_magnitude > 3 {
        return false;
    }
    if (difference/difference.abs()) + sign == 0 {
        return false;
    }
    true
}