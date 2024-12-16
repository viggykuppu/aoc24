use std::collections::{HashMap, HashSet};

use aocd::*;
use image::{Rgb, RgbImage};
use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[aocd(2024, 14, "src/day14/input.txt")]
pub fn one() {
    let input = input!();
    let regex = Regex::new(r"p=(\d+),(\d+) v=([\+\-]{0,1}\d+),([\+\-]{0,1}\d+)").unwrap();
    let bounds = (101, 103);
    let mut robots: Vec<Robot> = regex.captures_iter(&input).map(|cap| {
        let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vx = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vy = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
        Robot {
            position: (x, y),
            velocity: (vx, vy),
        }
    }).collect();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position.0 = ((robot.position.0 + robot.velocity.0) % bounds.0 + bounds.0) % bounds.0;
            robot.position.1 = ((robot.position.1 + robot.velocity.1) % bounds.1 + bounds.1) % bounds.1;
        }
    }
    let quad_bounds = (bounds.0/2, bounds.1/2);
    let mut quads= HashMap::new();
    for robot in robots.iter() {
        if robot.position.0 == quad_bounds.0 || robot.position.1 == quad_bounds.1 {
            continue;
        }
        let quad_score = (!(0..quad_bounds.0).contains(&robot.position.0) as i32, !(0..quad_bounds.1).contains(&robot.position.1) as i32);
        let num_robots_in_quad = quads.entry(quad_score).or_insert(0);
        *num_robots_in_quad += 1;
    }
    let safety_factor = quads.values().product::<i32>();
    submit!(1, safety_factor);
}

#[aocd(2024, 14, "src/day14/input.txt")]
pub fn two() {
    let input = input!();
    let regex = Regex::new(r"p=(\d+),(\d+) v=([\+\-]{0,1}\d+),([\+\-]{0,1}\d+)").unwrap();
    let bounds = (101, 103);
    let mut robots: Vec<Robot> = regex.captures_iter(&input).map(|cap| {
        let x = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vx = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vy = cap.get(4).unwrap().as_str().parse::<i32>().unwrap();
        Robot {
            position: (x, y),
            velocity: (vx, vy),
        }
    }).collect();
    let mut adjacency_score = 0;
    let mut christmas_tree_index = 0;
    while adjacency_score < 300 {
        adjacency_score = 0;
        let mut positions = HashSet::new();
        for robot in robots.iter_mut() {
            robot.position.0 = ((robot.position.0 + robot.velocity.0) % bounds.0 + bounds.0) % bounds.0;
            robot.position.1 = ((robot.position.1 + robot.velocity.1) % bounds.1 + bounds.1) % bounds.1;
            positions.insert(robot.position);
            for v in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
                let test_position = (robot.position.0 + v.0, robot.position.1 + v.1);
                if positions.contains(&test_position) {
                    adjacency_score += 1;
                }
            }
        }
        christmas_tree_index += 1;
    }
    pretty_print_robots_to_image(&robots, &bounds);
    submit!(2, christmas_tree_index);
}

fn pretty_print_robots(robots: &[Robot], bounds: &(i32, i32)) {
    let mut map = HashMap::new();
    for robot in robots.iter() {
        let v = map.entry(robot.position).or_insert(0);
        *v += 1;
    }
    for i in 0..bounds.1 {
        for j in 0..bounds.0 {
            if let Some(v) = map.get(&(j,i)) {
                print!("{v}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn pretty_print_robots_to_image(robots: &[Robot], bounds: &(i32, i32)) {
    let mut map = HashSet::new();
    for robot in robots.iter() {
        map.insert(robot.position);
    }
    let mut img = RgbImage::new(bounds.0 as u32, bounds.1 as u32);
    for i in 0..bounds.0 {
        for j in 0..bounds.1 {
            if map.contains(&(i,j)) {
                img.put_pixel(i as u32, j as u32, Rgb([255,255,255]));
            }
        }
    }
    img.save("src/day14/christmas_tree.png").unwrap();
}