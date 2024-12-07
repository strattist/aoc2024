advent_of_code::solution!(6);

use std::collections::HashSet;

fn parse_map(input: &str) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for row in map.iter_mut() {
        row.insert(0, 'S');
        row.push('S');
    }
    map.insert(0, vec!['S'; map[0].len()]);
    map.push(vec!['S'; map[0].len()]);

    let mut pos: (usize, usize) = (0, 0);
    for (i, row) in map.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            pos = (i, j);
            continue
        }
    }
    map[pos.0][pos.1] = 'o';
    map
}

fn increment_pos(pos: (usize, usize), c: char) -> (usize, usize) {
    match c {
        '^' => (pos.0 - 1, pos.1),
        '>' => (pos.0, pos.1 + 1),
        'v' => (pos.0 + 1, pos.1),
        '<' => (pos.0, pos.1 - 1),
        _ => panic!("Shouldn't be here (increment_pos({:?}, {})", pos, c),
    }
}

fn to_the_right(c: char) -> char {
    match c {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Shouldn't be here (to_the_right({})", c),
    }
}

fn find_guard_position(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'o' {
                return Some((i, j));
            }
        }
    }
    None
}

fn get_visited_path_and_update_map(map: &mut Vec<Vec<char>>) -> bool {
    let mut last_direction = '^';
    let mut visited_positions = HashSet::new();
    if let Some((mut y, mut x)) = find_guard_position(map) {
        loop {
            let (ny, nx) = increment_pos((y, x), last_direction);
            visited_positions.insert((y, x, last_direction));
            map[y][x] = 'X';

            if map[ny][nx] == 'S' {
                return false;
            }

            if visited_positions.contains(&(ny, nx, last_direction)) {
                // Looping
                return true;
            }

            match map[ny][nx] {
                '#' => {
                    last_direction = to_the_right(last_direction);
                },
                _ => {
                    y = ny;
                    x = nx;
                }
            }
        }
    } else {
        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_map(input);
    get_visited_path_and_update_map(&mut map);
    let mut count: u32 = 0;
    for row in &map {
        for col in row {
            if *col == 'X' {
                count += 1
            }
        }
    }
    Some(count)
}


pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);
    let guard = find_guard_position(&map);

    let mut visited = map.clone();
    get_visited_path_and_update_map(&mut visited);
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
    for (i, row) in visited.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'X' {
                visited_positions.insert((i, j));
            }
        }
    }
    if let Some((guard_x, guard_y)) = guard {
        visited_positions.remove(&(guard_x, guard_y));
    }

    let mut loop_count: u32 = 0;
    for visited_position in visited_positions {
        let (y, x) = visited_position;
        if map[y][x] == '.' {
            let mut new_map = map.clone();
            new_map[y][x] = '#';
            if get_visited_path_and_update_map(&mut new_map) {
                loop_count += 1;
            }
        }
    }
    Some(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
