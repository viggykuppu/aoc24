use std::collections::HashMap;

use aocd::*;
use itertools::Itertools;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[aocd(2024, 15)]
pub fn one() {
    let input = input!();
    let direction_map = HashMap::from([(Direction::Up, (-1, 0)), (Direction::Down, (1, 0)), (Direction::Left, (0, -1)), (Direction::Right, (0, 1))]);
    let grid_string = input.split("\n\n").collect::<Vec<_>>()[0];
    let instruction_string = input.split("\n\n").collect::<Vec<_>>()[1];
    let mut grid = HashMap::new();
    let instructions = build_instructions(&instruction_string);
    let mut robot_position = (0,0);
    let mut bounds = (0,0);
    grid_string.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let i = i as i32;
            let j = j as i32;
            grid.insert((i,j), c);
            if c == '@' {
                robot_position = (i,j);
            }
            bounds.0 = i+1;
            bounds.1 = j + 1;
        });
    });
    
    for instruction in instructions {
        let v = direction_map.get(&instruction).unwrap();
        let moves = make_move(&mut grid, robot_position, v, '@');
        for move_instruction in moves {
            *grid.get_mut(&move_instruction.start).unwrap() = '.';
            *grid.get_mut(&move_instruction.end).unwrap() = move_instruction.item;
            if move_instruction.item == '@' {
                robot_position = move_instruction.end;
            }
        }
    }
    let gps_sum: u32 = grid.keys().map(|k| {
        let c = grid.get(k).unwrap();
        if *c == 'O' {
            return (k.0 as u32)*100 + (k.1 as u32);
        }
        0
    }).sum();
    submit!(1, gps_sum);
}

#[aocd(2024, 15)]
pub fn two() {
    let input = input!();
    let direction_map = HashMap::from([(Direction::Up, (-1, 0)), (Direction::Down, (1, 0)), (Direction::Left, (0, -1)), (Direction::Right, (0, 1))]);
    let grid_string = input.split("\n\n").collect::<Vec<_>>()[0];
    let instruction_string = input.split("\n\n").collect::<Vec<_>>()[1];
    let mut grid = HashMap::new();
    let instructions = build_instructions(&instruction_string);
    let mut robot_position = (0,0);
    let mut bounds = (0,0);
    grid_string.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            let i = i as i32;
            let j = (j as i32)*2;
            if c == '#' {
                grid.insert((i,j), c);
                grid.insert((i,j+1), c);
            } else if c == '@' {
                grid.insert((i,j), c);
                robot_position = (i,j);
                grid.insert((i,j+1), '.');
            } else if c == '.' {
                grid.insert((i,j), c);
                grid.insert((i,j+1), c);
            } else if c == 'O' {
                grid.insert((i,j), '[');
                grid.insert((i,j+1), ']');
            }
            bounds.0 = i+1;
            bounds.1 = j + 2;
        });
    });
    for instruction in instructions.iter() {
        let v = direction_map.get(&instruction).unwrap();
        let mut moves = make_move(&mut grid, robot_position, v, '@');
        if *instruction == Direction::Up {
            moves.sort_by(|a, b| {
                a.start.0.cmp(&b.start.0)
            });
        } else if *instruction == Direction::Down {
            moves.sort_by(|a, b| {
                b.start.0.cmp(&a.start.0)
            });
        }
        for move_instruction in moves {
            *grid.get_mut(&move_instruction.start).unwrap() = '.';
            *grid.get_mut(&move_instruction.end).unwrap() = move_instruction.item;
            if move_instruction.item == '@' {
                robot_position = move_instruction.end;
            }
        }
    }
    let gps_sum: u64 = grid.keys().map(|k| {
        let c = grid.get(k).unwrap();
        if *c == '[' {
            return (k.0 as u64)*100 + (k.1 as u64);
        }
        0
    }).sum();
    submit!(2, gps_sum);
}

fn make_move(grid: &mut HashMap<(i32,i32), char>, current: (i32, i32), velocity: &(i32, i32), item: char) -> Vec<Move> {
    let new_position = (current.0 + velocity.0, current.1 + velocity.1);
    let new_position_space = *grid.get(&new_position).unwrap();
    let mut moves = vec![];
    if new_position_space == '.' {
        // move into free spot
        return vec![Move {
            item: item,
            start: current,
            end: new_position,
        }];
    } else if (new_position_space == '[' || new_position_space == ']') && velocity.1 != 0 {
        // push boxes left or right, so no special logic!!
        moves = make_move(grid, new_position, velocity, new_position_space);
        if !moves.is_empty() {
            moves.push(Move {
                item: item,
                start: current,
                end: new_position,
            });
        }
    } else if new_position_space == 'O' {
        // push boxes!!
        moves = make_move(grid, new_position, velocity, 'O');
        if !moves.is_empty() {
            moves.push(Move {
                item: item,
                start: current,
                end: new_position,
            });
        }
    } else if new_position_space == '[' {
        // push boxes!!
        let box_right_side_position = (new_position.0, new_position.1 + 1);
        let left_moves = make_move(grid, new_position, velocity, '[');
        let right_moves = make_move(grid, box_right_side_position, velocity, ']');
        if !left_moves.is_empty() && !right_moves.is_empty() {
            moves = left_moves.into_iter().interleave(right_moves.into_iter()).collect();
            moves.push(Move {
                item: item,
                start: current,
                end: new_position,
            });
        }
    }  else if new_position_space == ']' {
        // push boxes!!
        let box_left_side_position = (new_position.0, new_position.1 - 1);
        let left_moves = make_move(grid, box_left_side_position, velocity, '[');
        let right_moves = make_move(grid, new_position, velocity, ']');
        if !left_moves.is_empty() && !right_moves.is_empty() {
            moves = left_moves.into_iter().interleave(right_moves.into_iter()).collect();
            moves.push(Move {
                item: item,
                start: current,
                end: new_position,
            });
        }
    } else if new_position_space == '#' {
        // do nothing because we're running into a wall
        return vec![];
    }
    moves
}

#[derive(Clone, Debug)]
struct Move {
    item: char,
    start: (i32, i32),
    end: (i32, i32),
}

fn print_grid(grid: &HashMap<(i32,i32), char>, bounds: (i32, i32)) {
    for i in 0..bounds.0 {
        for j in 0..bounds.1 {
            let c = grid.get(&(i,j)).unwrap();
            print!("{c}");
        }
        println!();
    }
}

fn build_instructions(instruction_string: &str) -> Vec<Direction> {
    let mut instructions = Vec::new();
    instruction_string.lines().for_each(|line| {
        line.chars().for_each(|c| {
            if c == '^' {
                instructions.push(Direction::Up);
            } else if c == '>' {
                instructions.push(Direction::Right);
            } else if c == '<' {
                instructions.push(Direction::Left);
            } else if c == 'v' {
                instructions.push(Direction::Down);
            }
        });
    });
    instructions
}


#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}