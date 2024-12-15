advent_of_code::solution!(15);

use std::collections::HashSet;
use std::hash::Hash;
use std::fmt;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Wideness {
    Simple,
    Double
}

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
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Box {
    xy: XY,
    wideness: Wideness
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.xy)
    }
}

impl fmt::Debug for Box {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.xy)
    }
}

impl Box {
    pub fn update(&mut self, diff: (i32, i32)) {
        let i_xy = (self.xy.x as i32 + diff.0, self.xy.y as i32 + diff.1);
        assert!(i_xy.0 >= 0 && i_xy.1 >= 0);
        self.xy.x = i_xy.0 as usize;
        self.xy.y = i_xy.1 as usize;
    }

    pub fn contains(&self, pos: &XY) -> bool {
        match self.wideness {
            Wideness::Simple => self.xy == *pos,
            Wideness::Double => {
                let xy1 = XY {
                    x: self.xy.x + 1,
                    y: self.xy.y
                };
                self.xy == *pos || xy1 == *pos
            }
        }
    }

    pub fn contains_any(&self, pos: &Vec<XY>) -> bool {
        pos.iter().filter(|xy| self.contains(xy)).count() > 0
    }

    pub fn coordinates(&self) -> usize {
        (self.xy.y + 1) * 100 + (self.tr().x + 1)
    }

    pub fn tr(&self) -> XY {
        match self.wideness {
            Wideness::Simple => self.xy,
            Wideness::Double => XY {
                x: self.xy.x + 1,
                y: self.xy.y
            }
        }
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
    boxes: HashSet<Box>,
    walls: HashSet<XY>,
    robot: XY,
    wideness: Wideness
}

impl Warehouse {
    pub fn new(input: &str, wideness: Wideness) -> Self {
        let wide_ratio = match wideness {
            Wideness::Simple => 1,
            Wideness::Double => 2
        };
        let lines: Vec<&str> = input
            .lines() // Read lines
            .collect();
        // Don't care about the outside walls
        let height = lines.len() - 2;
        let width = lines[0].len() - 2;
        let (walls, boxes, robots) =
            input
                .lines()
                .skip(1)
                .take(height)
                .enumerate()
                .fold(
                (HashSet::<XY>::new(), HashSet::<Box>::new(), HashSet::<XY>::new()),
                |(mut w, mut b, mut r), (i, row)| {
                    row
                        .chars()
                        .skip(1)
                        .take(width)
                        .enumerate()
                        .for_each(
                            |(j, c)| {
                                if c == '#' {
                                    for jj in (j*wide_ratio)..((j+1)*wide_ratio) {
                                        w.insert(XY {x: jj, y: i});
                                    }
                                } else if c == 'O' {
                                    b.insert(Box {
                                        xy: XY {x: j * wide_ratio, y: i},
                                        wideness
                                    });
                                } else if c == '@' {
                                    r.insert(XY {x: j * wide_ratio, y: i});
                                }
                            });
                    (w, b, r)
                }
            );
        assert!(robots.len() == 1);
        Warehouse {
            height,
            width: width * wide_ratio,
            boxes,
            walls,
            robot: *robots.iter().next().unwrap(),
            wideness
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

    fn try_push_box(&self, b: &Box, diff: (i32, i32)) -> Option<Box> {
        if self.try_push(b.xy, diff).and(self.try_push(b.tr(), diff)).is_some() {
            let mut next_box = *b;
            next_box.update(diff);
            Some(next_box)
        } else {
            None
        }
    }

    fn find_pushed_boxes(&self, xys: &Vec<XY>) -> Option<HashSet<Box>> {
        let boxes: HashSet<Box> = self.boxes
            .iter()
            .filter(|b| b.contains_any(xys))
            .cloned()
            .collect();
        if !boxes.is_empty() {
            Some(boxes)
        } else {
            None
        }
    }

    fn find_boxes_to_move_alternative(&self, diff: (i32, i32)) -> Option<Vec<Box>> {
        if let Some(next_robot) = self.try_push(self.robot, diff) {
            // At this point we can move the robot in a position that is not a wall.
            // But maybe moving there will move boxes. So I first collect all boxes that are supposed to move.
            let mut pushed_xys: Vec<XY> = vec![next_robot];
            let mut boxes: HashSet<Box> = HashSet::new();
            let mut impossible = false;
            let mut tested: HashSet<XY> = HashSet::new();
            while let Some(pushed_boxes) = self.find_pushed_boxes(&pushed_xys) {
                for p in pushed_xys.iter() {
                    tested.insert(*p);
                }
                pushed_xys.clear();
                for b in pushed_boxes {
                    if let Some(new_b) = self.try_push_box(&b, diff) {
                        boxes.insert(b);
                        if !tested.contains(&new_b.xy) {
                            pushed_xys.push(new_b.xy);
                        }
                        if new_b.tr() != new_b.xy && !tested.contains(&new_b.tr()) {
                            pushed_xys.push(new_b.tr());
                        }
                    } else {
                        impossible = true;
                        break;
                    }
                }
                if impossible {
                    break;
                }
            }
            // Here we don't add more boxes we must ensure all thoses boxes can actually move
            if !impossible {
                return Some(boxes.iter().cloned().collect::<Vec<_>>());
            }
        }
        None
    }

    pub fn move_robot(&mut self, m: &Move) {
        let diff = match m {
            Move::Up => (0, -1),
            Move::Right => (1, 0),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
        };

        if let Some(mut boxes) = self.find_boxes_to_move_alternative(diff) {
            self.robot.update(diff);
            for b in boxes.iter() {
                self.boxes.remove(b);
            }
            for b in boxes.iter_mut() {
                b.update(diff);
                self.boxes.insert(*b);
            }
        }
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wide_ratio = match self.wideness {
            Wideness::Simple => 1,
            Wideness::Double => 2
        };

        let mut sw: Vec<Vec<char>> = vec![vec!['.'; self.width]; self.height];
        for wall in self.walls.iter() {
            sw[wall.y][wall.x] = '#';
        }
        for b in self.boxes.iter() {
            assert!(b.wideness == self.wideness);
            match b.wideness {
                Wideness::Simple => sw[b.xy.y][b.xy.x] = 'O',
                Wideness::Double => {
                    sw[b.xy.y][b.xy.x] = '[';
                    sw[b.xy.y][b.xy.x + 1] = ']';
                }
            }
        }
        sw[self.robot.y][self.robot.x] = '@';

        let wall = (0..wide_ratio).map(|_| '#').collect::<String>();
        let walltop = (0..self.width + 2 * wide_ratio).map(|_| '#').collect::<String>();
        _ = writeln!(f, "{}", walltop);
        for row in sw {
            _ = write!(f, "{}", wall);
            _ = write!(f, "{}", row.iter().collect::<String>());
            _ = writeln!(f, "{}", wall);
        }
        writeln!(f, "{}", walltop)
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .fold(
            Vec::<Move>::new(),
            |mut m, row| {
                row.chars().for_each(
                    |c| {
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

fn parse_input(input: &str, wideness: Wideness) -> (Warehouse, Vec<Move>) {
    let blocks: Vec<&str> = input.split("\n\n").map(str::trim).collect();
    assert!(blocks.len() == 2);
    (Warehouse::new(blocks[0], wideness), parse_moves(blocks[1]))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut warehouse, moves) = parse_input(input, Wideness::Simple);
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
    let (mut warehouse, moves) = parse_input(input, Wideness::Double);
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
