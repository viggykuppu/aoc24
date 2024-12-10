use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2024, 10)]
pub fn one() {
    let input = input!();
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(11)).collect()).collect();
    let total_trailheads: u32 = map.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, cell)| {
            if *cell == 0 {
                let z = get_trailhead_score(&map, (i,j), &mut HashSet::new(), &mut HashMap::new(), false);
                // return get_trailhead_score(&map, (i,j), &mut HashSet::new(), &mut memo, false);
                return z;
            }
            0
        }).sum::<u32>()
    }).sum();
    submit!(1, total_trailheads);
}

#[aocd(2024, 10)]
pub fn two() {
    let input = input!();
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap_or(11)).collect()).collect();
    let total_trailheads: u32 = map.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, cell)| {
            if *cell == 0 {
                return get_trailhead_score(&map, (i,j), &mut HashSet::new(), &mut HashMap::new(), true)
            }
            0
        }).sum::<u32>()
    }).sum();
    submit!(2, total_trailheads);
}

fn get_trailhead_score(map: &Vec<Vec<u32>>, location: (usize, usize), visited: &mut HashSet<(usize, usize)>, memo: &mut HashMap<(usize, usize), u32>, get_rating: bool) -> u32 {
    if let Some(score) = memo.get(&location) {
        println!("using memo");
        return *score;
    }
    let current_elevation = map[location.0][location.1];
    let mut total_trailheads = 0;
    for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
        let new_location = (location.0 as isize + v.0, location.1 as isize + v.1);
        if (0..(map.len() as isize)).contains(&new_location.0) && (0..(map[0].len() as isize)).contains(&new_location.1) {
            let new_location = (new_location.0 as usize, new_location.1 as usize);
            let new_elevation = map[new_location.0][new_location.1];
            if new_elevation.saturating_sub(current_elevation) == 1 {
                if !visited.contains(&new_location) {
                    visited.insert(new_location);
                    let score = if new_elevation == 9 {
                        // println!("found end at {new_location:?}");
                        1
                    } else {
                        if get_rating {
                            get_trailhead_score(map, new_location, &mut visited.clone(), memo, get_rating)
                        } else {
                            get_trailhead_score(map, new_location, visited, memo, get_rating)
                        }
                    };
                    memo.insert(new_location, score);
                    total_trailheads += score;
                }
            }
        }
    }
    
    total_trailheads
}
