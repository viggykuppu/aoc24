use std::{cmp::Ordering, collections::{HashMap, HashSet}};
use aocd::*;
use regex::Regex;

#[aocd(2024, 5)]
pub fn one() {
    let input = input!();
    let mut after_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let ordering_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let sum: u32 = input.lines().enumerate().map(|(i, line)| {
        // build ordering
        if line.contains('|') {
            let caps = ordering_regex.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            if !after_map.contains_key(&x) {
                after_map.insert(x, HashSet::new());
            }
            after_map.get_mut(&x).unwrap().insert(y);
        } else if line.contains(',') {
            let nums: Vec<u32> = line.split(',').map(|d| d.parse::<u32>().unwrap()).collect();
            let mut seen_nums:HashSet<u32> = HashSet::new();
            let mut valid = true;
            for num in nums.iter() {
                valid = seen_nums.iter().all(|seen_num| {
                    if after_map.contains_key(&num) {
                        !after_map.get(&num).unwrap().contains(seen_num)
                    } else {
                        true
                    }
                });
                if !valid {
                    break;
                }
                seen_nums.insert(*num);
            }
            if valid {
                let middle_num = nums.get(nums.len()/2).unwrap();
                return *middle_num;
            }
        }
        return 0;
    }).sum();

    submit!(1, sum);
}

#[aocd(2024, 5)]
pub fn two() {
    let input = input!();
    let mut after_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let ordering_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let sum: u32 = input.lines().enumerate().map(|(i, line)| {
        if line.contains('|') {
            let caps = ordering_regex.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            if !after_map.contains_key(&x) {
                after_map.insert(x, HashSet::new());
            }
            after_map.get_mut(&x).unwrap().insert(y);
        } else if line.contains(',') {
            let nums: Vec<u32> = line.split(',').map(|d| d.parse::<u32>().unwrap()).collect();
            let mut seen_nums:HashSet<u32> = HashSet::new();

            for num in nums.iter() {
                let valid = seen_nums.iter().all(|seen_num| {
                    if after_map.contains_key(&num) {
                        !after_map.get(&num).unwrap().contains(seen_num)
                    } else {
                        true
                    }
                });
                if !valid {
                    let mut bad_page = nums.clone();
                    bad_page.sort_by(|a, b| {
                        if after_map.contains_key(a) {
                            if after_map.get(a).unwrap().contains(b) {
                                return Ordering::Less;
                            }
                        } else if after_map.contains_key(b) {
                            if after_map.get(b).unwrap().contains(a) {
                                return Ordering::Greater;
                            }
                        }
                        return Ordering::Equal;
                    });
                    return *bad_page.get(bad_page.len()/2).unwrap();
                }
                seen_nums.insert(*num);
            }
        }
        0
    }).sum();

    submit!(2, sum);
}