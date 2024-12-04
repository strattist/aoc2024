advent_of_code::solution!(2);

pub fn get_input_to_levels(input: &str) -> Vec<Vec<i32>> {
    let levels: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect();
    levels
}

pub fn get_positions_with_errors(a: &Vec<i32>) -> Vec<usize> {
    let mut previous_value = a[0];
    let mut previous_sub_sign = 0;
    let mut positions_with_errors: Vec<usize> = vec![];
    for (pos, value) in a[1..].iter().enumerate() {
        let diff = value - previous_value;
        let sub_sign = if diff < 0 { -1 } else { 1 };
        if diff.abs() < 1 || diff.abs() > 3 {
            positions_with_errors.push(pos + 1);
        } else if previous_sub_sign != sub_sign && previous_sub_sign != 0 {
            positions_with_errors.push(pos + 1);
        } else {
            previous_value = *value;
            previous_sub_sign = sub_sign;
        }
    }
    positions_with_errors
}

pub fn part_one(input: &str) -> Option<u32> {
    let a = get_input_to_levels(input);
    let sum: u32 = a
        .iter()
        .map(|a| (get_positions_with_errors(a).len() == 0) as u32)
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let a = get_input_to_levels(input);
    let mut sum: u32 = 0;
    for aa in a.iter() {
        let mut count_ok = 0;
        for p in 0..aa.len() {
            let mut copy_aa = aa.to_vec();
            copy_aa.remove(p);
            let safe = if get_positions_with_errors(&copy_aa).len() == 0 { true } else { false };
            if safe {
                count_ok += 1
            }
        }
        sum += if count_ok > 0 { 1 } else { 0 };
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
