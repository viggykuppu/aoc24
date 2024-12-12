use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2024, 12)]
pub fn one() {
    let input = input!();
    let mut garden: HashMap<(isize, isize), char> = HashMap::new();
    let mut m: HashMap<char, (isize, isize)> = HashMap::new();
    let mut plants: HashSet<char> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            garden.insert((i as isize,j as isize), c);
            m.insert(c, (i as isize,j as isize));
            plants.insert(c);
        }
    };
    let mut total_price = 0;
    // for plant in plants {
    //     let start = m.get(&plant).unwrap();
    //     let mut visited = HashSet::new();
    //     let p = travel(&garden, *start, plant, &mut visited);
    //     let l = visited.len();
    //     println!("{plant} has per {p} and area {l}");
    //     total_price += p*visited.len();
    // }
    let mut visited = HashSet::new();
    for (i,j) in garden.keys() {
        let plant = garden.get(&(*i,*j)).unwrap();
        if !visited.contains(&(*i,*j)) {
            let (p, a) = travel(&garden, (*i, *j), *plant, &mut visited);
            total_price += p*a;
            // println!("{plant} has per {p} and area {a}");
        }
    }
    submit!(1, total_price);
}

fn travel(garden: &HashMap<(isize, isize), char>, current: (isize, isize), plant: char, visited: &mut HashSet<(isize, isize)>) -> (usize, usize) {
    visited.insert(current);
    let mut p = 0;
    let mut a = 1;
    for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
        let new_location = (current.0 + v.0, current.1 + v.1);
        let new_plant = garden.get(&new_location);
        if new_plant.is_some() && *new_plant.unwrap() == plant {
            if !visited.contains(&new_location) {
                let result = travel(garden, new_location, plant, visited);
                p += result.0;
                a += result.1;
            }
        } else {
            p += 1;
        }
    }
    (p, a)
}

#[aocd(2024, 12)]
pub fn two() {
    let direction_map = HashMap::from([(Direction::Up, (-1, 0)), (Direction::Down, (1, 0)), (Direction::Left, (0, -1)), (Direction::Right, (0, 1))]);
    let input = input!();
    let mut garden: HashMap<(isize, isize), char> = HashMap::new();
    let mut m: HashMap<char, (isize, isize)> = HashMap::new();
    let mut plants: HashSet<char> = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            garden.insert((i as isize,j as isize), c);
            m.insert(c, (i as isize,j as isize));
            plants.insert(c);
        }
    };
    let garden_grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut total_price = 0;
    let mut visited = HashSet::new();
    for i in 0..garden_grid.len() {
        for j in 0..garden_grid[0].len() {
            let i = i as isize;
            let j = j as isize;
            let plant = garden.get(&(i,j)).unwrap();
            if !visited.contains(&(i,j)) {
                let mut edges = HashMap::new();
                let a = travel_two(&garden, (i, j), *plant, &mut visited, &mut edges);
                let mut visited = HashSet::new();
                let mut graph = HashMap::new();
                for edge in edges {
                    if !visited.contains(&edge.0) {
                        foo(&garden, edge.0, *plant, &mut visited, &mut graph, &direction_map);
                    }
                }
                let mut visited = HashSet::new();
                let mut p = 0;
                for node in graph.iter() {
                    if !visited.contains(&(Orientation::Horizontal, *node.0)) {
                        p += bar(*node.0, &graph, &mut visited, Orientation::Horizontal);
                    }
                    if !visited.contains(&(Orientation::Vertical, *node.0)) {
                        p += bar(*node.0, &graph, &mut visited, Orientation::Vertical);
                    }
                    // corner case (literally we have 2 adjacent corners) where we'll end up missing 2 sides
                    if node.1.next.len() == 4 {
                        p += 2;
                    }
                }
                total_price += p*a;
            }
        }
    }
    submit!(2, total_price);
}

#[derive(Debug)]
struct Node {
    position: (isize, isize),
    next: Vec<(Orientation, (isize,isize))>,
}

