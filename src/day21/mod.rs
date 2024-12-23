use std::{collections::HashMap, usize};

use aocd::*;
use itertools::Itertools;

#[aocd(2024, 21)]
pub fn one() {
    let input = input!();
    let numpad_map = HashMap::from([
        ('0', (1, 0)),
        ('A', (2, 0)),
        ('1', (0, 1)),
        ('2', (1, 1)),
        ('3', (2, 1)),
        ('4', (0, 2)),
        ('5', (1, 2)),
        ('6', (2, 2)),
        ('7', (0, 3)),
        ('8', (1, 3)),
        ('9', (2, 3)),
    ]);
    let dirpad_map = HashMap::from([
        ('<', (0, 0)),
        ('v', (1, 0)),
        ('>', (2, 0)),
        ('^', (1, 1)),
        ('A', (2, 1)),
    ]);
    let direction_map = HashMap::from([
        ('<', (-1, 0)),
        ('v', (0, -1)),
        ('>', (1, 0)),
        ('^', (0, 1)),
        ('A', (0, 0)),
    ]);
    let mut memo = HashMap::new();
    let complexity_sum: usize = input
        .lines()
        .map(|line| {
            let numeric_code = line[0..3].parse::<usize>().unwrap();
            let mut current_robot_moves =
                builds_dirpad_commands_for_input(line, &numpad_map, &direction_map, 'A', &(0, 0));
            let min_input_length = current_robot_moves
                .iter()
                .map(|moves| {
                    get_shortest_input_length(
                        moves,
                        2,
                        &mut memo,
                        &dirpad_map,
                        &direction_map,
                        &(0, 1),
                    )
                })
                .min()
                .unwrap();
            min_input_length * numeric_code
        })
        .sum();
    submit!(1, complexity_sum);
}

#[aocd(2024, 21)]
pub fn two() {
    let input = input!();
    let numpad_map = HashMap::from([
        ('0', (1, 0)),
        ('A', (2, 0)),
        ('1', (0, 1)),
        ('2', (1, 1)),
        ('3', (2, 1)),
        ('4', (0, 2)),
        ('5', (1, 2)),
        ('6', (2, 2)),
        ('7', (0, 3)),
        ('8', (1, 3)),
        ('9', (2, 3)),
    ]);
    let dirpad_map = HashMap::from([
        ('<', (0, 0)),
        ('v', (1, 0)),
        ('>', (2, 0)),
        ('^', (1, 1)),
        ('A', (2, 1)),
    ]);
    let direction_map = HashMap::from([
        ('<', (-1, 0)),
        ('v', (0, -1)),
        ('>', (1, 0)),
        ('^', (0, 1)),
        ('A', (0, 0)),
    ]);
    let mut memo = HashMap::new();
    let complexity_sum: usize = input
        .lines()
        .map(|line| {
            let numeric_code = line[0..3].parse::<usize>().unwrap();
            let current_robot_moves =
                builds_dirpad_commands_for_input(line, &numpad_map, &direction_map, 'A', &(0, 0));
            let min_input_length = current_robot_moves
                .iter()
                .map(|moves| {
                    get_shortest_input_length(
                        moves,
                        25,
                        &mut memo,
                        &dirpad_map,
                        &direction_map,
                        &(0, 1),
                    )
                })
                .min()
                .unwrap();
            min_input_length * numeric_code
        })
        .sum();
    submit!(2, complexity_sum);
}

fn get_shortest_input_length<'a>(
    input: &'a str,
    num_robots_left: usize,
    memo: &mut HashMap<(String, usize), usize>,
    position_map: &HashMap<char, (i32, i32)>,
    direction_map: &HashMap<char, (i32, i32)>,
    forbidden: &(i32, i32),
) -> usize {
    if let Some(length) = memo.get(&(input.to_string(), num_robots_left)) {
        return *length;
    }
    if num_robots_left == 0 {
        return input.len();
    }
    let min_length = input
        .split('A')
        .map(|chunk| {
            let input_chunk = format!("{chunk}A");
            let results = builds_dirpad_commands_for_input(
                &input_chunk,
                position_map,
                direction_map,
                'A',
                forbidden,
            );
            let min_length = results.iter().map(|r| r.len()).min().unwrap();
            let mut x = usize::MAX;
            for r in results {
                if r.len() == min_length {
                    let y = get_shortest_input_length(
                        &r,
                        num_robots_left - 1,
                        memo,
                        position_map,
                        direction_map,
                        forbidden,
                    );
                    if y < x {
                        x = y;
                    }
                }
            }
            memo.insert((input_chunk, num_robots_left), min_length);
            x
        })
        .sum::<usize>()
        - 1;
    memo.insert((input.to_string(), num_robots_left), min_length);
    min_length
}

fn builds_dirpad_commands_for_input(
    input: &str,
    position_map: &HashMap<char, (i32, i32)>,
    direction_map: &HashMap<char, (i32, i32)>,
    current: char,
    forbidden: &(i32, i32),
) -> Vec<String> {
    let mut possibilities = Vec::new();
    if input.len() == 0 {
        return vec![String::new()];
    }
    let current_position = position_map.get(&current).unwrap();
    let next_button = input.chars().next().unwrap();
    let target_position = position_map.get(&next_button).unwrap();
    let v_x = target_position.0 - current_position.0;
    let v_y = target_position.1 - current_position.1;
    let mut move_options = Vec::new();
    for _ in 0..v_x.abs() {
        if v_x.is_positive() {
            move_options.push('>');
        } else if v_x.is_negative() {
            move_options.push('<')
        }
    }
    for _ in 0..v_y.abs() {
        if v_y.is_positive() {
            move_options.push('^');
        } else if v_y.is_negative() {
            move_options.push('v')
        }
    }
    'perm_loop: for perm in move_options
        .iter()
        .permutations(move_options.len())
        .map(|perm| perm.into_iter().collect::<String>())
        .unique()
    {
        let mut current_position = current_position.clone();
        for m in perm.chars() {
            let v = direction_map.get(&m).unwrap();
            current_position = (current_position.0 + v.0, current_position.1 + v.1);
            if current_position == *forbidden {
                continue 'perm_loop;
            }
        }
        let rest = builds_dirpad_commands_for_input(
            &input[1..],
            position_map,
            direction_map,
            next_button,
            forbidden,
        );
        for m in rest {
            possibilities.push(format!("{perm}A{m}"));
        }
    }
    possibilities
}
