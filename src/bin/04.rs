advent_of_code::solution!(4);

use regex::Regex;

fn found_word(lines: &Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32), word: &str) -> bool {
    let n = (word.len() as i32) - 1;
    let end = (pos.0 + dir.0 * n, pos.1 + dir.1 * n);
    if end.0 < 0 || end.1 < 0 || end.0 >= lines.len() as i32 || end.1 >= lines.first().unwrap().len() as i32 {
        return false;
    } else {
        for (i, c) in word.chars().enumerate() {
            let pos_i = (pos.0 + dir.0 * i as i32, pos.1 + dir.1 * i as i32);
            let upos_i = (pos_i.0 as usize, pos_i.1 as usize);
            if lines[upos_i.0][upos_i.1] != c {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let xmas = "XMAS";
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut count: usize = 0;
    let moves = [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == xmas.chars().next().unwrap() {
                count += moves.iter().filter(|dir| found_word(&lines, (y as i32, x as i32), **dir, xmas)).count();
            }
        }
    }
    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_i = lines.len() - 2;
    let max_j = lines.first().unwrap().len() - 2;
    let mut count: u32 = 0;
    let re = Regex::new("(M.M\n.A.\nS.S)|(M.S\n.A.\nM.S)|(S.M\n.A.\nS.M)|(S.S\n.A.\nM.M)").unwrap();
    for i in 0..max_i {
        for j in 0..max_j {
            let tmb: Vec<String> = vec![lines[i][j..j+3].iter().collect(), lines[i+1][j..j+3].iter().collect(), lines[i+2][j..j+3].iter().collect()];
            let joined_tmb: String = tmb.join("\n");
            if re.is_match(joined_tmb.as_str()) {
                count += 1;
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
