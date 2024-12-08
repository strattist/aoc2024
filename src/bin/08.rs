advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

type Pos = (usize, usize);

fn get_grid_size(input: &str) -> (usize, usize) {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();
    (rows, cols)
}

fn get_antennas(input: &str) -> HashMap<char, Vec<Pos>> {
    let antennas = input
        .lines()
        .enumerate()
        .fold(
            HashMap::<char, Vec<Pos>>::new(),
            |mut antennas, (i, line)| {
                for (j, c) in line.chars().enumerate() {
                    if c != '.' {
                        antennas.entry(c).or_insert_with(Vec::new).push((i, j));
                    }
                }
                antennas
            });
    antennas
}

fn in_grid(size: &(usize, usize), p: &(i32, i32)) -> bool {
    p.0 >= 0 && p.1 >= 0 && p.0 < size.0 as i32 && p.1 < size.1 as i32
}

fn get_single_antinodes(size: &(usize, usize), p0: &(usize, usize), p1: &(usize, usize)) -> Vec<Pos> {
    let diff = (p0.0 as i32 - p1.0 as i32, p0.1 as i32 - p1.1 as i32);
    let mut antinodes: Vec<Pos> = Vec::new();
    let mut p: (i32, i32) = (p0.0 as i32 + diff.0, p0.1 as i32 + diff.1);
    if in_grid(size, &p) {
        antinodes.push((p.0 as usize, p.1 as usize));
    }
    p = (p1.0 as i32 - diff.0, p1.1 as i32 - diff.1);
    if in_grid(size, &p) {
        antinodes.push((p.0 as usize, p.1 as usize));
    }
    antinodes
}

fn get_multi_antinodes(size: &(usize, usize), p0: &(usize, usize), p1: &(usize, usize)) -> Vec<Pos> {
    let diff = (p0.0 as i32 - p1.0 as i32, p0.1 as i32 - p1.1 as i32);
    let mut antinodes: Vec<Pos> = Vec::new();

    let mut p: (i32, i32) = (p0.0 as i32 + diff.0, p0.1 as i32 + diff.1);
    while in_grid(size, &p) {
        antinodes.push((p.0 as usize, p.1 as usize));
        p.0 += diff.0;
        p.1 += diff.1;
    }
    let mut p: (i32, i32) = (p0.0 as i32 - diff.0, p0.1 as i32 - diff.1);
    while in_grid(size, &p) {
        antinodes.push((p.0 as usize, p.1 as usize));
        p.0 -= diff.0;
        p.1 -= diff.1;
    }
    let mut p: (i32, i32) = (p1.0 as i32 + diff.0, p1.1 as i32 + diff.1);
    while in_grid(size, &p) {
        antinodes.push((p.0 as usize, p.1 as usize));
        p.0 += diff.0;
        p.1 += diff.1;
    }
    let mut p: (i32, i32) = (p1.0 as i32 - diff.0, p1.1 as i32 - diff.1);
    while in_grid(size, &p) {
        antinodes.push((p.0 as usize, p.1 as usize));
        p.0 -= diff.0;
        p.1 -= diff.1;
    }
    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid_size = get_grid_size(input);
    let antennas = get_antennas(input);

    let mut antinodes: HashSet<Pos> = HashSet::new();
    for positions in antennas.values() {
        for (i, p0) in positions[0..positions.len() - 1].iter().enumerate() {
            for p1 in &positions[i+1..] {
                let p01_antinodes: Vec<(usize, usize)> = get_single_antinodes(&grid_size, p0, p1);
                for node in p01_antinodes {
                    antinodes.insert(node);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid_size = get_grid_size(input);
    let antennas = get_antennas(input);

    let mut antinodes: HashSet<Pos> = HashSet::new();
    for positions in antennas.values() {
        for (i, p0) in positions[0..positions.len() - 1].iter().enumerate() {
            for p1 in &positions[i+1..] {
                let p01_antinodes: Vec<(usize, usize)> = get_multi_antinodes(&grid_size, p0, p1);
                for node in p01_antinodes {
                    antinodes.insert(node);
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
