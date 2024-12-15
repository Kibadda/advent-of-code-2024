advent_of_code::solution!(9);

fn solver(input: &str, formatter: fn(Vec<u64>) -> Vec<u64>) -> u64 {
    let mut blocks: Vec<u64> = Vec::new();
    let mut is_free = false;
    let mut id = 1;

    input.chars().for_each(|c| {
        if let Some(num) = c.to_digit(10) {
            for _ in 0..num {
                match is_free {
                    true => blocks.push(0),
                    false => blocks.push(id),
                }
            }

            if !is_free {
                id += 1;
            }

            is_free = !is_free;
        }
    });

    blocks = formatter(blocks);

    blocks.iter().enumerate().fold(0, |acc, (i, n)| {
        if *n == 0 {
            acc
        } else {
            acc + (i as u64) * (n - 1)
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solver(input, |mut blocks| {
        let mut i = 0;
        while i < blocks.len() {
            if blocks[i] == 0 {
                for n in (i..blocks.len()).rev() {
                    if blocks[n] != 0 {
                        blocks[i] = blocks[n];
                        blocks[n] = 0;
                        break;
                    }
                }
            }
            i += 1;
        }
        blocks
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solver(input, |mut blocks| {
        let mut length = 0;
        let mut number = 0;
        blocks.clone().iter().enumerate().rev().for_each(|(i, n)| {
            if *n != number {
                if length > 0 && number != 0 {
                    for (j, k) in blocks.clone().windows(length).enumerate() {
                        if j > i {
                            break;
                        }
                        if k.iter().all(|u| *u == 0) {
                            for o in 0..length {
                                blocks[j + o] = blocks[i + 1 + o];
                                blocks[i + 1 + o] = 0;
                            }
                            break;
                        }
                    }
                }
                number = *n;
                length = 1;
            } else {
                length += 1;
            }
        });
        blocks
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
