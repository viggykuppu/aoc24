use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, usize};

use aocd::*;
use nalgebra::base;

#[aocd(2024, 20, "src/day20/input.txt")]
pub fn one() {
    let input = input!();
    let mut start = (0,0);
    let mut end = (0,0);
    let mut walls = HashSet::new();
    let mut grid = HashMap::new();
    let mut nodes = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let i = i as i32;
            let j = j as i32;
            if c == '#' {
                grid.insert((i,j), '#');
                if i != 0 && j != 0 {
                    walls.insert((i,j));
                }
            } else {
                grid.insert((i,j), '.');
                nodes.insert((i,j), Node {
                    position: (i,j),
                    distance_from_end: usize::MAX,
                    distance_from_start: usize::MAX,
                });
            }
            if c == 'S' {
                start = (i,j);
                nodes.get_mut(&(i,j)).unwrap().distance_from_start = 0;
            }
            if c == 'E' {
                end = (i,j);
                nodes.get_mut(&(i,j)).unwrap().distance_from_end = 0;
            }
        })
    });
    let base_distance = 9432;
    let target = base_distance - 100;
    let mut potential_start_nodes = Vec::new();
    let mut to_visit = Vec::new();
    for node in nodes.values() {
        to_visit.push(node.position);
    }
    let mut visited = HashSet::new();
    visited.insert(start);
    potential_start_nodes.push(start);
    'search: while let Some(current) = get_min_from_start(&mut to_visit, &nodes) {
        let current_distance_from_start = nodes.get(&current).unwrap().distance_from_start;
        for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
            let new_position = (current.0 + v.0, current.1 + v.1);
            if visited.contains(&new_position) {
                continue;
            }
            if let Some(node) = nodes.get_mut(&new_position) {
                visited.insert(node.position);
                node.distance_from_start = current_distance_from_start + 1;
                if node.distance_from_start > target {
                    break 'search;
                }
                potential_start_nodes.push(node.position);
            }
        }
    }
    
    let mut potential_end_nodes = Vec::new();
    let mut to_visit = Vec::new();
    for node in nodes.values() {
        to_visit.push(node.position);
    }
    let mut visited = HashSet::new();
    visited.insert(end);
    potential_end_nodes.push(end);
    'search: while let Some(current) = get_min_from_end(&mut to_visit, &nodes) {
        let current_distance_from_end = nodes.get(&current).unwrap().distance_from_end;
        for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
            let new_position = (current.0 + v.0, current.1 + v.1);
            if visited.contains(&new_position) {
                continue;
            }
            if let Some(node) = nodes.get_mut(&new_position) {
                visited.insert(node.position);
                node.distance_from_end = current_distance_from_end + 1;
                if node.distance_from_end > target {
                    break 'search;
                }
                potential_end_nodes.push(node.position);
            }
        }
    }
    
    let mut fast_cheats = 0;
    for x in potential_start_nodes.iter() {
        for y in potential_end_nodes.iter() {
            if x.0.abs_diff(y.0) + x.1.abs_diff(y.1) == 2 {
                let start = nodes.get(&x).unwrap();
                let end = nodes.get(&y).unwrap();
                let total_time = start.distance_from_start + end.distance_from_end + 2;
                let saved_time = base_distance - total_time;
                if total_time <= target {
                    fast_cheats += 1;
                }
            }
        }
    }
    submit!(1, fast_cheats);
}

