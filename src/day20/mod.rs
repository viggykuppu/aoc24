use std::{
    collections::{HashMap, HashSet},
    usize,
};

use aocd::*;

#[aocd(2024, 20)]
pub fn one() {
    let input = input!();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = HashSet::new();
    let mut grid = HashMap::new();
    let mut nodes = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let i = i as i32;
            let j = j as i32;
            if c == '#' {
                grid.insert((i, j), '#');
                if i != 0 && j != 0 {
                    walls.insert((i, j));
                }
            } else {
                grid.insert((i, j), '.');
                nodes.insert(
                    (i, j),
                    Node {
                        position: (i, j),
                        distance_from_start: usize::MAX,
                    },
                );
            }
            if c == 'S' {
                start = (i, j);
                nodes.get_mut(&(i, j)).unwrap().distance_from_start = 0;
            }
            if c == 'E' {
                end = (i, j);
            }
        })
    });
    let path = traverse(&grid, start, end);
    let path_length = path.len() - 1;
    let target = path_length - 100;
    let mut fast_cheats = 0;
    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let start: &Node = &path[i];
            let end: &Node = &path[j];
            let distance = end.position.0.abs_diff(start.position.0)
                + end.position.1.abs_diff(start.position.1);
            if distance == 2 {
                let time = start.distance_from_start
                    + (path_length - end.distance_from_start)
                    + distance as usize;
                if time <= target {
                    fast_cheats += 1;
                }
            }
        }
    }
    submit!(1, fast_cheats);
}

#[aocd(2024, 20)]
pub fn two() {
    let input = input!();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut walls = HashSet::new();
    let mut grid = HashMap::new();
    let mut nodes = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let i = i as i32;
            let j = j as i32;
            if c == '#' {
                grid.insert((i, j), '#');
                if i != 0 && j != 0 {
                    walls.insert((i, j));
                }
            } else {
                grid.insert((i, j), '.');
                nodes.insert(
                    (i, j),
                    Node {
                        position: (i, j),
                        distance_from_start: usize::MAX,
                    },
                );
            }
            if c == 'S' {
                start = (i, j);
                nodes.get_mut(&(i, j)).unwrap().distance_from_start = 0;
            }
            if c == 'E' {
                end = (i, j);
            }
        })
    });
    let path = traverse(&grid, start, end);
    let path_length = path.len() - 1;
    let target = path_length - 100;
    let mut fast_cheats = 0;
    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let start: &Node = &path[i];
            let end: &Node = &path[j];
            let distance = end.position.0.abs_diff(start.position.0)
                + end.position.1.abs_diff(start.position.1);
            if distance <= 20 {
                let time = start.distance_from_start
                    + (path_length - end.distance_from_start)
                    + distance as usize;
                if time <= target {
                    fast_cheats += 1;
                }
            }
        }
    }
    submit!(2, fast_cheats);
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    position: (i32, i32),
    distance_from_start: usize,
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

fn traverse(grid: &HashMap<(i32, i32), char>, start: (i32, i32), end: (i32, i32)) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut visited = HashSet::new();
    let mut current = Node {
        position: start,
        distance_from_start: 0,
    };
    nodes.push(current.clone());
    visited.insert(current.position);
    while current.position != end {
        for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
            let new_position = (current.position.0 + v.0, current.position.1 + v.1);
            if Some(&'.') == grid.get(&new_position) && !visited.contains(&new_position) {
                current = Node {
                    position: new_position,
                    distance_from_start: current.distance_from_start + 1,
                };
                break;
            }
        }
        visited.insert(current.position);
        nodes.push(current.clone());
    }
    nodes
}
