use std::fmt;
use itertools::Itertools;

advent_of_code::solution!(7);

struct Computation {
    result: u64,
    values: Vec<u64>

}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
    Concat
}

impl Computation {
    pub fn is_solved_by(&self, ops: &Vec<Op>) -> bool {
        let mut value = self.values[0];
        for (next, op) in self.values[1..].iter().zip(ops) {
            match op {
                Op::Mul => value *= next,
                Op::Add => value += next,
                Op::Concat => {
                    let s_value = value.to_string() + &next.to_string();
                    value = s_value.parse::<u64>().unwrap();
                }
            }
            if value > self.result {
                return false;
            }
        }
        value == self.result
    }
}

impl From<&str> for Computation {
    fn from(line: &str) -> Self {
        let parts: Vec<&str> = line.split(':').collect();
        let result = parts[0].parse::<u64>().unwrap();
        let values: Vec<u64> = parts[1].split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();

        Computation {
            result,
            values
        }
    }
}

impl fmt::Display for Computation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.result, self.values)
    }
}

fn generate_ops_combinations(ops: &[Op], size: usize) -> Vec<Vec<Op>> {
    if size == 0 {
        return vec![vec![]];
    }

    let mut combinations = Vec::new();
    for &op in ops {
        // Generate smaller combinations
        let smaller_combinations = generate_ops_combinations(ops, size - 1);
        for mut smaller in smaller_combinations {
            // Prepend the current value
            smaller.insert(0, op);
            combinations.push(smaller);
        }
    }

    combinations
}

fn solve_computations(computations: &Vec<Computation>, available_ops: Vec<Op>) -> Option<u64> {
    let mut count: u64 = 0;
    for computation in computations {
        for to_solve in generate_ops_combinations(&available_ops, computation.values.len() - 1) {
            if computation.is_solved_by(&to_solve) {
                count += computation.result;
                break
            }
        }
    }
    Some(count)
}

pub fn part_one(input: &str) -> Option<u64> {
    let computations: Vec<Computation> = input.lines().map(|line| Computation::from(line)).collect();
    solve_computations(&computations, vec![Op::Mul, Op::Add])
}

pub fn part_two(input: &str) -> Option<u64> {
    let computations: Vec<Computation> = input.lines().map(|line| Computation::from(line)).collect();
    solve_computations(&computations, vec![Op::Mul, Op::Add, Op::Concat])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
