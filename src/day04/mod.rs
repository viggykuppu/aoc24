use std::collections::HashSet;
use aocd::*;

#[aocd(2024, 4)]
pub fn one() {
    let input = input!();
    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut xs = HashSet::<(isize, isize)>::new();
    for i in 0..grid.len() {
        for j in 0..grid.first().unwrap().len() {
            let c = grid.get(i).unwrap().get(j).unwrap();
            if *c == 'X' {
                xs.insert((i as isize,j as isize));
            }
        }
    }
    let bounds = (grid.len() as isize, grid.first().unwrap().len() as isize);
    let total_xmases: u32 = xs.iter().map(|start| {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    count += find(&grid, *start, bounds, 'X', (i,j));
                }
            }
        }
        count
    }).sum();

    submit!(1, total_xmases);
}

#[aocd(2024, 4)]
pub fn two() {
    let input = input!();
    let grid = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    // Find all the As and then check the corners to see if they make MAS-es
    let mut starting_as = HashSet::<(isize, isize)>::new();
    for i in 0..grid.len() {
        for j in 0..grid.first().unwrap().len() {
            let c = grid.get(i).unwrap().get(j).unwrap();
            if *c == 'A' {
                starting_as.insert((i as isize,j as isize));
            }
        }
    }
    let bounds = (grid.len() as isize, grid.first().unwrap().len() as isize);
    let total_xmases: u32 = starting_as.iter().map(|start| {
        let top_left = (start.0-1, start.1-1);
        let bottom_right = (start.0+1, start.1+1);
        let top_right = (start.0-1, start.1+1);
        let bottom_left = (start.0+1, start.1-1);

        if check_bounds(top_left, bounds) && check_bounds(bottom_right, bounds) {
            let tl = *grid.get(top_left.0 as usize).unwrap().get(top_left.1 as usize).unwrap();
            let br = *grid.get(bottom_right.0 as usize).unwrap().get(bottom_right.1 as usize).unwrap();
            if check_bounds(top_left, bounds) && check_bounds(bottom_right, bounds) {
                let tr = *grid.get(top_right.0 as usize).unwrap().get(top_right.1 as usize).unwrap();
                let bl = *grid.get(bottom_left.0 as usize).unwrap().get(bottom_left.1 as usize).unwrap();
                if (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M') ||
                    (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M') {
                    return 1;
                }   
            }
        }
        0
    }).sum();

    submit!(2, total_xmases);
}

fn find(grid: &Vec<Vec<char>>, start: (isize, isize), bounds: (isize, isize), current: char, direction: (isize, isize)) -> u32 {
    let next = if current == 'X' {
        'M'
    } else if current == 'M' {
        'A'
    } else {
        'S'
    };
    let mut count = 0;
    let new_pos = (start.0 + direction.0, start.1 + direction.1);
    if check_bounds(new_pos, bounds) {
        let next_char = grid.get(new_pos.0 as usize).unwrap().get(new_pos.1 as usize).unwrap();
        if *next_char == next {
            if next == 'S' {
                count += 1;
            } else {
                count += find(grid, new_pos, bounds, next, direction);
            }
        }
    }
    count
}

fn check_bounds(pos: (isize, isize), bounds: (isize, isize)) -> bool {
    (0..bounds.0).contains(&pos.0) && (0..bounds.1).contains(&pos.1)
}