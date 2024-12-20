use std::collections::HashMap;

advent_of_code::solution!(12);

type ScoreFn = fn(&HashMap<(char, usize, usize), bool>) -> u32;

fn find(
    regions: &[HashMap<(char, usize, usize), bool>],
    pos: (usize, usize),
    c: char,
) -> Option<usize> {
    regions
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .find(|&i| regions[i].contains_key(&(c, pos.0, pos.1)))
}

fn solve(input: &str, score: ScoreFn) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut regions: Vec<HashMap<(char, usize, usize), bool>> = Vec::new();

    grid.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, c)| {
            let left = match j > 0 {
                true => find(&regions, (i, j - 1), *c),
                false => None,
            };
            let mut top = match i > 0 {
                true => find(&regions, (i - 1, j), *c),
                false => None,
            };

            if left == top {
                top = None
            }

            if left.is_some() && top.is_some() {
                let l = left.unwrap();
                let t = top.unwrap();
                regions.clone()[std::cmp::max(l, t)].keys().for_each(|k| {
                    regions[std::cmp::min(l, t)].insert(*k, true);
                });
                regions.remove(std::cmp::max(l, t));
                regions[std::cmp::min(l, t)].insert((*c, i, j), true);
            } else if let Some(l) = left {
                regions[l].insert((*c, i, j), true);
            } else if let Some(t) = top {
                regions[t].insert((*c, i, j), true);
            } else {
                let mut m: HashMap<(char, usize, usize), bool> = HashMap::new();
                m.insert((*c, i, j), true);
                regions.push(m);
            }
        });
    });

    regions
        .iter()
        .fold(0, |acc, region| acc + region.len() as u32 * score(region))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, |region| {
        let perimeter = region.keys().fold(0, |p, (c, i, j)| {
            let mut perimeter = 0;
            if *i == 0 || !region.contains_key(&(*c, i - 1, *j)) {
                perimeter += 1;
            }
            if *j == 0 || !region.contains_key(&(*c, *i, j - 1)) {
                perimeter += 1;
            }
            if !region.contains_key(&(*c, i + 1, *j)) {
                perimeter += 1;
            }
            if !region.contains_key(&(*c, *i, j + 1)) {
                perimeter += 1;
            }
            p + perimeter
        });

        perimeter
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
