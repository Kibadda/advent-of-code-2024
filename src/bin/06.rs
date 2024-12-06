use std::collections::HashMap;

advent_of_code::solution!(6);

#[derive(Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

enum Return {
    Exit(HashMap<String, u32>),
    Loop,
}

fn walk(mut pos: (usize, usize), grid: &[Vec<char>]) -> Return {
    let mut visits: HashMap<String, u32> = HashMap::new();
    let mut loop_check: HashMap<String, u32> = HashMap::new();
    let mut dir = Dir::Up;
    loop {
        match dir {
            Dir::Up => {
                if pos.0 == 0 {
                    break;
                } else if grid[pos.0 - 1][pos.1] == '#' {
                    dir = Dir::Right;
                } else {
                    pos = (pos.0 - 1, pos.1);
                }
            }
            Dir::Right => {
                if pos.1 == grid[0].len() - 1 {
                    break;
                } else if grid[pos.0][pos.1 + 1] == '#' {
                    dir = Dir::Down;
                } else {
                    pos = (pos.0, pos.1 + 1);
                }
            }
            Dir::Down => {
                if pos.0 == grid.len() - 1 {
                    break;
                } else if grid[pos.0 + 1][pos.1] == '#' {
                    dir = Dir::Left;
                } else {
                    pos = (pos.0 + 1, pos.1);
                }
            }
            Dir::Left => {
                if pos.1 == 0 {
                    break;
                } else if grid[pos.0][pos.1 - 1] == '#' {
                    dir = Dir::Up;
                } else {
                    pos = (pos.0, pos.1 - 1);
                }
            }
        }

        visits.insert(format!("{}|{}", pos.0, pos.1), 1);

        let check = format!("{}|{}|{:?}", pos.0, pos.1, dir);

        if let std::collections::hash_map::Entry::Vacant(e) = loop_check.entry(check) {
            e.insert(1);
        } else {
            return Return::Loop;
        }
    }

    Return::Exit(visits)
}

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut pos: (usize, usize) = (0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let chars = line.chars();

            if let Some(j) = chars.clone().position(|c| c == '^') {
                pos = (i, j);
            }

            chars.collect()
        })
        .collect();

    (pos, grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pos, grid) = parse_input(input);

    if let Return::Exit(visits) = walk(pos, &grid) {
        return Some(visits.len() as u32);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let (pos, grid) = parse_input(input);
    let mut visits: HashMap<String, u32> = HashMap::new();
    if let Return::Exit(v) = walk(pos, &grid) {
        visits = v;
    }

    let mut count = 0;

    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, _)| {
            if (i, j) != pos && visits.contains_key(&format!("{}|{}", i, j)) {
                let mut clone = grid.clone();
                clone[i][j] = '#';

                if let Return::Loop = walk(pos, &clone) {
                    count += 1;
                }
            }
        });
    });

    Some(count)
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
