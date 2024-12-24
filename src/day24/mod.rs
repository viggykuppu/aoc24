use std::collections::{HashMap, HashSet};

use aocd::*;
use regex::Regex;

#[aocd(2024, 24)]
pub fn one() {
    let input = input!();
    let mut outputs = HashMap::new();
    let initial_value_regex = Regex::new(r"(.*): (\d)").unwrap();
    let value_regex = Regex::new(r"(.+) (AND|XOR|OR) (.+) -> (.+)").unwrap();
    let mut gates = Vec::new();
    let mut z_bits = 0; 
    input.lines().for_each(|line| {
        if let Some(caps) =  initial_value_regex.captures(line) {
            let label = caps.get(1).unwrap().as_str();
            let initial_value = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
            outputs.insert(label, initial_value);
        } else if let Some(caps) = value_regex.captures(line) {
            let lhs = caps.get(1).unwrap().as_str();
            let op_string: &str = caps.get(2).unwrap().as_str();
            let rhs = caps.get(3).unwrap().as_str();
            let output = caps.get(4).unwrap().as_str();
            let gate = Gate {
                lhs,
                rhs,
                output,
                op_string,
            };
            gates.push(gate);
            if output.contains('z') {
                z_bits += 1;
            }
        }
    });
    evaluate_gates(&gates, &mut outputs);
    let z = convert_output_to_decimal("z", &outputs);
    submit!(1, z);
}

#[derive(Debug, Clone, Copy)]
struct Gate<'a> {
    lhs: &'a str,
    rhs: &'a str,
    output: &'a str,
    op_string: &'a str,
}

impl Gate<'_> {
    fn calculate(&self, outputs: &HashMap<&str, u8>) -> u8 {
        let lhs_value = outputs[self.lhs];
        let rhs_value = outputs[self.rhs];
        let mut output: u8 = 0;
        if self.op_string == "AND" {
            output = lhs_value & rhs_value;
        } else if self.op_string == "OR" {
            output = lhs_value | rhs_value;
        } else if self.op_string == "XOR" {
            output = lhs_value ^ rhs_value;
        }
        output
    }
}

#[aocd(2024, 24, "src/day24/input.txt")]
pub fn two() {
    let input = input!();
    let mut outputs = HashMap::new();
    let initial_value_regex = Regex::new(r"(.*): (\d)").unwrap();
    let value_regex = Regex::new(r"(.+) (AND|XOR|OR) (.+) -> (.+)").unwrap();
    let mut gates = Vec::new();
    let mut gates_map = HashMap::new();
    let mut z_bits = 0; 
    input.lines().for_each(|line| {
        if let Some(caps) =  initial_value_regex.captures(line) {
            let label = caps.get(1).unwrap().as_str();
            let initial_value = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
            outputs.insert(label, initial_value);
        } else if let Some(caps) = value_regex.captures(line) {
            let lhs = caps.get(1).unwrap().as_str();
            let op_string: &str = caps.get(2).unwrap().as_str();
            let rhs = caps.get(3).unwrap().as_str();
            let output = caps.get(4).unwrap().as_str();
            let gate = Gate {
                lhs,
                rhs,
                output,
                op_string,
            };
            gates.push(gate);
            gates_map.insert(gate.output, gate);
            if output.contains('z') {
                z_bits += 1;
            }
        }
    });
    // let x = 1;
    // let y = 1;
    // let x_string = format!("{:#045b}", x);
    // let y_string = format!("{:#045b}", y);
    // println!("{x_string}\n{y_string}");
    // set_input("x", &x_string, &mut outputs);
    // set_input("y", &x_string, &mut outputs);
    // let out = get_base_wires("z17", &gates_map);
    // println!("{out:?}");
    for i in 0..46 {
        let k = format!("z{}",convert_i_to_index_string(i));
        print_dependencies(&k, &gates_map, 2);
        println!();
    }
    println!();
    evaluate_gates(&gates, &mut outputs);
    let x = convert_output_to_decimal("x", &outputs);
    let y = convert_output_to_decimal("y", &outputs);
    let a = x + y;
    let z = convert_output_to_decimal("z", &outputs);
    println!("{:#045b}\n{:#045b}", a,z);
    let mut x = vec!["dkr","z05","htp","hhh","ggk","z15","z20","rhv"];
    x.sort();
    let z = x.join(",");
    submit!(2, z);
}

fn set_input(prefix: &str, binary_string: &str, outputs: &mut HashMap<&str, u8>) {
    binary_string.chars().skip(3).enumerate().for_each(|(i, c)| {
        let index = 45 - i;
        let mut index_string = index.to_string();
        if index_string.len() == 1 {
            index_string = format!("0{index_string}");
        }
        let key_string = format!("{prefix}{index_string}");
        let key = key_string.as_str();
        println!("{key}");
        if let Some(v) = outputs.get_mut(key) {
            *v = c.to_string().parse::<u8>().unwrap();
        }
    })
}

fn convert_output_to_decimal(prefix: &str, outputs: &HashMap<&str, u8>) -> u64 {
    let mut decimal_value = 0;
    let mut i = 0;
    loop {
        let i_string = convert_i_to_index_string(i);
        let key_string = format!("{prefix}{i_string}");
        let key = key_string.as_str();
        if let Some(output) = outputs.get(&key) {
            decimal_value += 2_u64.pow(i)*(*output as u64);
        } else {
            break;
        }
        i += 1;
    }
    return decimal_value;
}

fn convert_i_to_index_string(i: u32) -> String {
    let mut i_string = i.to_string();
    if i_string.len() == 1 {
        i_string = format!("0{i_string}");
    }
    return i_string;
}

fn print_dependencies(output: &str, gates_map: &HashMap<&str, Gate>, depth: usize) {
    if depth == 0 || output.contains('x') || output.contains('y') {
        print!("{output}")
    } else {
        print!("[{output} = ");
        let gate = gates_map.get(output).unwrap();
        print_dependencies(gate.lhs, gates_map, depth - 1);
        let op = gate.op_string;
        print!(" {op} ");
        print_dependencies(gate.rhs, gates_map, depth - 1);
        print!("]");
    }

}

fn get_base_wires<'a>(output: &'a str, gates_map: &'a HashMap<&'a str, Gate>) -> Vec<&'a str> {
    let mut deps = Vec::new();
    if output.contains('x') || output.contains('y') {
        deps.push(output);
    } else {    
        let gate = gates_map.get(output).unwrap();
        deps = [get_base_wires(gate.lhs, gates_map), get_base_wires(gate.rhs, gates_map)].concat();
    }
    deps
}


fn evaluate_gates<'a>(gates: &'a [Gate], outputs: &mut HashMap<&'a str, u8>) {
    let mut gates_to_evaluate: HashSet<_> = (0..gates.len()).into_iter().collect();
    loop {
        let mut gates_to_remove = HashSet::new(); 
        for gate_id in gates_to_evaluate.iter() {
            let gate = &gates[*gate_id];
            if outputs.get(gate.lhs).is_some() && outputs.get(gate.rhs).is_some() {
                outputs.insert(gate.output, gate.calculate(&outputs));
                gates_to_remove.insert(*gate_id);
            }
        }
        for gate_id in gates_to_remove {
            gates_to_evaluate.remove(&gate_id);
        }
        if gates_to_evaluate.len() == 0 {
            break;
        }
    }
}