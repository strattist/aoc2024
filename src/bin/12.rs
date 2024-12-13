advent_of_code::solution!(12);

use std::collections::HashSet;

struct Cloud {
    id: char,
    points: HashSet<(usize, usize)>,
}

impl Cloud {
    pub fn new(grid: &mut Vec<Vec<char>>, pos: (usize, usize)) -> Self {
        let mut cloud = Cloud {
            id: grid[pos.0][pos.1],
            points: HashSet::new(),
        };
        cloud.fill_from_position(grid, &pos);
        cloud
    }

    pub fn area(&self) -> u32 {
        self.points.len() as u32
    }

    fn is_fence(&self, pos: &(usize, usize), diff: (i32, i32)) -> bool {
        let i_pos = (pos.0 as i32 + diff.0, pos.1 as i32 + diff.1);
        (i_pos.0 < 0 || i_pos.1 < 0) || !self.points.contains(&(i_pos.0 as usize, i_pos.1 as usize))
    }

    pub fn perimeter(&self) -> u32 {
        let mut n_fences = 0;
        for p in self.points.iter() {
            if self.is_fence(p, (-1, 0)) {
                n_fences += 1;
            }
            if self.is_fence(p, (1, 0)) {
                n_fences += 1;
            }
            if self.is_fence(p, (0, -1)) {
                n_fences += 1;
            }
            if self.is_fence(p, (0, 1)) {
                n_fences += 1;
            }
        }
        n_fences
    }

    pub fn sides(&self) -> u32 {
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        for p in &self.points {
            let ip = (p.0 as i32, p.1 as i32);
            if ip.0 < min_y {
                min_y = ip.0;
            }
            if ip.1 < min_x {
                min_x = ip.1;
            }
            if ip.0 > max_y {
                max_y = ip.0;
            }
            if ip.1 > max_x {
                max_x = ip.1;
            }
        }
        let mut rows = max_y - min_y + 1;
        let mut cols = max_x - min_x + 1;
        let new_points: Vec<(usize, usize)> = self.points.iter().map(|p| (p.0 - min_y as usize, p.1 - min_x as usize)).collect();

        let mut grid: Vec<Vec<char>> = vec![vec!['.'; (cols + 2) as usize]; (rows + 2) as usize];
        for y in 0..rows {
            for x in 0..cols {
                if new_points.contains(&(y as usize, x as usize)) {
                    grid[(y + 1) as usize][(x + 1) as usize]  = 'X';
                }
            }
        }

        let mut n_sides = 0;
        for _ in 0..2 {
            for y in 1..(rows + 1) {
                let row_y0 = &grid[y as usize];
                let row_y1 = &grid[(y - 1) as usize];
                let mut s = String::new();
                for (r0, r1) in row_y0.iter().zip(row_y1) {
                    if r0 != r1 && *r0 == 'X' {
                        s.push('X');
                    } else {
                        s.push(' ');
                    }
                }
                n_sides += s.split_whitespace().count();


                let row_y0 = &grid[y as usize];
                let row_y1 = &grid[(y + 1) as usize];
                let mut s = String::new();
                for (r0, r1) in row_y0.iter().zip(row_y1) {
                    if r0 != r1 && *r0 == 'X' {
                        s.push('X');
                    } else {
                        s.push(' ');
                    }
                }
                n_sides += s.split_whitespace().count();
            }
            let mut new_grid = vec![vec!['.'; (rows + 2) as usize]; (cols + 2) as usize];
            for y in 0..(rows + 2) {
                for x in 0..(cols + 2) {
                    new_grid[x as usize][y as usize] = grid[y as usize][x as usize];
                }
            }
            grid = new_grid;
            (rows, cols) = (cols, rows);
        }
        n_sides as u32
    }

    fn fill_from_position(&mut self, grid: &mut Vec<Vec<char>>, pos: &(usize, usize)) {
        if grid[pos.0][pos.1] == self.id {
            reset_grid_position(grid, pos);
            if self.points.contains(pos) {
                return;
            }
            self.points.insert(*pos);
            if let Some(top) = in_grid(grid, pos, (-1, 0)) {
                self.fill_from_position(grid, &top);
            }
            if let Some(bottom) = in_grid(grid, pos, (1, 0)) {
                self.fill_from_position(grid, &bottom);
            }
            if let Some(left) = in_grid(grid, pos, (0, -1)) {
                self.fill_from_position(grid, &left);
            }
            if let Some(right) = in_grid(grid, pos, (0, 1)) {
                self.fill_from_position(grid, &right);
            }
        }
   }
}

fn in_grid(grid: &Vec<Vec<char>>, pos: &(usize, usize), diff: (i32, i32)) -> Option<(usize, usize)> {
    let i_pos = (pos.0 as i32 + diff.0, pos.1 as i32 + diff.1);
    let (h, w) = (grid.len(), grid[0].len());
    if i_pos.0 >= 0 && i_pos.0 < h as i32 && i_pos.1 >= 0 && i_pos.1 < w as i32 {
        Some((i_pos.0 as usize, i_pos.1 as usize))
    } else {
        None
    }
}

fn reset_grid_position(grid: &mut Vec<Vec<char>>, pos: &(usize, usize)) {
    grid[pos.0][pos.1] = '.';
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line|
             line
                .chars()
                .collect()
        ).collect();
    grid
}

fn find_next_cloud_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c != '.') {
            return Some((i, j));
        }
    }
    None
}

fn get_clouds_from_grid(grid: &mut Vec<Vec<char>>) -> Vec<Cloud> {
    let mut clouds: Vec<Cloud> = Vec::new();
    while let Some((i, j)) = find_next_cloud_start(grid) {
        clouds.push(Cloud::new(grid, (i, j)));
    }
    clouds
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = get_grid(input);
    let clouds = get_clouds_from_grid(&mut grid);
    let count: u32 = clouds
        .iter()
        .map(|cloud| cloud.area() * cloud.perimeter())
        .sum();
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = get_grid(input);
    let clouds = get_clouds_from_grid(&mut grid);
    let count: u32 = clouds
        .iter()
        .map(|cloud| cloud.area() * cloud.sides())
        .sum();
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
