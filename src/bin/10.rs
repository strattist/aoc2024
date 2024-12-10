advent_of_code::solution!(10);

use std::collections::HashSet;

#[derive(Debug)]
struct Path {
    value: i32,
    pos: (usize, usize),
    nexts: Vec<Path>
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.pos == other.pos
    }
}

fn in_grid(hw: &(i32, i32), offset: &(i32, i32), pos: &(usize, usize)) -> Option<(usize, usize)>{
    let npos = (pos.0 as i32 + offset.0, pos.1 as i32 + offset.1);
    if npos.0 >= 0 && npos.0 < hw.0 && npos.1 >= 0 && npos.1 < hw.1 {
        Some((npos.0 as usize, npos.1 as usize))
    } else {
        None
    }
}

impl Path {
    fn new(value: i32, pos: (usize, usize)) -> Self {
        Path {
            value,
            pos,
            nexts: Vec::<Path>::new(),
        }
    }

    pub fn navigate(&mut self, grid: &Vec<Vec<i32>>, height: i32) {
        if self.value < height {
            let hw = (grid.len() as i32, grid[0].len() as i32);
            let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1)];
            for offset in offsets {
                if let Some(pos) = in_grid(&hw, &offset, &self.pos) {
                    if grid[pos.0][pos.1] == (self.value + 1) {
                        let new_path = Path::new(self.value + 1, pos);
                        if !self.nexts.contains(&new_path) {
                            self.nexts.push(new_path);
                        }
                    }
                }
            }
            for next in self.nexts.iter_mut() {
                next.navigate(grid, height);
            }
        }
    }

    pub fn find_unique_accessible_heights(&self, height: i32) -> HashSet<(usize, usize)> {
        let mut height_positions: HashSet<(usize, usize)> = HashSet::new();
        for next in self.nexts.iter() {
            if next.value == height {
                height_positions.insert(next.pos);
            } else {
                height_positions.extend(next.find_unique_accessible_heights(height).iter());
            }
        }
        height_positions
    }

    pub fn count_unique_trailheads(&self, height_position: (usize, usize)) -> u32 {
        let mut rating: u32 = 0;
        for next in self.nexts.iter() {
            if next.pos == height_position {
                rating += 1;
            } else {
                rating += next.count_unique_trailheads(height_position);
            }
        }
        rating
    }

}

fn get_grid(input: &str) -> Vec<Vec<i32>> {
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line|
             line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        ).collect();
    grid
}

fn get_starts_of_path(grid: &Vec<Vec<i32>>) -> Vec<Path> {
    let paths: Vec<Path> = grid
        .iter()
        .enumerate()
        .fold(
            Vec::<Path>::new(),
            |mut p, (i, row)| {
                let positions: Vec<(usize, usize)> = row
                    .iter()
                    .enumerate()
                    .filter_map(|(j, v)| if *v == 0 {Some((i, j))} else {None})
                    .collect();
                p.extend(positions.iter().map(|pos| Path::new(0, *pos)));
                p
            });
    paths
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = get_grid(input);
    let mut zeros = get_starts_of_path(&grid);
    let mut count = 0;
    let stop = 9;
    for zero in zeros.iter_mut() {
        zero.navigate(&grid, stop);
        let heights= zero.find_unique_accessible_heights(stop);
        count += heights.len() as u32;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = get_grid(input);
    let mut zeros = get_starts_of_path(&grid);
    let mut count = 0;
    let stop = 9;
    for zero in zeros.iter_mut() {
        zero.navigate(&grid, stop);
        let heights= zero.find_unique_accessible_heights(stop);
        for height in heights {
            count += zero.count_unique_trailheads(height);
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
