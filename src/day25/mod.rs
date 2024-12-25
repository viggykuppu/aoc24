use std::collections::{HashMap, HashSet};

use aocd::*;
use regex::Regex;

#[aocd(2024, 25)]
pub fn one() {
    let input = input!();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    input.split("\n\n").for_each(|chunk| {
        let is_lock = chunk.starts_with('#');
        let mut values = HashMap::new();
        chunk.lines().enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                if c == '#' {
                    let v = values.entry(j).or_insert(-1);
                    *v += 1;
                }
            });
        });
        if is_lock {
            locks.push(values);
        } else {
            keys.push(values);
        }
    });
    let mut num_lock_key_combos = 0; 
    for lock in locks.iter() {
        for key in keys.iter() {
            for i in 0..5 {
                if lock.get(&i).unwrap() + key.get(&i).unwrap() >= 6 {
                    break;
                }
                if i == 4 {
                    num_lock_key_combos += 1;
                }
            }
        }
    }
    
    submit!(1, num_lock_key_combos);
}

#[aocd(2024, 25)]
pub fn two() {
    submit!(2, 0);
}