#[aocd(2024, 20, "src/day20/input.txt")]
pub fn two() {
    let input = input!();
    let mut start = (0,0);
    let mut end = (0,0);
    let mut walls = HashSet::new();
    let mut grid = HashMap::new();
    let mut nodes = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let i = i as i32;
            let j = j as i32;
            if c == '#' {
                grid.insert((i,j), '#');
                if i != 0 && j != 0 {
                    walls.insert((i,j));
                }
            } else {
                grid.insert((i,j), '.');
                nodes.insert((i,j), Node {
                    position: (i,j),
                    distance_from_end: usize::MAX,
                    distance_from_start: usize::MAX,
                });
            }
            if c == 'S' {
                start = (i,j);
                nodes.get_mut(&(i,j)).unwrap().distance_from_start = 0;
            }
            if c == 'E' {
                end = (i,j);
                nodes.get_mut(&(i,j)).unwrap().distance_from_end = 0;
            }
        })
    });
    let base_distance = 9432;
    let target = base_distance - 100;
    let mut potential_start_nodes = Vec::new();
    let mut to_visit = Vec::new();
    for node in nodes.values() {
        to_visit.push(node.position);
    }
    let mut visited = HashSet::new();
    visited.insert(start);
    potential_start_nodes.push(start);
    'search: while let Some(current) = get_min_from_start(&mut to_visit, &nodes) {
        let current_distance_from_start = nodes.get(&current).unwrap().distance_from_start;
        for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
            let new_position = (current.0 + v.0, current.1 + v.1);
            if visited.contains(&new_position) {
                continue;
            }
            if let Some(node) = nodes.get_mut(&new_position) {
                visited.insert(node.position);
                node.distance_from_start = current_distance_from_start + 1;
                if node.distance_from_start > target {
                    break 'search;
                }
                potential_start_nodes.push(node.position);
            }
        }
    }
    
    let mut potential_end_nodes = Vec::new();
    let mut to_visit = Vec::new();
    for node in nodes.values() {
        to_visit.push(node.position);
    }
    let mut visited = HashSet::new();
    visited.insert(end);
    potential_end_nodes.push(end);
    'search: while let Some(current) = get_min_from_end(&mut to_visit, &nodes) {
        let current_distance_from_end = nodes.get(&current).unwrap().distance_from_end;
        for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
            let new_position = (current.0 + v.0, current.1 + v.1);
            if visited.contains(&new_position) {
                continue;
            }
            if let Some(node) = nodes.get_mut(&new_position) {
                visited.insert(node.position);
                node.distance_from_end = current_distance_from_end + 1;
                if node.distance_from_end > target {
                    break 'search;
                }
                potential_end_nodes.push(node.position);
            }
        }
    }
    
    let mut fast_cheats = 0;
    let max_cheat_jump = 20;
    for x in potential_start_nodes.iter() {
        for y in potential_end_nodes.iter() {
            let distance = x.0.abs_diff(y.0) + x.1.abs_diff(y.1);
            if  distance <= max_cheat_jump {
                let start = nodes.get(&x).unwrap();
                let end = nodes.get(&y).unwrap();
                let total_time = start.distance_from_start + end.distance_from_end + distance as usize;
                let saved_time = base_distance - total_time;
                if total_time <= target {
                    // println!("{saved_time}: {start:?} {end:?}");
                    fast_cheats += 1;
                }
            }
        }
    }
    submit!(2, fast_cheats);
}

fn get_min_from_start(to_visit: &mut Vec<(i32, i32)>, nodes: &HashMap<(i32,i32), Node>) -> Option<(i32, i32)> {
    to_visit.sort_by(|a, b| {
        nodes[b].distance_from_start.cmp(&nodes[a].distance_from_start)
    });
    to_visit.pop()
}

fn get_min_from_end(to_visit: &mut Vec<(i32, i32)>, nodes: &HashMap<(i32,i32), Node>) -> Option<(i32, i32)> {
    to_visit.sort_by(|a, b| {
        nodes[b].distance_from_end.cmp(&nodes[a].distance_from_end)
    });
    to_visit.pop()
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    position: (i32, i32),
    distance_from_start: usize,
    distance_from_end: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.distance_from_start).cmp(&(other.distance_from_start))
    }
}

// fn traverse(grid: &HashMap<(i32, i32), char>, start: (i32, i32), goal: (i32, i32)) -> usize {
//     let mut to_visit = BinaryHeap::new();
//     to_visit.push(Reverse(Node {
//         position: start,
//         distance: 0,
//         h: (goal.0.abs_diff(start.0)) as i32 + (goal.1.abs_diff(start.1)) as i32,
//     }));
//     let mut visited = HashSet::new();
//     let mut distance_to_goal = 0; 
//     let mut came_from = HashMap::new();
//     while let Some(Reverse(current)) = to_visit.pop() {
//         if current.position == goal {
//             distance_to_goal = current.distance;
//             break;
//         }
//         visited.insert(current.position);
//         for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
//             let new_position = (current.position.0 + v.0, current.position.1 + v.1);
//             if Some(&'.') == grid.get(&new_position) && !visited.contains(&new_position) {
//                 came_from.insert(new_position, current.position);
//                 to_visit.push(Reverse(Node {
//                     position: new_position,
//                     distance: current.distance + 1,
//                     h: (goal.0.abs_diff(new_position.0)) as i32 + (goal.1.abs_diff(new_position.1)) as i32,
//                 }));
//             }
//         }
//     }
//     (distance_to_goal, came_from)
// }