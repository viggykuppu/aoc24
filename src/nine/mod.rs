use std::collections::{BinaryHeap, HashSet};

use aocd::*;

#[aocd(2024, 9)]
pub fn one() {
    let input = input!();
    let mut index: u64 = 0;
    let mut disk: Vec<Option<u64>> = Vec::new();
    let mut total_blocks: usize = 0;
    input.chars().enumerate().for_each(|(i, char)| {
        let num_blocks = char.to_digit(10).unwrap();
        let new_block = if i %2 == 0 {
            // file
            index += 1;
            total_blocks += num_blocks as usize;
            Some(index-1)
        } else {
            // free space
            None
        };
        for _ in 0..num_blocks {
            disk.push(new_block);
        }
    });
    let mut last_moved_to_index = 0;
    let mut condensed_disk: Vec<u64> = Vec::new();
    for i in 0..disk.len() {
        let j = disk.len() - (i+1);
        if total_blocks == condensed_disk.len() {
            break;
        }
        let value_to_be_moved = disk[j];
        if let Some(value_to_be_moved) = value_to_be_moved {
            loop {
                let potential_move_spot = disk[last_moved_to_index];
                if let Some(move_spot_value) =  potential_move_spot {
                    condensed_disk.push(move_spot_value);
                } else {
                    condensed_disk.push(value_to_be_moved);
                    last_moved_to_index += 1;
                    break;
                }
                if total_blocks == condensed_disk.len() {
                    break;
                }
                last_moved_to_index += 1;
            }
        }
    }
    let checksum: u64 = condensed_disk.iter().enumerate().map(|(i, n)| {
        return ((i as u64)*n) as u64;
    }).sum();
    submit!(1, checksum);
}

#[aocd(2024, 9)]
pub fn two() {
    let input = input!();
    let mut index: u64 = 0;
    let mut disk: Vec<Option<u64>> = Vec::new();
    input.chars().enumerate().for_each(|(i, char)| {
        let num_blocks = char.to_digit(10).unwrap();
        let new_block = if i %2 == 0 {
            // file
            index += 1;
            Some(index-1)
        } else {
            // free space
            None
        };
        for _ in 0..num_blocks {
            disk.push(new_block);
        }
    });

    let mut disk_reverse = disk.clone();
    disk_reverse.reverse();
    let mut condensed_disk: Vec<Option<u64>> = Vec::new();
    let mut i = 0;
    let mut moved_blocks: HashSet<u64> = HashSet::new();
    while condensed_disk.len() < disk.len() {
        if condensed_disk.len() == disk.len() {
            break;
        }
        let block = disk[i];
        let mut full_block = get_full_block(&disk, i);
        if let Some(v) = block {
            // this is an already placed block so let's add it to our list unless we've already moved it
            i += full_block.len();
            if moved_blocks.contains(&v) {
                for _ in 0..full_block.len() {
                    condensed_disk.push(None);
                }
            } else {
                condensed_disk.append(&mut full_block);
            }
        } else {
            // free block
            let mut j = 0;
            let mut added_block = false;
            while j < disk_reverse.len() {
                if disk.len() - (j) < i {
                    break;
                }
                let mut full_reverse_block = get_full_block(&disk_reverse, j);
                let full_reverse_block_length = full_reverse_block.len();
                if let Some(v) = disk_reverse[j] {
                    if !moved_blocks.contains(&v) {
                        if full_reverse_block.len() <= full_block.len() {
                            moved_blocks.insert(v);
                            i += full_reverse_block.len();
                            condensed_disk.append(&mut full_reverse_block);
                            added_block = true;
                            break;
                        }
                    }
                }
                j += full_reverse_block_length;
            }
            if !added_block {
                i += full_block.len();
                for _ in 0..full_block.len() {
                    condensed_disk.push(None);
                }
            }
        }
    }
    let checksum: u64 = condensed_disk.iter().enumerate().map(|(i, n)| {
        return ((i as u64)*(n.unwrap_or(0))) as u64;
    }).sum();

    submit!(2, checksum);
}

#[aocd(2024, 9)]
pub fn two_clean() {
    let input = input!();
    let mut index: u64 = 0;
    let mut disk: Vec<Option<u64>> = Vec::new();
    input.chars().enumerate().for_each(|(i, char)| {
        let num_blocks = char.to_digit(10).unwrap();
        let new_block = if i %2 == 0 {
            // file
            index += 1;
            Some(index-1)
        } else {
            // free space
            None
        };
        for _ in 0..num_blocks {
            disk.push(new_block);
        }
    });

    let mut disk_reverse = disk.clone();
    disk_reverse.reverse();

    let mut i = 0;
    while i < disk_reverse.len() {
        let full_block = get_full_block(&disk_reverse, i);
        let original_index = disk.len() - (i+full_block.len());
        if let Some(index) = find_space(&disk, full_block.len()) {
            if index < original_index {
                for j in 0..full_block.len() {
                    disk.swap(index + j, original_index + j);
                }
            }
        }
        pretty_print_disk(&disk);
        i += full_block.len();
    }
    let checksum: u64 = disk.iter().enumerate().map(|(i, n)| {
        return ((i as u64)*(n.unwrap_or(0))) as u64;
    }).sum();
    submit!(2, checksum);
}

fn pretty_print_disk(disk: &Vec<Option<u64>>) {
    for x in disk {
        if let Some(a) = x {
            print!("{a}");
        } else {
            print!(".");
        }
    }
    println!("");
}

fn find_space(disk: &Vec<Option<u64>>, space: usize) -> Option<usize> {
    let mut i = 0;
    while i < disk.len() {
        let block_start = disk[i];
        let full_block = get_full_block(disk, i);
        if block_start.is_none() {
            if full_block.len() >= space {
                return Some(i);
            }
        }
        i += full_block.len();
    }
    None
}

fn get_full_block(disk: &Vec<Option<u64>>, index: usize) -> Vec<Option<u64>> {
    let mut full_block: Vec<Option<u64>> = Vec::new();
    let start = disk[index];
    let mut i = index;
    while (0..disk.len()).contains(&i) && disk[i] == start {
        full_block.push(disk[i]);
        i += 1;
    }
    full_block
}