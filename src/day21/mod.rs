use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, usize};

use aocd::*;
use itertools::Itertools;
use nalgebra::base;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Input {
    Up,
    Right,
    Left,
    Down,
    Activate,
}

#[aocd(2024, 21, "src/day21/input.txt")]
pub fn one() {
    let input = input!();
    let numpad_map = HashMap::from([('0', (1,0)),('A',(2,0)), ('1',(0,1)), ('2',(1,1)), ('3',(2,1)), ('4',(0,2)), ('5',(1,2)), ('6',(2,2)), ('7',(0,3)), ('8',(1,3)), ('9',(2,3))]);
    let position_to_numpad_map = HashMap::from([((1,0), '0'),((2,0), 'A'), ((0,1), '1'), ((1,1), '2'), ((2,1), '3'), ((0,2), '4'), ((1,2), '5'), ((2,2), '6'), ((0,3), '7'), ((1,3), '8'), ((2,3), '9')]);
    let position_to_dirpad_map = HashMap::from([((0,0), '<'),((1,0), 'v'), ((2,0), '>'), ((1,1), '^'), ((2,1), 'A')]);
    let dirpad_map = HashMap::from([('<', (0,0)),('v', (1,0)), ('>', (2,0)), ('^', (1,1)), ('A', (2,1))]);
    let direction_map = HashMap::from([('<', (-1,0)),('v', (0,-1)), ('>', (1,0)), ('^', (0,1)), ('A', (0,0))]);
    let complexity_sum: usize = input.lines().map(|line| {
        // println!("{line}-----------");
        let numeric_code = line[0..3].parse::<usize>().unwrap();
        let mut current_robot_moves = builds_dirpad_commands_for_input(line, &numpad_map, &direction_map, 'A', &(0,0));
        // println!("{moves:?}");
        let mut smallest = usize::MAX;
        for _ in 0..2 {
            smallest = usize::MAX;
            let mut smallest_moves = Vec::new();
            for current_robot_move in current_robot_moves {
                let moves = builds_dirpad_commands_for_input(&current_robot_move, &dirpad_map, &direction_map, 'A', &(0, 1));
                for m in moves {
                    if m.len() < smallest {
                        smallest = m.len();
                        smallest_moves.clear();
                    }
                    if m.len() == smallest {
                        smallest_moves.push(m);
                    }
                }
            }
            current_robot_moves = smallest_moves;
        }
        // for m in moves.iter() {
        //     let first_robot_moves = foo(&m, &dirpad_map, &direction_map, 'A', &(0, 1));
        //     let l = m.len();
        //     // println!("first robot moves length: {l:?}");
        //     for m in first_robot_moves.iter() {
        //         let l = m.len();
        //     // println!("second robot moves length: {l:?}");
        //         let second_robot_moves = foo(&m, &dirpad_map, &direction_map, 'A', &(0, 1));
        //         for m2 in second_robot_moves {
        //             if m2.len() < smallest {
        //                 smallest = m2.len();
        //                 // println!("m: {m}");
        //                 // println!("m2: {m2}");
        //                 // let x = bar(&m2, &dirpad_map, &direction_map, &position_to_dirpad_map, 'A');
        //                 // println!("{x}");
        //                 // let y = bar(&x, &dirpad_map, &direction_map, &position_to_dirpad_map, 'A');
        //                 // println!("{y}");
        //                 // let z = bar(&y, &numpad_map, &direction_map, &position_to_numpad_map, 'A');
        //                 // println!("{z}");
        //             }
        //         }
        //     }
        // }
        numeric_code*smallest
    }).sum();
    // let z = bar("<A^A>^^AvvvA", &numpad_map, &direction_map, &position_to_numpad_map, 'A');
    // println!("{z}");
    submit!(1, complexity_sum);
}

#[aocd(2024, 20, "src/day20/input.txt")]
pub fn two() {
    let numpad_map = HashMap::from([('0', (1,0)),('A',(2,0)), ('1',(0,1)), ('2',(1,1)), ('3',(2,1)), ('4',(0,2)), ('5',(1,2)), ('6',(2,2)), ('7',(0,3)), ('8',(1,3)), ('9',(2,3))]);
    let position_to_numpad_map = HashMap::from([((1,0), '0'),((2,0), 'A'), ((0,1), '1'), ((1,1), '2'), ((2,1), '3'), ((0,2), '4'), ((1,2), '5'), ((2,2), '6'), ((0,3), '7'), ((1,3), '8'), ((2,3), '9')]);
    let position_to_dirpad_map = HashMap::from([((0,0), '<'),((1,0), 'v'), ((2,0), '>'), ((1,1), '^'), ((2,1), 'A')]);
    let dirpad_map = HashMap::from([('<', (0,0)),('v', (1,0)), ('>', (2,0)), ('^', (1,1)), ('A', (2,1))]);
    let direction_map = HashMap::from([('<', (-1,0)),('v', (0,-1)), ('>', (1,0)), ('^', (0,1)), ('A', (0,0))]);
    let mut memo = HashMap::new();
    let z = foo("029A", 2, &mut memo, &dirpad_map, &direction_map, 'A', &(0, 1));
    println!("{z}");
    // submit!(2, fast_cheats);
}

