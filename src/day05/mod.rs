use std::{cmp::Ordering, collections::{HashMap, HashSet}};
use aocd::*;
use regex::Regex;

#[aocd(2024, 5)]
pub fn one() {
    let input = input!();
    let after_map = parse_rules(&input);
    let sum: u32 = input.lines().filter(|line| line.contains(',') && is_valid_page(line, &after_map)).map(|line| {
        let nums: Vec<u32> = line.split(',').map(|d| d.parse::<u32>().unwrap()).collect();
        let middle_num = nums.get(nums.len()/2).unwrap();
        *middle_num
    }).sum();
    submit!(1, sum);
}

#[aocd(2024, 5)]
pub fn two() {
    let input = input!();
    let after_map = parse_rules(&input);
    let sum: u32 = input.lines().filter(|line| line.contains(',') && !is_valid_page(line, &after_map)).map(|line| {
        let nums: Vec<u32> = line.split(',').map(|d| d.parse::<u32>().unwrap()).collect();
        let mut bad_page = nums.clone();
        bad_page.sort_by(|a, b| {
            if after_map.contains_key(a) && after_map[a].contains(b) {
                return Ordering::Less;
            } else if after_map.contains_key(b) && after_map[b].contains(a) {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        println!("{bad_page:?}");
        *bad_page.get(bad_page.len()/2).unwrap()
    }).sum();

    submit!(2, sum);
}

fn parse_rules(input: &str) -> HashMap<u32, HashSet<u32>> {
    let mut after_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let ordering_regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    input.lines().for_each(|line| {
        if line.contains('|') {
            let caps = ordering_regex.captures(line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            after_map.entry(x).or_default().insert(y);
        }
    });
    after_map
}

fn is_valid_page(line: &str, after_map: &HashMap<u32,HashSet<u32>>) -> bool {
    if !line.contains(',') {
        return false;
    }
    let nums: Vec<u32> = line.split(',').map(|d| d.parse::<u32>().unwrap()).collect();
    let mut seen_nums:HashSet<u32> = HashSet::new();
    for num in nums.iter() {
        let valid = seen_nums.iter().all(|seen_num| {
            if after_map.contains_key(num) {
                !after_map.get(num).unwrap().contains(seen_num)
            } else {
                true
            }
        });
        if !valid {
            return false;
        }
        seen_nums.insert(*num);
    }
    true
}