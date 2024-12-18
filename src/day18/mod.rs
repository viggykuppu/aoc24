use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

use aocd::*;

#[aocd(2024, 18)]
pub fn one() {
    let input = input!();
    let bytes = input.lines().map(|line| {
        let split: Vec<_> = line.split(',').collect();
        let x = split[0].parse::<i32>().unwrap();
        let y = split[1].parse::<i32>().unwrap();
        (x,y)
    });
    let dimension = 70;
    let num_fallen_bytes = 1024;
    let mut grid = HashMap::new();
    for i in 0..=dimension {
        for j in 0..=dimension {
            grid.insert((i,j), '.');
        }
    }
    bytes.take(num_fallen_bytes).for_each(|byte| {
        grid.insert((byte.0, byte.1), '#');
    });
    let goal = (dimension, dimension);
    let (distance_to_goal, came_from) = a_star(&grid, goal);
    let mut path = HashSet::new();
    path.insert(goal);
    let mut c = goal;
    loop {
        let came_from_position = came_from.get(&c).unwrap();
        c = *came_from_position;
        path.insert(*came_from_position);
        if *came_from_position == (0,0) {
            break;
        }
    }
    submit!(1, distance_to_goal);
}

#[aocd(2024, 18)]
pub fn two() {
    let input = input!();
    let bytes: Vec<_> = input.lines().map(|line| {
        let split: Vec<_> = line.split(',').collect();
        let x = split[0].parse::<i32>().unwrap();
        let y = split[1].parse::<i32>().unwrap();
        (x,y)
    }).collect();
    let dimension = 70;
    let mut num_fallen_bytes = 1024;
    let mut grid = HashMap::new();
    for i in 0..=dimension {
        for j in 0..=dimension {
            grid.insert((i,j), '.');
        }
    }
    bytes.iter().take(num_fallen_bytes).for_each(|byte| {
        grid.insert((byte.0, byte.1), '#');
    });
    let goal = (dimension, dimension);
    let mut breaking_byte = (0,0);
    loop {
        let (distance_to_goal, came_from) = a_star(&grid, goal);
        if distance_to_goal == 0 {
            break;
        }
        let mut path = HashSet::new();
        path.insert(goal);
        let mut c = goal;
        loop {
            let came_from_position = came_from.get(&c).unwrap();
            c = *came_from_position;
            path.insert(*came_from_position);
            if *came_from_position == (0,0) {
                break;
            }
        }
        loop {
            let byte = bytes[num_fallen_bytes];
            num_fallen_bytes += 1;
            grid.insert(byte, '#');
            if path.contains(&byte) {
                breaking_byte = byte;
                break;
            }
        }
    }
    let (x, y) = breaking_byte;
    let answer = format!("{x},{y}");
    submit!(2, answer);
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    position: (i32, i32),
    distance: usize,
    h: i32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.distance as i32 + self.h).cmp(&(other.distance as i32 + other.h))
    }
}

fn a_star(grid: &HashMap<(i32, i32), char>, goal: (i32, i32)) -> (usize, HashMap<(i32, i32), (i32, i32)>) {
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Node {
        position: (0,0),
        distance: 0,
        h: goal.0 + goal.1,
    }));
    let mut visited = HashSet::new();
    let mut distance_to_goal = 0; 
    let mut came_from = HashMap::new();
    while let Some(Reverse(current)) = to_visit.pop() {
        if current.position == goal {
            distance_to_goal = current.distance;
            break;
        }
        visited.insert(current.position);
        for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
            let new_position = (current.position.0 + v.0, current.position.1 + v.1);
            if Some(&'.') == grid.get(&new_position) && !visited.contains(&new_position) {
                came_from.insert(new_position, current.position);
                to_visit.push(Reverse(Node {
                    position: new_position,
                    distance: current.distance + 1,
                    h: (goal.0 - new_position.0) + (goal.1 - new_position.1),
                }));
            }
        }
    }
    (distance_to_goal, came_from)
}