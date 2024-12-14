advent_of_code::solution!(14);

use regex::Regex;
use std::fmt;

static mut HEIGHT: usize = 103;
static mut WIDTH: usize = 101;

struct XY {
    x: i32,
    y: i32
}

struct Guard {
    p: XY,
    v: XY
}

impl Guard {
    pub fn new(desc: &str) -> Self {
        let pv: Vec<&str> = desc.split_whitespace().collect();
        let p = get_xy(pv[0]);
        let v = get_xy(pv[1]);
        Guard {
            p: XY {x: p.0, y: p.1},
            v: XY {x: v.0, y: v.1},
        }
    }

    pub fn move_n(&self, n: u32) -> XY {
        XY {
            x: self.p.x + n as i32 * self.v.x,
            y: self.p.y + n as i32 * self.v.y,
        }
    }
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guard(x={},y={}, velocity={},{})", self.p.x, self.p.y, self.v.x, self.v.y)
    }
}

struct Grid {
    height: usize,
    width: usize,
    guards: Vec<Vec<u32>>
}

impl Grid {
    pub fn new(h: usize, w: usize) -> Self {
        Grid {
            height: h,
            width: w,
            guards: vec![vec![0; w]; h],
        }
    }

    pub fn reset(&mut self) {
        self.guards = vec![vec![0; self.width]; self.height];
    }

    pub fn estimate_guards_position(&mut self, guards: &Vec<Guard>, seconds: u32) {
        for guard in guards {
            let mut p = guard.move_n(seconds);
            while p.x < 0 {
                p.x += self.width as i32;
            }
            while p.y < 0 {
                p.y += self.height as i32;
            }
            let p_in_grid = ((p.x as usize) % self.width, (p.y as usize) % self.height);
            self.guards[p_in_grid.1][p_in_grid.0] += 1;
        }
    }

    fn split_grid_into_quadrants(&self) -> (Vec<Vec<u32>>, Vec<Vec<u32>>, Vec<Vec<u32>>, Vec<Vec<u32>>) {
        // Ensure the grid dimensions are odd.
        assert!(self.height % 2 == 1 && self.width % 2 == 1, "Grid must have odd dimensions");

        let mid_row = self.height / 2;
        let mid_col = self.width / 2;

        let mut top_left = Vec::new();
        let mut top_right = Vec::new();
        let mut bottom_left = Vec::new();
        let mut bottom_right = Vec::new();

        for i in 0..self.height {
            if i == mid_row {
                // Skip the middle row
                continue;
            }

            let mut left = Vec::new();
            let mut right = Vec::new();

            for j in 0..self.width {
                if j == mid_col {
                    // Skip the middle column
                    continue;
                }

                if i < mid_row && j < mid_col {
                    left.push(self.guards[i][j]);
                } else if i < mid_row && j > mid_col {
                    right.push(self.guards[i][j]);
                } else if i > mid_row && j < mid_col {
                    left.push(self.guards[i][j]);
                } else if i > mid_row && j > mid_col {
                    right.push(self.guards[i][j]);
                }
            }

            if i < mid_row {
                top_left.push(left);
                top_right.push(right);
            } else if i > mid_row {
                bottom_left.push(left);
                bottom_right.push(right);
            }
        }

        (top_left, top_right, bottom_left, bottom_right)
    }

    pub fn safety_factor(&self) -> u32 {
        let (top_left, top_right, bottom_left, bottom_right) = self.split_grid_into_quadrants();
        let mut product = top_left.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>();
        product *= top_right.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>();
        product *= bottom_left.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>();
        product *= bottom_right.iter().map(|row| row.iter().sum::<u32>()).sum::<u32>();
        product
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.guards.iter() {
            let row_s: String = row
                .iter()
                .map(|c| {
                    if c == &0 {
                        return ".".to_string();
                    } else {
                        return c.to_string();
                    }
                }).collect();
            writeln!(f, "{}", row_s);
        }
        Ok(())
    }
}

fn get_xy(xy_string: &str) -> (i32, i32) {
    let re = Regex::new(r"(p|v)=(?<x>-?\d+),(?<y>-?\d+)").unwrap();
    let xy = re.captures(xy_string).unwrap();
    (xy["x"].parse::<i32>().unwrap(), xy["y"].parse::<i32>().unwrap())
}

fn parse_input(input: &str) -> Vec<Guard> {
    let guards: Vec<Guard> = input
        .lines()
        .map(|line| Guard::new(line))
        .collect();
    guards
}

pub fn part_one(input: &str) -> Option<u32> {
    let guards = parse_input(input);
    unsafe {
        let mut grid = Grid::new(HEIGHT, WIDTH);
        grid.estimate_guards_position(&guards, 100);
        Some(grid.safety_factor())
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let guards = parse_input(input);
    unsafe {
        let mut grid = Grid::new(HEIGHT, WIDTH);
        let reference_safety_factor = 229839456 as f32;

        let mut found_anomaly = false;
        let mut n = 0;
        while !found_anomaly {
            grid.reset();
            grid.estimate_guards_position(&guards, n);
            let ratio = (grid.safety_factor() as f32 / reference_safety_factor).round() as u32;
            if ratio != 1 {
                found_anomaly = true;
                println!("{}", grid);
            } else {
                n += 1
            }
        }
        Some(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        unsafe {
            HEIGHT = 7;
            WIDTH = 11;
            let result = part_one(&advent_of_code::template::read_file("examples", DAY));
            assert_eq!(result, Some(12));
        }
    }
}
