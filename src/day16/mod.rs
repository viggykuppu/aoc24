use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

use aocd::*;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    position: (i32, i32),
    cost: u32,
    direction: Direction,
    path: HashSet<(i32, i32)>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[aocd(2024, 16)]
pub fn one() {
    let input = input!();
    let direction_map = HashMap::from([(Direction::Up, (-1, 0)), (Direction::Down, (1, 0)), (Direction::Left, (0, -1)), (Direction::Right, (0, 1))]);
    let mut start = (0,0);
    let mut exit = (0,0);
    let mut maze = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let i = i as i32;
            let j = j as i32;
            maze.insert((i,j), c);
            if c == 'S' {
                start = (i,j);
            } else if c == 'E' {
                exit = (i,j);
            }
        });
    });
    let mut visited: HashSet<((i32,i32), Direction)> = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Node {
        position: start,
        cost: 0,
        direction: Direction::Right,
        path: HashSet::from([start]),
    }));
    let exit_cost: u32;
    loop {
        let current_node = to_visit.pop().unwrap().0;
        visited.insert((current_node.position, current_node.direction));
        if current_node.position == exit {
            exit_cost = current_node.cost;
            break;
        }
        //check adjacent nodes and add them to the to_visit set
        let current_v = direction_map.get(&current_node.direction).unwrap();
        for direction in [Direction::Up, Direction::Left, Direction::Right, Direction::Down] {
            let v = direction_map.get(&direction).unwrap();
            let new_position = (current_node.position.0 + v.0, current_node.position.1 + v.1);
            if !visited.contains(&(new_position, direction)) && *maze.get(&new_position).unwrap_or(&'#') != '#'  {
                let cost = current_v.0.abs_diff(v.0).max(current_v.1.abs_diff(v.1))*1000 + 1;
                let mut path = current_node.path.clone();
                path.insert(new_position);
                to_visit.push(Reverse(Node {
                    position: new_position,
                    cost: cost + current_node.cost,
                    direction,
                    path,
                }));
            }
        }
    }
    submit!(1, exit_cost);
}

#[aocd(2024, 16)]
pub fn two() {
    let input = input!();
    let direction_map = HashMap::from([(Direction::Up, (-1, 0)), (Direction::Down, (1, 0)), (Direction::Left, (0, -1)), (Direction::Right, (0, 1))]);
    let mut start = (0,0);
    let mut exit = (0,0);
    let mut maze = HashMap::new();
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let i = i as i32;
            let j = j as i32;
            maze.insert((i,j), c);
            if c == 'S' {
                start = (i,j);
            } else if c == 'E' {
                exit = (i,j);
            }
        });
    });
    let mut visited: HashSet<((i32,i32), Direction)> = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(Node {
        position: start,
        cost: 0,
        direction: Direction::Right,
        path: HashSet::from([start]),
    }));
    let mut exit_cost = None;
    let mut optimal_path_nodes = HashSet::new();
    loop {
        let current_node = to_visit.pop().unwrap().0;
        if current_node.position == exit {
            if let Some(exit_cost) = exit_cost {
                if current_node.cost > exit_cost {
                    break;
                }
            }
            exit_cost = Some(current_node.cost);
            for node in current_node.path {
                optimal_path_nodes.insert(node);
            }
            continue;
        }
        visited.insert((current_node.position, current_node.direction));
        //check adjacent nodes and add them to the to_visit set
        let current_v = direction_map.get(&current_node.direction).unwrap();
        for direction in [Direction::Up, Direction::Left, Direction::Right, Direction::Down] {
            let v = direction_map.get(&direction).unwrap();
            let new_position = (current_node.position.0 + v.0, current_node.position.1 + v.1);
            if !visited.contains(&(new_position, direction)) && *maze.get(&new_position).unwrap_or(&'#') != '#'  {
                let cost = current_v.0.abs_diff(v.0).max(current_v.1.abs_diff(v.1))*1000 + 1;
                let mut path = current_node.path.clone();
                path.insert(new_position);
                to_visit.push(Reverse(Node {
                    position: new_position,
                    cost: cost + current_node.cost,
                    direction,
                    path,
                }));
            }
        }
    }
    submit!(2, optimal_path_nodes.len());
}