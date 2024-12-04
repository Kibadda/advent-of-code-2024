advent_of_code::solution!(2);

fn is_safe(report: Vec<i32>) -> bool {
    let count = report.len();

    let is_increasing = report[0] < report[1];

    for n in 1..count {
        let diff = (report[n-1] - report[n]).abs();

        if !(1..=3).contains(&diff) || ((report[n-1] < report[n]) != is_increasing) {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe = 0;

    input.lines().for_each(|line| {
        let split: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

        if is_safe(split) {
            safe += 1;
        }
    });

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe = 0;

    input.lines().for_each(|line| {
        let split: Vec<i32> = line.split_whitespace().map(|c| c.parse().unwrap()).collect();

        if is_safe(split.clone()) {
            safe += 1;
        } else {
            for n in 0..split.len() {
                let clone = split.iter().enumerate ().filter(|&(i, _)| i != n).map(|(_, e)| *e).collect();
                if is_safe(clone) {
                    safe += 1;
                    break;
                }
            }
        }
    });

    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
