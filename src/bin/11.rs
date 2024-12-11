advent_of_code::solution!(11);

use std::collections::HashMap;
use rayon::prelude::*;

fn parse_stones(input: &str) -> Vec<u64> {
    let stones: Vec<u64> = input.split_whitespace().flat_map(str::parse::<u64>).collect();
    stones
}

fn blink_stone(stone: u64, n: i32, max_blinks: i32, cache: &mut HashMap<(u64, i32), u64>) -> u64 {
    if let Some(&n_stones) = cache.get(&(stone, n)) {
        return n_stones;
    }

    if n == max_blinks {
        return 1;
    }

    let n_stones = {
        if stone == 0 {
            blink_stone(1, n + 1, max_blinks, cache)
        } else {
            let num_digits = stone.ilog10() + 1;
            if num_digits % 2 == 0 {
                let divisor = 10_u64.pow(num_digits / 2);
                let (left, right) = (stone / divisor, stone % divisor);
                blink_stone(left, n + 1, max_blinks, cache) + blink_stone(right, n + 1, max_blinks, cache)
            } else {
                blink_stone(stone * 2024, n + 1, max_blinks, cache)
            }
        }
    };
    cache.insert((stone, n), n_stones);
    n_stones
}

fn get_number_of_stones_after_blinking(input: &str, n_blinks: i32) -> Option<u64> {
    let stones: Vec<u64> = parse_stones(input);

    let count: u64 = stones
        .into_par_iter()
        .map(|stone| {
            let mut cache: HashMap<(u64, i32), u64> = HashMap::new();
            blink_stone(stone, 0, n_blinks, &mut cache)
        })
        .sum();
    Some(count)
}

pub fn part_one(input: &str) -> Option<u64> {
    get_number_of_stones_after_blinking(input, 25)
}

pub fn part_two(input: &str) -> Option<u64> {
    get_number_of_stones_after_blinking(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
