use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2024, 11)]
pub fn one() {
    let input = input!();
    let stones: Vec<_> = input.split(" ").map(|stone| stone.parse::<u64>().unwrap()).collect();
    let mut memo = HashMap::new();
    let mut total_stones = 0;
    for stone in stones {
        total_stones += blink_stone(&stone, 25, &mut memo);
    }
    submit!(1, total_stones);
}

#[aocd(2024, 11)]
pub fn two() {
    let input = input!();
    let stones: Vec<_> = input.split(" ").map(|stone| stone.parse::<u64>().unwrap()).collect();
    let mut memo = HashMap::new();
    let mut total_stones = 0;
    for stone in stones {
        total_stones += blink_stone(&stone, 75, &mut memo);
    }
    submit!(2, total_stones);
}

fn blink_stone(stone: &u64, n: usize, memo: &mut HashMap::<(u64, usize), usize>) -> usize {
    if let Some(v) = memo.get(&(*stone, n)) {
        return *v;
    }
    if n == 0 {
        return 1;
    }
    let mut total_stones = 0;
    let new_stones = process_blink(stone);
    for stone in new_stones {
        total_stones += blink_stone(&stone, n-1, memo);
    }
    memo.insert((*stone, n), total_stones);
    total_stones
}

fn process_blink(stone: &u64) -> Vec<u64> {
    let string_stone = stone.to_string();
    if *stone == 0 {
        return vec![1];
    } else if string_stone.len() % 2 == 0 {
        let (stone_one, stone_two) = string_stone.split_at(string_stone.len()/2);
        return vec![stone_one.parse::<u64>().unwrap(), stone_two.parse::<u64>().unwrap()];
    }
    return vec![stone*2024];
}