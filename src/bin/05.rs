advent_of_code::solution!(5);

use std::collections::{HashMap};

fn get_pages(pages_str: &str) -> HashMap<i32, Vec<i32>> {
    let pages: Vec<(i32, i32)> = pages_str.lines().map(|line| {
        let l_low_high: Vec<&str> = line.split("|").collect();
        (l_low_high[0].parse::<i32>().ok().unwrap(), l_low_high[1].parse::<i32>().ok().unwrap())
    }).collect();

    let mut hashmap = HashMap::<i32, Vec<i32>>::new();
    for (low, high) in pages {
        hashmap.entry(low).or_insert_with(Vec::new).push(high);
        hashmap.entry(high).or_insert_with(Vec::new);
    }
    hashmap
}

fn is_valid_update(pages: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    for i in 0..update.len() - 1 {
        let (low, high) = (update[i], update[i + 1]);
        if !pages[&low].contains(&high) {
            return false;
        }
    }
    true
}

fn fix_update(pages: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut update_pages_length = Vec::<(i32, usize)>::new();
    for v in update.iter() {
        let mut selection = pages[v].clone();
        selection.retain(|item| update.contains(item));
        update_pages_length.push((*v, selection.len()));
    }
    update_pages_length.sort_by(|a, b| b.1.cmp(&a.1));
    let fixed_update: (Vec<i32>, Vec<usize>) = update_pages_length.into_iter().unzip();
    fixed_update.0
}

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let pages: HashMap<i32, Vec<i32>> = get_pages(parts[0]);
    let mut sum: u32 = 0;
    for line in parts[1].lines() {
        let update = line.split(",").flat_map(str::parse::<i32>).collect();
        if is_valid_update(&pages, &update) {
            sum += update[update.len() / 2] as u32;
        };
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let pages: HashMap<i32, Vec<i32>> = get_pages(parts[0]);
    let mut sum: u32 = 0;
    for line in parts[1].lines() {
        let update = line.split(",").flat_map(str::parse::<i32>).collect();
        if !is_valid_update(&pages, &update) {
            let fixed_update = fix_update(&pages, &update);
            sum += fixed_update[fixed_update.len() / 2] as u32;
        };
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
