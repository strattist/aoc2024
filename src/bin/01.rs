advent_of_code::solution!(1);

use std::collections::HashMap;
use std::collections::HashSet;

pub fn get_input_to_sorted_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut a, mut b): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line.split_whitespace().flat_map(str::parse::<i32>);
            Some((nums.next()?, nums.next()?))
        })
        .unzip();
    a.sort();
    b.sort();
    (a, b)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (a, b) = get_input_to_sorted_lists(input);
    let sum_of_diffs: u32 = a
        .iter()
        .zip(b.iter())
        .map(|(va, vb)| (va - vb).abs() as u32)
        .sum();
    Some(sum_of_diffs)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b) = get_input_to_sorted_lists(input);
    let hash_a: HashSet<i32> = a
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let counts_b: HashMap<i32, i32> = b
        .into_iter()
        .fold(
            HashMap::new(),
            |mut acc, vb| {
                *acc.entry(vb).or_insert(0) += 1;
                acc
            }
        );
    let sum_of_mults: i32 = hash_a
        .iter()
        .map(|va| va * counts_b.get(va).unwrap_or(&0))
        .sum();
    Some(sum_of_mults as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
    }
}
