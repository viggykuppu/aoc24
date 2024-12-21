use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2024, 8)]
pub fn one() {
    let input = input!();
    let (grid, nodes) = build_grid(&input);
    let bounds = (grid.len(), grid[0].len());
    let mut anti_nodes: HashSet<(isize, isize)> = HashSet::new();
    for same_nodes in nodes.values() {
        for node1 in same_nodes {
            for node2 in same_nodes {
                if node1 != node2 {
                    let dx = (node1.0 as isize) - (node2.0 as isize);
                    let dy = (node1.1 as isize) - (node2.1 as isize);
                    let anti_node = (node1.0 as isize + dx, node1.1 as isize + dy);
                    anti_nodes.insert(anti_node);
                }
            }
        }
    }
    print_grid_with_anti_nodes(&grid, &anti_nodes, &bounds);
    let total_anti_nodes: usize = anti_nodes
        .iter()
        .filter(|anti_node| {
            (0..(bounds.0 as isize)).contains(&anti_node.0)
                && (0..(bounds.1 as isize)).contains(&anti_node.1)
        })
        .collect::<Vec<_>>()
        .len();
    submit!(1, total_anti_nodes);
}

#[aocd(2024, 8)]
pub fn two() {
    let input = input!();
    let (grid, nodes) = build_grid(&input);
    let bounds = (grid.len(), grid[0].len());
    let mut anti_nodes: HashSet<(isize, isize)> = HashSet::new();
    for same_nodes in nodes.values() {
        for node1 in same_nodes {
            for node2 in same_nodes {
                if node1 != node2 {
                    let dx = (node1.0 as isize) - (node2.0 as isize);
                    let dy = (node1.1 as isize) - (node2.1 as isize);
                    // get all antinodes starting at node1 in that direction
                    let mut current = (node1.0 as isize, node1.1 as isize);
                    while (0..(bounds.0 as isize)).contains(&current.0)
                        && (0..(bounds.1 as isize)).contains(&current.1)
                    {
                        anti_nodes.insert(current);
                        current.0 += dx;
                        current.1 += dy;
                    }
                }
            }
        }
    }
    print_grid_with_anti_nodes(&grid, &anti_nodes, &bounds);
    let total_anti_nodes: usize = anti_nodes
        .iter()
        .filter(|anti_node| {
            (0..(bounds.0 as isize)).contains(&anti_node.0)
                && (0..(bounds.1 as isize)).contains(&anti_node.1)
        })
        .collect::<Vec<_>>()
        .len();
    submit!(2, total_anti_nodes);
}

fn build_grid(input: &str) -> (Vec<Vec<char>>, HashMap<char, HashSet<(usize, usize)>>) {
    let mut nodes: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    for i in 0..(grid.len()) {
        for j in 0..(grid[0].len()) {
            let c = grid[i][j];
            if c != '.' {
                nodes.entry(c).or_default().insert((i, j));
            }
        }
    }
    (grid, nodes)
}

fn print_grid_with_anti_nodes(
    grid: &[Vec<char>],
    anti_nodes: &HashSet<(isize, isize)>,
    bounds: &(usize, usize),
) {
    for (i, row) in grid.iter().enumerate().take(bounds.0) {
        for (j, c) in row.iter().enumerate().take(bounds.1) {
            if *c == '.' {
                if anti_nodes.contains(&(i as isize, j as isize)) {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!("{c}");
            }
        }
        println!();
    }
}
