advent_of_code::solution!(3);

use std::num::ParseIntError;

use regex::Regex;


pub fn get_muls(input: &str) -> Vec<Result<(i32, i32), ParseIntError>> {
    let re = Regex::new(r"mul\((?<lhs>\d{1,3}),(?<rhs>\d{1,3})\)").unwrap();
    let results = re
        .captures_iter(input)
        .map(|c| {
            let lhs = c.name("lhs").unwrap().as_str().parse::<i32>();
            let rhs = c.name("rhs").unwrap().as_str().parse::<i32>();
            Ok((lhs?, rhs?))
        })
        .collect();
    results
}

pub fn part_one(input: &str) -> Option<u32> {
    let muls = get_muls(input);
    let sum: u32 = muls
        .iter()
        .map(|values| match values {
            Ok((lhs, rhs)) => (lhs * rhs) as u32,
            Err(_) => 1
        })
        .sum();
    Some(sum)
}

pub fn find_next_do_donts(input: &str, from: usize) ->Option<(usize, usize)> {
    match input[from..].find("do()") {
        Some(i_do) => {
            let real_i_do = i_do + from;
            match input[real_i_do..].find("don't()") {
                Some(i_dont) => Some((real_i_do, i_dont + real_i_do)),
                None => Some((real_i_do, input.len()))
            }
        }
        None => None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut i_dont = input.find("don't()").unwrap();
    let mut sum = part_one(&input[0..i_dont]).unwrap();

    while let Some(next) = find_next_do_donts(input, i_dont) {
        sum += part_one(&input[next.0..next.1]).unwrap();
        i_dont = next.1;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
