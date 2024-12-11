use aocd::*;
use regex::Regex;

#[aocd(2024, 3)]
pub fn one() {
    let input = input!();
    let number_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let sum: u32 = number_regex.captures_iter(&input).map(|cap| { 
        let x = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        x*y
    }).sum();

    submit!(1, sum);
}

#[aocd(2024, 3)]
pub fn two() {
    let input = input!();
    let number_regex = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
    let mut enabled = true;
    let sum: u32 = number_regex.captures_iter(&input).map(|cap| { 
        if cap.get(3).is_some() {
            enabled = true;
        } else if cap.get(4).is_some() {
            enabled = false;
        } else if enabled {
            let x = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let y = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            return x*y;
        }
        0
    }).sum();

    submit!(2, sum);
}