fn foo(garden: &HashMap<(isize, isize), char>, current: (isize, isize), plant: char, visited: &mut HashSet<(isize, isize)>, graph: &mut HashMap<(isize, isize), Node>, direction_map: &HashMap<Direction, (isize, isize)>) {
    visited.insert(current);
    for direction in [Direction::Right, Direction::Down, Direction::Left, Direction::Up].iter() {
        let v: &(isize, isize) = direction_map.get(direction).unwrap();
        let new_location = (current.0 + v.0, current.1 + v.1);
        let new_plant = garden.get(&new_location);
        if !(new_plant.is_some() && *new_plant.unwrap() == plant) {
            build_edge_nodes(current, direction, graph);
        }
    }
}

fn bar(start: (isize, isize), graph: &HashMap<(isize, isize), Node>, visited: &mut HashSet<(Orientation, (isize, isize))>, orientation: Orientation) -> usize {
    let node = graph.get(&start).unwrap();
    let mut has_edge = 0;
    for next in &node.next {
        if next.0 == orientation {
            if visited.contains(next) {
                continue;
            }
            visited.insert(*next);
            has_edge = 1;
            bar(next.1, graph, visited, orientation);
        }
    }
    has_edge
}


fn travel_two(garden: &HashMap<(isize, isize), char>, current: (isize, isize), plant: char, visited: &mut HashSet<(isize, isize)>, edges: &mut HashMap<(isize, isize), char>) -> usize {
    visited.insert(current);
    let mut a = 1;
    for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
        let new_location = (current.0 + v.0, current.1 + v.1);
        let new_plant = garden.get(&new_location);
        if new_plant.is_some() && *new_plant.unwrap() == plant {
            if !visited.contains(&new_location) {
                let result = travel_two(garden, new_location, plant, visited, edges);
                a += result;
            }
        } else {
            edges.insert(current, plant);
        }
    }
    a
}


fn build_edge_nodes(current: (isize, isize), direction: &Direction, graph: &mut HashMap<(isize, isize), Node>) {
    if *direction == Direction::Up {
        let x = (current.0, current.1);
        let y =  (current.0, current.1+1);
        let a = graph.entry(x).or_insert(Node {
            position: x,
            next: Vec::new(),
        });
        // println!("a: {a:?}");
        a.next.push((Orientation::Horizontal, y));
        // println!("a: {a:?}");
        let b = graph.entry(y).or_insert(Node {
            position: y,
            next: Vec::new(),
        });
        // println!("b: {b:?}");
        b.next.push((Orientation::Horizontal, x));
        // println!("b: {b:?}");
    } else if *direction == Direction::Right {
        let x = (current.0, current.1+1);
        let y =  (current.0+1, current.1+1);
        let a = graph.entry(x).or_insert(Node {
            position: x,
            next: Vec::new(),
        });
        a.next.push((Orientation::Vertical,y));
        let b = graph.entry(y).or_insert(Node {
            position: y,
            next: Vec::new(),
        });
        b.next.push((Orientation::Vertical,x));
    } else if *direction == Direction::Down {
        let x = (current.0+1, current.1+1);
        let y =  (current.0+1, current.1);
        let a = graph.entry(x).or_insert(Node {
            position: x,
            next: Vec::new(),
        });
        a.next.push((Orientation::Horizontal,y));
        let b = graph.entry(y).or_insert(Node {
            position: y,
            next: Vec::new(),
        });
        b.next.push((Orientation::Horizontal,x));
    } else if *direction == Direction::Left {
        let x = (current.0+1, current.1);
        let y =  (current.0, current.1);
        let a = graph.entry(x).or_insert(Node {
            position: x,
            next: Vec::new(),
        });
        a.next.push((Orientation::Vertical,y));
        let b = graph.entry(y).or_insert(Node {
            position: y,
            next: Vec::new(),
        });
        b.next.push((Orientation::Vertical,x));
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}