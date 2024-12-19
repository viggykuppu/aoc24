use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

use aocd::*;

#[aocd(2024, 19)]
pub fn one() {
    let input = input!();
    let mut towels = HashSet::new();
    let mut designs = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        if i == 0 {
            towels = HashSet::<&str>::from_iter(line.split(',').map(|s| s.trim()));
        }
        if i > 1 {
            designs.push(line);
        }
    });
    let mut memo = HashMap::new();
    let num_possible_towels: usize = designs.iter().map(|design| {
        let result = foo(&towels, design, &mut memo);
        result as usize
    }).sum();
    submit!(1, num_possible_towels);
}

fn foo<'a>(towels: &HashSet<&str>, design: &'a str, memo: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(v) = memo.get(design) {
        return *v;
    }
    if design.len() == 0 {
        return true;
    }
    for i in 1..=design.len() {
        let possible_towel = &design[0..i];
        if towels.contains(possible_towel) {
            let result = foo(towels, &design[i..], memo);
            if result {
                memo.insert(design, true);
                return true;
            }
        }
    }
    memo.insert(design, false);
    return false;
}

#[aocd(2024, 19)]
pub fn two() {
    let input = input!();
    let mut towels = HashSet::new();
    let mut designs = Vec::new();
    input.lines().enumerate().for_each(|(i, line)| {
        if i == 0 {
            towels = HashSet::<&str>::from_iter(line.split(',').map(|s| s.trim()));
        }
        if i > 1 {
            designs.push(line);
        }
    });
    let mut memo = HashMap::new();
    let num_possible_towels: usize = designs.iter().map(|design| {
        let result = bar(&towels, design, &mut memo);
        result as usize
    }).sum();
    submit!(2, num_possible_towels);
}

fn bar<'a>(towels: &HashSet<&str>, design: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(v) = memo.get(design) {
        return *v;
    }
    if design.len() == 0 {
        return 1;
    }
    let mut num_possible_towels = 0;
    for i in 1..=design.len() {
        let possible_towel = &design[0..i];
        if towels.contains(possible_towel) {
            let result = bar(towels, &design[i..], memo);
            num_possible_towels += result;
        }
    }
    memo.insert(design, num_possible_towels);
    return num_possible_towels;
}