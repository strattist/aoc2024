advent_of_code::solution!(15);

use std::collections::HashSet;
use std::hash::Hash;
use std::fmt;

static mut WIDE_RATIO: usize = 1;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct XY {
    x: usize,
    y: usize
}

impl XY {
    pub fn update(&mut self, diff: (i32, i32)) {
        let i_xy = (self.x as i32 + diff.0, self.y as i32 + diff.1);
        assert!(i_xy.0 >= 0 && i_xy.1 >= 0);
        self.x = i_xy.0 as usize;
        self.y = i_xy.1 as usize;
    }

    pub fn coordinates(&self) -> usize {
        (self.y + 1) * 100 + (self.x + 1)
    }
}

enum Move {
    Up,
    Right,
    Down,
    Left
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Move::Up => write!(f, "^"),
            Move::Right => write!(f, ">"),
            Move::Down => write!(f, "v"),
            Move::Left => write!(f, "<"),
        }
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Move::Up => write!(f, "^"),
            Move::Right => write!(f, ">"),
            Move::Down => write!(f, "v"),
            Move::Left => write!(f, "<"),
        }
    }
}

struct Warehouse {
    height: usize,
    width: usize,
    boxes: HashSet<XY>,
    walls: HashSet<XY>,
    robot: XY
}

impl Warehouse {
    pub fn new(input: &str) -> Self {
        unsafe {
            let lines: Vec<&str> = input
                .lines() // Read lines
                .collect();
            // Don't care about the outside walls
            let height = lines.len() - 2;
            let width = (lines[0].len() - 2) * WIDE_RATIO;
            let (walls, boxes, robots) =
                input
                    .lines()
                    .skip(1)
                    .take(height)
                    .enumerate()
                    .fold(
                    (HashSet::<XY>::new(), HashSet::<XY>::new(), HashSet::<XY>::new()),
                    |(mut w, mut b, mut r), (i, row)| {
                        row
                            .chars()
                            .skip(1)
                            .take(width)
                            .enumerate()
                            .for_each(
                                |(j, c)| {
                                    if c == '#' {
                                        w.insert(XY {x: j * WIDE_RATIO, y: i});
                                    } else if c == 'O' {
                                        b.insert(XY {x: j * WIDE_RATIO, y: i});
                                    } else if c == '@' {
                                        r.insert(XY {x: j * WIDE_RATIO, y: i});
                                    }
                                });
                        (w, b, r)
                    }
                );
            assert!(robots.len() == 1);
            Warehouse {
                height,
                width,
                boxes,
                walls,
                robot: *robots.iter().next().unwrap()
            }
        }
    }

    fn try_push(&self, pos: XY, diff: (i32, i32)) -> Option<XY> {
        let i_pos = (pos.x as i32 + diff.0, pos.y as i32 + diff.1);
        if i_pos.0 >= 0 && i_pos.1 >= 0 && i_pos.0 < self.width as i32 && i_pos.1 < self.height as i32 {
            let next_xy = XY {x: i_pos.0 as usize, y: i_pos.1 as usize};
            if !self.walls.contains(&next_xy) {
                return Some(next_xy);
            }
        }
        None
    }

    fn find_boxes_to_move(&self, diff: (i32, i32)) -> Option<Vec<XY>> {
        let mut pos = self.robot;
        let mut boxes: Vec<XY> = Vec::new();
        let mut found_empty = false;
        while let Some(next_pos) = self.try_push(pos, diff) {
            pos = next_pos;
            if !self.boxes.contains(&next_pos) {
                // next pos is empty
                found_empty = true;
                break;
            } else {
                // this is a box we want to move also
                boxes.push(next_pos);
            }
        }
        if pos == self.robot || !found_empty {
            None
        } else {
            Some(boxes)
        }
    }

    pub fn move_robot(&mut self, m: &Move) {
        let diff = match m {
            Move::Up => (0, -1),
            Move::Right => (1, 0),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
        };

        if let Some(mut boxes) = self.find_boxes_to_move(diff) {
            self.robot.update(diff);
            boxes.reverse();
            for b in boxes.iter_mut() {
                self.boxes.remove(b);
                b.update(diff);
                self.boxes.insert(*b);
            }
        }
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let walltop = (0..(self.width + 2) * WIDE_RATIO).map(|_| '#').collect::<String>();
            let wall = (0..WIDE_RATIO).map(|_| '#').collect::<String>();
            let empty = (0..WIDE_RATIO).map(|_| '.').collect::<String>();
            let sbox = if WIDE_RATIO == 2 {"[]"} else {"O"};
            writeln!(f, "{}", walltop);
            for y in 0..self.height {
                write!(f, "{}", wall);
                for x in 0..self.width {
                    let xy = XY {x, y};
                    if xy == self.robot {
                        if WIDE_RATIO == 1 {
                            write!(f, "@");
                        } else {
                            if xy.x % 2 == 0 {
                                write!(f, "@.");
                            } else {
                                write!(f, ".@");
                            }
                        }
                    } else {
                        if self.walls.contains(&xy) {
                            write!(f, "{}", wall);
                        } else if self.boxes.contains(&xy) {
                            write!(f, "{}", sbox);
                        } else {
                            write!(f, "{}", empty);
                        }
                    }
                }
                writeln!(f, "{}", wall);
            }
            writeln!(f, "{}", walltop)
        }
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .enumerate()
        .fold(
            Vec::<Move>::new(),
            |mut m, (i, row)| {
                row.chars().enumerate().for_each(
                    |(j, c)| {
                        match c {
                            '^' => m.push(Move::Up),
                            '>' => m.push(Move::Right),
                            'v' => m.push(Move::Down),
                            '<' => m.push(Move::Left),
                            _ => ()
                        };
                    }
                );
                m
            })
}

fn parse_input(input: &str) -> (Warehouse, Vec<Move>) {
    let blocks: Vec<&str> = input.split("\n\n").map(str::trim).collect();
    assert!(blocks.len() == 2);
    (Warehouse::new(blocks[0]), parse_moves(blocks[1]))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut warehouse, moves) = parse_input(input);
    for m in moves.iter() {
        warehouse.move_robot(m);
    }
    let coordinates: usize =
        warehouse.boxes
            .iter()
            .map(|b| b.coordinates())
            .sum();
    Some(coordinates as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut warehouse, moves) = parse_input(input);
    println!("{}", warehouse);
    //     warehouse.move_robot(m);
    // for m in moves.iter() {
    // }
    // let coordinates: usize =
    //     warehouse.boxes
    //         .iter()
    //         .map(|b| b.coordinates())
    //         .sum();
    // Some(coordinates as u32)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        unsafe {
            WIDE_RATIO = 2;
            let result = part_two(&advent_of_code::template::read_file("examples", DAY));
            assert_eq!(result, None);
        }
    }
}