fn foo<'a>(input: &'a str, depth: usize, memo: &mut HashMap<(String, usize), usize>, position_map: &HashMap<char, (i32, i32)>, direction_map: &HashMap<char, (i32, i32)>, current: char, forbidden: &(i32, i32)) -> usize {
    if let Some(length) = memo.get(&(input.to_string(), depth)) {
        println!("used memo!");
        return *length;
    }
    if depth == 0 {
        return input.len();
    }
    let min_length = input.split('A').map(|chunk| {
        let input_chunk = format!("{chunk}A");
        let results = builds_dirpad_commands_for_input(&input_chunk, position_map, direction_map, current, forbidden);
        let min_length = results.iter().map(|r| r.len()).min().unwrap();
        let mut x = usize::MAX;
        for r in results {
            if r.len() == min_length {
                let y = foo(&r, depth - 1, memo, position_map, direction_map, current, forbidden);
                if y < x {
                    x = y;
                }
            }
        }
        // println!("{input_chunk} {x}");
        memo.insert((input_chunk, depth), min_length);
        x
    }).sum::<usize>() - 1;
    min_length
}

fn builds_dirpad_commands_for_input(input: &str, position_map: &HashMap<char, (i32, i32)>, direction_map: &HashMap<char, (i32, i32)>, current: char, forbidden: &(i32, i32)) -> Vec<String> {
    // println!("called with {code} and {current}");
    let mut possibilities = Vec::new();
    if input.len() == 0 {
        return vec![String::new()];
    }
    let current_position = position_map.get(&current).unwrap();
    let next_button = input.chars().next().unwrap();
    // println!("next is {next_button}");
    let target_position = position_map.get(&next_button).unwrap();
    let v_x = target_position.0 - current_position.0;
    let v_y = target_position.1 - current_position.1;
    // println!("vx {v_x:?}, v_y {v_y:?}");
    let mut move_options = Vec::new();
    for _ in 0..v_x.abs() {
        if v_x.is_positive() {
            move_options.push('>');
        } else if v_x.is_negative() {
            move_options.push('<')
        }
    }
    let mut y_moves = String::new();
    for _ in 0..v_y.abs() {
        if v_y.is_positive() {
            move_options.push('^');
        } else if v_y.is_negative() {
            move_options.push('v')
        }
    }
    'perm_loop: for perm in move_options.iter().permutations(move_options.len()).map(|perm| perm.into_iter().collect::<String>()).unique() {
        let mut current_position = current_position.clone();
        for m in perm.chars() {
            let v = direction_map.get(&m).unwrap();
            current_position = (current_position.0 + v.0, current_position.1 + v.1);
            if current_position == *forbidden {
                continue 'perm_loop;
            }
        }
        let rest = builds_dirpad_commands_for_input(&input[1..], position_map, direction_map, next_button, forbidden);
        for m in rest {
            possibilities.push(format!("{perm}A{m}"));
        }
    }
    possibilities
}

fn bar(sequence: &str, dirpad: &HashMap<char, (i32, i32)>, direction_map: &HashMap<char, (i32, i32)>, position_to_dirpad_map: &HashMap<(i32, i32), char>, current: char) -> String {
    if sequence.len() == 0 {
        return String::new();
    }
    let mut output = String::new();
    let command = sequence.chars().next().unwrap();
    if command == 'A' {
        output.push(current);
    }
    let current_location = dirpad.get(&current).unwrap();
    let v = direction_map.get(&command).unwrap();
    let new_location = (current_location.0 + v.0, current_location.1 + v.1);
    println!("{new_location:?}");
    let new_char = position_to_dirpad_map.get(&new_location).unwrap();
    let rest = bar(&sequence[1..],dirpad, direction_map, position_to_dirpad_map, *new_char);
    output.push_str(&rest);
    return output;
}