use std::collections::HashMap;

advent_of_code::solution!(10);

fn find_path(start: (usize, usize), grid: &[Vec<u32>], part: u32) -> u32 {
    let mut score = 0;
    let mut positions = vec![start];
    let mut finishes: HashMap<String, bool> = HashMap::new();

    while let Some((i, j)) = positions.pop() {
        if grid[i][j] == 9 {
            if part == 1 {
                finishes.insert(format!("{}, {}", i, j), true);
            } else {
                score += 1;
            }
        } else {
            if i > 0 && grid[i - 1][j] == grid[i][j] + 1 {
                positions.push((i - 1, j));
            }
            if i < grid.len() - 1 && grid[i + 1][j] == grid[i][j] + 1 {
                positions.push((i + 1, j));
            }
            if j > 0 && grid[i][j - 1] == grid[i][j] + 1 {
                positions.push((i, j - 1));
            }
            if j < grid[0].len() - 1 && grid[i][j + 1] == grid[i][j] + 1 {
                positions.push((i, j + 1));
            }
        }
    }

    match part {
        1 => finishes.len() as u32,
        2 => score,
        _ => 0,
    }
}

fn solver(input: &str, part: u32) -> u32 {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if *c == 0 {
                trailheads.push((i, j));
            }
        });
    });

    trailheads.iter().map(|h| find_path(*h, &grid, part)).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solver(input, 1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solver(input, 2))
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
