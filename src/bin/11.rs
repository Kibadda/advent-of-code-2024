advent_of_code::solution!(11);

fn solver(input: &str, blinking: u32) -> u64 {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..blinking {
        stones = stones
            .iter()
            .flat_map(|e| {
                if *e == 0 {
                    vec![1]
                } else if e.to_string().len() % 2 == 0 {
                    let s = e.to_string();
                    let (a, b) = s.split_at(s.len() / 2);
                    vec![a.parse().unwrap(), b.parse().unwrap()]
                } else {
                    vec![e * 2024]
                }
            })
            .collect();
    }

    stones.len() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solver(input, 25))
}

pub fn part_two(_input: &str) -> Option<u64> {
    // Some(solver(input, 75))
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
