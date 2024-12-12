use aocd::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
        (i as u64)*n
    }).sum();
    submit!(1, checksum);
}

#[aocd(2024, 9)]
pub fn two() {
    let input = input!();
    let mut block_id: u64 = 0;
    let mut disk: Vec<(Option<u64>, u32, u32)> = Vec::new();
    let mut free_space: Vec<(u32, u32)> = Vec::new();
    let mut location = 0;
    input.chars().enumerate().for_each(|(i, char)| {
        let num_blocks = char.to_digit(10).unwrap();
        if i %2 == 0 {
            disk.push((Some(block_id), num_blocks, location));
            block_id += 1;
        } else {
            disk.push((None, num_blocks, location));
            free_space.push((location, num_blocks));
        };
        location += num_blocks;
    });
    for block in disk.iter_mut().rev() {
        if block.0.is_some() {
            for free in free_space.iter_mut() {
                if block.1 <= free.1 && block.2 > free.0 {
                    block.2 = free.0;
                    free.1 = free.1 - block.1;
                    free.0 = free.0 + block.1;
                    break;
                }
            }
        }
    }
    let checksum: u64 = disk.par_iter().map(|block| {
        if let Some(v) = block.0 {
            let mut total = 0;
            for i in block.2..(block.2 + block.1) {
                total += (i as u64)*v;
            }
            return total;
        }
        0
    }).sum();
    submit!(2, checksum);
}