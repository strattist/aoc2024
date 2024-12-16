

advent_of_code::solution!(16);

use std::collections::{VecDeque, HashMap, HashSet};

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
enum Direction {
    East,
    West,
    North,
    South
}

#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
struct XY {
    x: usize,
    y: usize
}

fn find_routes(labyrinth: &Vec<Vec<char>>) -> Vec<(Vec<XY>, u32)> {
    let (start, end) = find_start_and_end(labyrinth);
    let mut queue: VecDeque<(XY, Vec<XY>, u32, Direction)> = VecDeque::new();
    queue.push_back((start, vec![start], 0, Direction::East));

    let directions: Vec<(Direction, (i32, i32))> = vec![
        (Direction::East, (1, 0)),
        (Direction::West, (-1, 0)),
        (Direction::South, (0, 1)),
        (Direction::North, (0, -1)),
    ];
    let mut visited: HashMap<(XY, Direction), u32> = HashMap::new();

    let mut routes: Vec<(Vec<XY>, u32)> = Vec::new();
    while let Some((current_pos, history, current_score, current_direction)) = queue.pop_front() {
        if current_pos == end {
            routes.push((history, current_score));
            continue;
        }

        if visited.contains_key(&(current_pos, current_direction)) && visited.get(&(current_pos, current_direction)).unwrap() < &current_score {
            continue;
        }

        visited.insert((current_pos, current_direction), current_score);

        for (direction, (dx, dy)) in directions.iter() {
            let new_pos = (current_pos.x as i32 + dx, current_pos.y as i32 + dy);
            if new_pos.0 >= 0 && new_pos.0 < labyrinth[0].len() as i32 &&
               new_pos.1 >= 0 && new_pos.1 < labyrinth.len() as i32 &&
               labyrinth[new_pos.1 as usize][new_pos.0 as usize] != '#' {
                let new_pos_xy = XY {x: new_pos.0 as usize, y: new_pos.1 as usize};
                let mut new_history = history.clone();
                if *direction == current_direction {
                    new_history.push(new_pos_xy);
                    queue.push_back((new_pos_xy, new_history, current_score + 1, *direction));
                } else {
                    queue.push_back((current_pos, new_history, current_score + 1000, *direction));
                }
            }
        }
    }
    routes
}

fn find_start_and_end(labyrinth: &Vec<Vec<char>>) -> (XY, XY) {
    let mut start = XY {x: 0, y: 0};
    let mut end = XY {x: 0, y: 0};
    for (y, line) in labyrinth.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if *col == 'S' {
                start = XY {x, y};
            } else if *col == 'E' {
                end = XY {x, y};
            }
        }
    }
    (start, end)
}

pub fn part_one(input: &str) -> Option<u32> {
    let labyrinth: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let routes = find_routes(&labyrinth);
    let min = routes.iter().map(|(_, score)| score).min().unwrap();
    Some(*min)
}

pub fn part_two(input: &str) -> Option<u32> {
    let labyrinth: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    let routes = find_routes(&labyrinth);
    let min = routes.iter().map(|(_, score)| score).min().unwrap();
    let best_seats = routes
        .iter()
        .filter_map(|(route, score)| {
            if score == min {
                Some(route)
            } else {
                None
            }
        })
        .fold(
            Vec::<XY>::new(),
            |mut v, route| {
                v.extend(route);
                v
            }
        );
    let best_seats_set: HashSet<XY> = HashSet::from_iter(best_seats.iter().cloned());
    Some(best_seats_set.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
