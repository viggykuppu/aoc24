use std::{
    collections::{HashMap, HashSet},
    usize,
};

use aocd::*;
use itertools::Itertools;

#[derive(Debug)]
struct Node<'a> {
    next: HashSet<&'a str>,
}

#[aocd(2024, 23)]
pub fn one() {
    let input = input!();
    let mut graph = HashMap::new();
    let mut possible_historian_computers = HashSet::new();
    input.lines().for_each(|line| {
        let a = &line[0..2];
        let b = &line[3..5];
        if a.starts_with('t') {
            possible_historian_computers.insert(a);
        }
        if b.starts_with('t') {
            possible_historian_computers.insert(b);
        }
        let left_node = graph.entry(a).or_insert(Node {
            next: HashSet::new(),
        });
        left_node.next.insert(b);
        let right_node = graph.entry(b).or_insert(Node {
            next: HashSet::new(),
        });
        right_node.next.insert(a);
    });
    let mut seen_3_cycles = HashSet::new();
    for node in possible_historian_computers {
        find_3_cycles(node, &graph, &mut seen_3_cycles);
    }
    submit!(1, seen_3_cycles.len());
}

fn find_3_cycles(
    start: &str,
    graph: &HashMap<&str, Node>,
    seen_3_cycles: &mut HashSet<String>,
) -> usize {
    let start_adjacencies = &graph.get(start).unwrap().next;
    for adjacent_node in start_adjacencies {
        let next_adjacencies = &graph.get(adjacent_node).unwrap().next;
        for f in next_adjacencies.intersection(start_adjacencies) {
            let mut cycle_string = vec![start, adjacent_node, f];
            cycle_string.sort();
            let cycle_string = cycle_string.join("");
            seen_3_cycles.insert(cycle_string);
        }
    }
    seen_3_cycles.len()
}

#[aocd(2024, 23)]
pub fn two() {
    let input = input!();
    let mut graph: HashMap<&str, Node<'_>> = HashMap::new();
    let mut computers = HashSet::new();
    input.lines().for_each(|line| {
        let a = &line[0..2];
        let b = &line[3..5];
        computers.insert(a);
        computers.insert(b);
        let left_node = graph.entry(a).or_insert(Node {
            next: HashSet::new(),
        });
        left_node.next.insert(b);
        let right_node = graph.entry(b).or_insert(Node {
            next: HashSet::new(),
        });
        right_node.next.insert(a);
    });
    let mut seen_3_cycles = HashSet::new();
    for node in computers {
        find_3_cycles(node, &graph, &mut seen_3_cycles);
    }
    let three_cycles: Vec<_> = seen_3_cycles.iter().map(|s| cycle_to_ids(s)).collect();
    let mut map = HashMap::new();
    for cycle in three_cycles.iter() {
        for n in cycle.iter() {
            map.insert(n, cycle);
        }
    }
    let mut memo = HashMap::new();
    let max_lan_size = three_cycles
        .iter()
        .map(|c| get_lan_size(&graph, c, &mut memo))
        .max()
        .unwrap();
    let mut answer = String::new();
    for (k, _) in memo.iter() {
        if k.len() == max_lan_size * 2 {
            let mut ids = Vec::new();
            while ids.len() < max_lan_size {
                let i = ids.len();
                ids.push(k[i * 2..(i * 2 + 2)].to_string());
            }
            answer = ids.join(",");
        }
    }
    submit!(2, answer);
}

fn get_lan_size(
    graph: &HashMap<&str, Node>,
    cycle: &HashSet<&str>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let mut lan_size = cycle.len();
    let x = ids_to_cycle(cycle);
    if let Some(v) = memo.get(&x) {
        return *v;
    }
    for node in cycle.iter() {
        let potential_additions_to_cycle = &graph.get(node).unwrap().next;
        for n in potential_additions_to_cycle.iter() {
            if !cycle.contains(n) {
                let n_is_part_of_cycle =
                    cycle.iter().all(|x| graph.get(x).unwrap().next.contains(n));
                if n_is_part_of_cycle {
                    let mut new_cycle = cycle.clone();
                    new_cycle.insert(n);
                    let y = get_lan_size(graph, &new_cycle, memo);
                    if y > lan_size {
                        lan_size = y;
                    }
                }
            }
        }
    }
    memo.insert(x, lan_size);
    lan_size
}

fn cycle_to_ids(cycle: &str) -> HashSet<&str> {
    HashSet::from([&cycle[0..2], &cycle[2..4], &cycle[4..6]])
}

fn ids_to_cycle(ids: &HashSet<&str>) -> String {
    ids.into_iter().sorted().join("")
}
