advent_of_code::solution!(13);

use regex::Regex;
use std::fmt;

type XY = (i128, i128);

struct ClawMachine {
    a: XY,
    b: XY,
    prize: XY,
}

impl ClawMachine {
    pub fn new(block: &Vec<&str>) -> Self {
        assert!(block.len() == 3);
        let re_ab = Regex::new(r"Button .: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
        let re_prize = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
        let a_xy = re_ab.captures(block[0]).unwrap();
        let b_xy = re_ab.captures(block[1]).unwrap();
        let prize_xy = re_prize.captures(block[2]).unwrap();
        ClawMachine {
            a: (a_xy["x"].parse::<i128>().unwrap(), a_xy["y"].parse::<i128>().unwrap()),
            b: (b_xy["x"].parse::<i128>().unwrap(), b_xy["y"].parse::<i128>().unwrap()),
            prize: (prize_xy["x"].parse::<i128>().unwrap(), prize_xy["y"].parse::<i128>().unwrap()),
        }
    }

    pub fn get_n_tokens(&self) -> Option<i128> {
        if let Some((a, b)) = solve_system(
            self.a.0 as f64,
            self.b.0 as f64,
            self.prize.0 as f64,
            self.a.1 as f64,
            self.b.1 as f64,
            self.prize.1 as f64) {
            if a == (a as i64) as f64 && b == (b as i64) as f64 {
                return Some(a as i128 * 3 + b as i128);
            }
        }
        None
    }

    pub fn add_to_prize(&mut self, offset: i128) {
        self.prize = (self.prize.0 + offset, self.prize.1 + offset)
    }
}

/// Solves a system of two linear equations:
/// ax + by = c
/// dx + ey = f
/// Returns (x, y) as Option<(f64, f64)> if a solution exists.
fn solve_system(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Option<(f64, f64)> {
    let denominator = a * e - b * d;

    if denominator.abs() < 1e-10 {
        // The equations are either parallel or coincident, no unique solution.
        return None;
    }

    let x = (c * e - b * f) / denominator;
    let y = (a * f - c * d) / denominator;

    Some((x, y))
}

impl fmt::Display for ClawMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A: X+{} Y+{}\nB: X+{} Y+{}\nPrize: X={}, Y={}", self.a.0, self.a.1, self.b.0, self.b.1, self.prize.0, self.prize.1)
    }
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut lines = input.lines().peekable();
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    while lines.peek().is_some() {
        let block: Vec<&str> =
            lines
                .by_ref()
                .take_while(|l| l.trim().len() > 0).collect();
        claw_machines.push(ClawMachine::new(&block));
    }
    claw_machines
}

pub fn part_one(input: &str) -> Option<u32> {
    let claw_machines = parse_input(input);
    let mut count: u32 = 0;
    for claw_machine in claw_machines {
        if let Some(solution) = claw_machine.get_n_tokens() {
            count += solution as u32;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut claw_machines = parse_input(input);
    let mut count: u128 = 0;
    for claw_machine in claw_machines.iter_mut() {
        claw_machine.add_to_prize(10000000000000);
        if let Some(solution) = claw_machine.get_n_tokens() {
            count += solution as u128;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
