advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    let grid = input.lines().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();
    let directions: Vec<(i32, i32)> = vec![(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)];
    let word = vec!['X', 'M', 'A', 'S'];

    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if c == &word[0] {
                count += directions.iter().map(|dir| {
                    for n in 1..=3  {
                        let pos = (i as i32 + n * dir.0, j as i32 + n * dir.1);
                        if pos.0 < 0 || pos.0 >= grid.len() as i32 || pos.1 < 0 || pos.1 >= row.len() as i32 || grid[pos.0 as usize][pos.1 as usize] != word[n as usize] {
                            return 0;
                        }
                    }
                    1
                }).sum::<u32>();
            }
        });
    });

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    let grid = input.lines().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();

    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, c)| {
            if c == &'A' {
                if i > 0 && i < grid.len() - 1 && j > 0 && j < row.len() - 1 {
                    let tl = (i - 1, j - 1);
                    let br = (i + 1, j + 1);
                    let tr = (i - 1, j + 1);
                    let bl = (i + 1, j - 1);

                    if ((grid[tl.0][tl.1] == 'M' && grid[br.0][br.1] == 'S') || (grid[tl.0][tl.1] == 'S' && grid[br.0][br.1] == 'M')) && ((grid[tr.0][tr.1] == 'M' && grid[bl.0][bl.1] == 'S') || (grid[tr.0][tr.1] == 'S' && grid[bl.0][bl.1] == 'M')) {
                        count += 1;
                    }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
