use std::{collections::{HashMap, HashSet}, usize};

use aocd::*;
use itertools::Itertools;

#[aocd(2024, 22)]
pub fn one() {
    let input = input!();
    let sum: usize = input
        .lines()
        .map(|line| {
            let mut secret = line.parse::<i64>().unwrap();
            for _ in 0..2000 {
                secret = prune(mix(secret, secret * 64));
                secret = prune(mix(secret, secret / 32));
                secret = prune(mix(secret, secret * 2048));
            }
            secret as usize
        })
        .sum();
    submit!(1, sum);
}

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

#[aocd(2024, 22)]
pub fn two() {
    let input = input!();
    let mut delta_maxes = HashMap::new();
    let mut max_value = 0;
    input.lines().enumerate().for_each(|(id, line)| {
        let id = id as isize;
        let mut secret = line.parse::<i64>().unwrap();
        let mut previous = 0;
        let mut hash: u32 = 0;
        for i in 0..2000 {
            secret = prune(mix(secret, secret * 64));
            secret = prune(mix(secret, secret / 32));
            secret = prune(mix(secret, secret * 2048));
            let price = (secret % 10) as i8;
            let delta = price - previous;
            hash = (hash << 8) | (delta as u32 & 0xff);
            if i > 3 {
                if price > 0 {
                    let (last_inserted_id, max) = delta_maxes.entry(hash).or_insert((-1, 0));
                    if *last_inserted_id != id {
                        *max += price as u32;
                        *last_inserted_id = id;
                        max_value = max_value.max(*max);
                    }
                }
            }
            previous = price;
        }
    });
    submit!(2, max_value);
}
