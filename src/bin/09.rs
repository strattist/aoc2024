advent_of_code::solution!(9);

use std::fmt;
use std::ops::Mul;

#[derive(Clone, Copy)]
enum Item {
    Empty,
    File(i32)
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Empty => write!(f, "."),
            Item::File(value) => write!(f, "{}", value),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Empty => write!(f, "."),
            Item::File(value) => write!(f, "{}", value),
        }
    }
}

impl Mul<&Item> for usize {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: &Item) -> Self {
        match rhs {
            Item::Empty => 0,
            Item::File(value) => self * (*value as usize),
        }
    }
}

impl Item {
    pub fn is_empty(&self) -> bool {
        match self {
            Item::Empty => true,
            _ => false
        }
    }
}

fn get_disk_layout(input: &str) -> Vec<Item> {
    let disk = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .fold(
            Vec::<Item>::new(),
            |mut d, (i, c)| {
                if i % 2 == 0 {
                    let chars = vec![Item::File(i as i32 / 2); c.to_digit(10).unwrap() as usize];
                    d.extend(chars);
                } else {
                    let items = vec![Item::Empty; c.to_digit(10).unwrap() as usize];
                    d.extend(items);
                }
                d
            }
        );
    disk
}

fn compute_checksum(disk: &Vec<Item>) -> u128 {
    let count: u128 = disk
        .iter()
        .enumerate()
        .map(|(i, item)| (i * item) as u128)
        .sum();
    count
}

fn reorganise_disk_single_files(disk: &mut Vec<Item>) {
    let mut first_empty = disk.iter().position(|&item| item.is_empty()).unwrap();
    let mut last_file  = disk.len() - disk.iter().rev().position(|&item| !item.is_empty()).unwrap() - 1;
    while first_empty < last_file {
        disk.swap(first_empty, last_file);
        first_empty = disk.iter().position(|&item| item.is_empty()).unwrap();
        last_file  = disk.len() - disk.iter().rev().position(|&item| !item.is_empty()).unwrap() - 1;
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut disk = get_disk_layout(input);
    reorganise_disk_single_files(&mut disk);
    let checksum = compute_checksum(&disk);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
