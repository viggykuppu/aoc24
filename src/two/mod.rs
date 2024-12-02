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
        
        let sign = if digits.get(0).unwrap() > digits.get(1).unwrap() {
            -1
        } else {
            1
        };
        for i in 0..(digits.len()-1) {
            let current = digits.get(i).unwrap();
            let next = digits.get(i+1).unwrap();
            if !safe_check(current, next, &sign) {
                return 0;
            }
        }
        1
    }).sum();

    
    submit!(1, num_safe_reports);
}

#[aocd(2024, 2)]
pub fn two() {
    let input = input!();
    let number_regex = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    
    let num_safe_reports: u32 = input.lines().enumerate().map(|(i, line)| {
        let digits: Vec<i32> = line.split(" ").map(|d| d.parse::<i32>().unwrap()).collect();
        
        let mut sign = if digits.get(0).unwrap() > digits.get(1).unwrap() {
            -1
        } else {
            1
        };
        let mut failed = false;
        for i in 0..(digits.len()-1) {
            let current = digits.get(i).unwrap();
            let next = digits.get(i+1).unwrap();
            if !safe_check(current, next, &sign) {
                failed = true;
                break;
            }
        }
        if failed {
            for j in 0..digits.len() {
                failed = false;
                let mut digits_copy = digits.clone();
                digits_copy.remove(j);
                let l = digits_copy.len();
                sign = if digits_copy.get(0).unwrap() > digits_copy.get(1).unwrap() {
                    -1
                } else {
                    1
                };
                for i in 0..(digits_copy.len()-1) {
                    let current = digits_copy.get(i).unwrap();
                    let next = digits_copy.get(i+1).unwrap();
                    if !safe_check(current, next, &sign) {
                        failed = true;
                        break;
                    }
                }
                if !failed {
                    break;
                }
            }
        }
        
        if failed {
            0
        } else {
            1
        }
    }).sum();

    
    submit!(2, num_safe_reports);
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

fn safe_check_two(current: &i32, next: &i32, sign: &i32) -> bool {
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