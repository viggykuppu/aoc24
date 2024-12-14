use aocd::*;
use nalgebra::{Matrix2, Matrix2x1};
use regex::Regex;

#[aocd(2024, 13)]
pub fn one() {
    let input = input!();
    let regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    let total_cost: u64 = regex.captures_iter(&input).map(|cap| {
        let x1 = cap.get(1).unwrap().as_str().parse::<f64>().unwrap();
        let y1 = cap.get(3).unwrap().as_str().parse::<f64>().unwrap();
        let x2 = cap.get(2).unwrap().as_str().parse::<f64>().unwrap();
        let y2 = cap.get(4).unwrap().as_str().parse::<f64>().unwrap();
        let p_x = cap.get(5).unwrap().as_str().parse::<f64>().unwrap();
        let p_y = cap.get(6).unwrap().as_str().parse::<f64>().unwrap();
        let m1 = Matrix2::new(x1, y1, x2, y2);
        let m2 = Matrix2x1::new(p_x, p_y);
        let c = m1.try_inverse().unwrap()*m2;
        // println!("solution? {c:?}");
        let a = c[0];
        let b = c[1];
        if (a - a.round()).abs() < 0.01 && (b - b.round()).abs() < 0.01 {
            // println!("solution found a {a} and b {b}");
            return 3*(a.round() as u64) + (b.round() as u64);
        }
        0
    }).sum();
    submit!(1, total_cost);
}

#[aocd(2024, 13)]
pub fn two() {
    let input = input!();
    let regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    let total_cost: u64 = regex.captures_iter(&input).map(|cap| {
        let x1 = cap.get(1).unwrap().as_str().parse::<f64>().unwrap();
        let y1 = cap.get(3).unwrap().as_str().parse::<f64>().unwrap();
        let x2 = cap.get(2).unwrap().as_str().parse::<f64>().unwrap();
        let y2 = cap.get(4).unwrap().as_str().parse::<f64>().unwrap();
        let p_x = cap.get(5).unwrap().as_str().parse::<f64>().unwrap() + 10000000000000_f64;
        let p_y = cap.get(6).unwrap().as_str().parse::<f64>().unwrap() + 10000000000000_f64;
        let m1 = Matrix2::new(x1, y1, x2, y2);
        let m2 = Matrix2x1::new(p_x, p_y);
        let c = m1.try_inverse().unwrap()*m2;
        // println!("solution? {c:?}");
        let a = c[0];
        let b = c[1];
        if (a - a.round()).abs() < 0.01 && (b - b.round()).abs() < 0.01 {
            // println!("solution found a {a} and b {b}");
            return 3*(a.round() as u64) + (b.round() as u64);
        }
        0
    }).sum();
    submit!(2, total_cost);
}