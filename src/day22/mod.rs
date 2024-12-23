use std::{collections::HashMap, usize};

use aocd::*;
use itertools::Itertools;

#[aocd(2024, 22)]
pub fn one() {
    let input = input!();
    let sum: usize = input.lines().map(|line| {
        let mut secret = line.parse::<i64>().unwrap();
        for _ in 0..2000 {
            secret = prune(mix(secret, secret*64));
            secret = prune(mix(secret, secret/32));
            secret = prune(mix(secret, secret*2048));
        }
        secret as usize
    }).sum();
    submit!(1, sum);
}

fn mix(a: i64, b: i64) -> i64 {
    a^b
}

fn prune(a: i64) -> i64 {
    a%16777216
}

#[aocd(2024, 22)]
pub fn two() {
    let input = input!();
    let mut delta_maxes = HashMap::new();
   input.lines().for_each(|line| {
        let mut secret = line.parse::<i64>().unwrap();
        let mut secrets = Vec::new();
        for _ in 0..2000 {
            secret = prune(mix(secret, secret*64));
            secret = prune(mix(secret, secret/32));
            secret = prune(mix(secret, secret*2048));
            secrets.push(secret%10);
        }
        let deltas: Vec<_> = secrets.windows(2).map(|a| {
            a[1]-a[0]
        }).collect();
        let mut maxes = HashMap::new();
        deltas.windows(4).enumerate().for_each(|(i, delta)| {
            let delta_string = String::from_iter(delta.iter().map(|d| d.to_string()));
            let price = secrets[i+4];
            if maxes.get(&delta_string).is_none() {
                maxes.insert(delta_string, price);
            }
        });
        for (key, value) in maxes.iter() {
            let max = delta_maxes.entry(key.clone()).or_insert(0);
            *max += value;
        }
    });
    let mut max_value = 0;
    for v in delta_maxes.values() {
        if *v > max_value {
            max_value = *v;
        }
    }
    submit!(2, max_value);
}