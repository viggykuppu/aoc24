use std::{collections::{HashMap, HashSet}};
use aocd::*;

#[aocd(2024, 6)]
pub fn one() {
    let input = input!();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard: (isize, isize) = (0,0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                guard = (i as isize,j  as isize);
            }
        }
    }
    let visited = travel(&grid, guard, Direction::UP);
    submit!(1, visited.len());
}

fn travel(grid: &Vec<Vec<char>>, start: (isize, isize), start_direction: Direction) -> HashSet<(isize, isize)> {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut direction: Direction = start_direction;
    let mut guard = start;
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let turn_map = HashMap::from([(Direction::UP, Direction::RIGHT), (Direction::DOWN, Direction::LEFT),(Direction::LEFT, Direction::UP),(Direction::RIGHT, Direction::DOWN)]);
    loop {
        // record current position as visited
        visited.insert(guard);
        // determine new position if going in a straight line
        let velocity = direction_map.get(&direction).unwrap();
        let mut new_position = (guard.0 + velocity.0, guard.1 + velocity.1);
        // check if new position goes out of bounds
        if !(0..(grid.len() as isize)).contains(&new_position.0) || !(0..(grid[0].len() as isize)).contains(&new_position.1) {
            break;
        }
        // check if new position hits barrier
        if grid[new_position.0 as usize][new_position.1 as usize] == '#' {
            direction = *turn_map.get(&direction).unwrap();
            let velocity = direction_map.get(&direction).unwrap();
            new_position = (guard.0 + velocity.0, guard.1 + velocity.1);
            if grid[new_position.0 as usize][new_position.1 as usize] == '#' {
                direction = *turn_map.get(&direction).unwrap();
                let velocity = direction_map.get(&direction).unwrap();
                new_position = (guard.0 + velocity.0, guard.1 + velocity.1);
            }
        }
        guard = new_position;
    }
    visited
}

#[aocd(2024, 6)]
pub fn two() {
    let input = input!();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard: (isize, isize) = (0,0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                guard = (i as isize,j  as isize);
            }
        }
    }

    let visited = travel(&grid, guard, Direction::UP);
    let mut new_barriers: HashSet<(isize, isize)> = HashSet::new();
    for position in visited {
        if position != guard {
            let mut new_grid = grid.clone();
            new_grid[position.0 as usize][position.1 as usize] = '#';
            if is_infinite_loop(&new_grid, guard, Direction::UP) {
                new_barriers.insert(position);
            }
        }
    }
    submit!(2, new_barriers.len());
}

fn is_infinite_loop(grid: &Vec<Vec<char>>, start: (isize, isize), start_direction: Direction) -> bool {
    let mut visited: HashSet<((isize, isize), Direction)> = HashSet::new();
    let mut direction: Direction = start_direction;
    let mut guard = start;
    let direction_map = HashMap::from([(Direction::UP, (-1, 0)), (Direction::DOWN, (1, 0)),(Direction::LEFT, (0, -1)),(Direction::RIGHT, (0, 1))]);
    let turn_map = HashMap::from([(Direction::UP, Direction::RIGHT), (Direction::DOWN, Direction::LEFT),(Direction::LEFT, Direction::UP),(Direction::RIGHT, Direction::DOWN)]);
    loop {
        // double check we're not caught in a loop
        if visited.contains(&(guard, direction)) {
            return true;
        }
        // record current position as visited
        visited.insert((guard, direction));
        // determine new position if going in a straight line
        let velocity = direction_map.get(&direction).unwrap();
        let mut new_position = (guard.0 + velocity.0, guard.1 + velocity.1);
        // check if new position goes out of bounds
        if !(0..(grid.len() as isize)).contains(&new_position.0) || !(0..(grid[0].len() as isize)).contains(&new_position.1) {
            break;
        }
        // check if new position hits barrier
        if grid[new_position.0 as usize][new_position.1 as usize] == '#' {
            direction = *turn_map.get(&direction).unwrap();
            let velocity = direction_map.get(&direction).unwrap();
            new_position = (guard.0 + velocity.0, guard.1 + velocity.1);
            if grid[new_position.0 as usize][new_position.1 as usize] == '#' {
                direction = *turn_map.get(&direction).unwrap();
                let velocity = direction_map.get(&direction).unwrap();
                new_position = (guard.0 + velocity.0, guard.1 + velocity.1);
            }
        }
        guard = new_position;
    }
    return false;
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    UP,
    RIGHT,
    LEFT,
    DOWN,
}