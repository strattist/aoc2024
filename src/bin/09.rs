advent_of_code::solution!(9);

use std::cmp::{min, max};

type Item = (i32, i32);

fn get_disk_layout(input: &str) -> Vec<Item> {
    let disk = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .fold(
            Vec::<Item>::new(),
            |mut d, (i, c)| {
                if i % 2 == 0 {
                    d.push((c.to_digit(10).unwrap() as i32, i as i32 / 2));
                } else {
                    d.push((c.to_digit(10).unwrap() as i32, 0));
                }
                d
            }
        );
        disk
}

fn compute_checksum(disk: &Vec<Item>) -> u128 {
    let oneline_disk: Vec<i32> = disk
        .iter()
        .fold(
            Vec::<i32>::new(),
            |mut d, (n, id)| {
                let v: Vec<i32> = vec![*id; *n as usize].to_vec();
                d.extend(v);
                d
    });
    let count: u128 = oneline_disk
        .iter()
        .enumerate()
        .map(|(i, value)| (i as i32 * value) as u128)
        .sum();
    count
}

fn reorganise_disk_into_single_files(disk: &Vec<Item>) -> Vec<Item> {
    let mut cloned_disk = disk.clone();
    let mut disk_single_files: Vec<Item> = Vec::new();
    disk_single_files.push(cloned_disk[0]);
    cloned_disk.remove(0);

    while !cloned_disk.is_empty() {
        if cloned_disk[0].0 > 0 {
            let mut n_remaining_spaces = cloned_disk[0].0;
            let mut to_insert: Vec<Item> = Vec::new();
            while n_remaining_spaces > 0 {
                if let Some(found) = cloned_disk.iter().rev().position(|item| item.1 != 0) {
                    let p: usize = cloned_disk.len() - found - 1;
                    let last_file = cloned_disk[p];
                    if last_file.0 > n_remaining_spaces {
                        cloned_disk[p].0 -= n_remaining_spaces;
                        to_insert.push((n_remaining_spaces, cloned_disk[p].1));
                    } else {
                        to_insert.push(cloned_disk[p]);
                        cloned_disk.remove(p);
                    }
                    n_remaining_spaces -= to_insert.last().unwrap().0;
                } else {
                    break;
                }
            }
            disk_single_files.extend(to_insert);
        }
        if let Some(item) = cloned_disk.get(1) {
            if item.1 != 0 {
                disk_single_files.push(*item);
            }
        }

        cloned_disk.drain(0..(min(2, cloned_disk.len())));
    }
    disk_single_files
}

fn reorganise_disk_into_block_files(disk: &Vec<Item>) -> Vec<Item> {
    let mut cloned_disk = disk.clone();
    let mut disk_block_files: Vec<Item> = Vec::new();
    disk_block_files.push(cloned_disk[0]);
    cloned_disk.remove(0);

    while !cloned_disk.is_empty() {
        if cloned_disk[0].1 == 0 {
            if cloned_disk[0].0 > 0 {
                let mut n_remaining_spaces = cloned_disk[0].0;
                let mut to_insert: Vec<Item> = Vec::new();
                while n_remaining_spaces > 0 {
                    if let Some(found) = cloned_disk.iter().rev().position(|item| item.1 != 0 && item.0 <= n_remaining_spaces) {
                        let p: usize = cloned_disk.len() - found - 1;
                        to_insert.push(cloned_disk[p]);
                        cloned_disk[p].1 = 0;
                        n_remaining_spaces -= to_insert.last().unwrap().0;
                    } else {
                        to_insert.push((n_remaining_spaces, 0));
                        n_remaining_spaces -= n_remaining_spaces;
                    }
                }
                disk_block_files.extend(to_insert);
            }
        } else {
            disk_block_files.push(cloned_disk[0]);
        }
        cloned_disk.remove(0);
        // And remove last items that are empty space
        if let Some(found) = cloned_disk.iter().rev().position(|item| item.1 != 0) {
            let p: usize = cloned_disk.len() - found;
            cloned_disk.drain(p..cloned_disk.len());
        }
    }
    disk_block_files
}

pub fn part_one(input: &str) -> Option<u128> {
    let disk = get_disk_layout(input);
    let disk_single = reorganise_disk_into_single_files(&disk);
    let checksum = compute_checksum(&disk_single);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u128> {
    let disk = get_disk_layout(input);
    let disk_single = reorganise_disk_into_block_files(&disk);
    let checksum = compute_checksum(&disk_single);
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
