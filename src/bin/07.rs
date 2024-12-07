use regex::Regex;

advent_of_code::solution!(7);

fn solve(input: &str, part: u8) -> u64 {
    fn operate(operators: &[u64], value: u64, part: u8) -> Vec<u64> {
        match operators.len() {
            0 => vec![value],
            _ => {
                let mut o = operators.to_owned();
                let x = o.remove(0);

                let mut plus = operate(&o, value + x, part);
                let mul = operate(&o, value * x, part);
                plus.extend(&mul);

                if part == 2 {
                    let con = operate(&o, value * 10u64.pow(x.ilog10() + 1) + x, part);
                    plus.extend(&con);
                }

                plus
            }
        }
    }

    let regex = Regex::new(r"^(\d+): (.+)$").unwrap();

    input.lines().fold(0, |acc, line| {
        let captures = regex.captures(line).unwrap();
        let l: u64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let mut r: Vec<u64> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();

        let x = r.remove(0);
        let results = operate(&r, x, part);

        if results.iter().any(|v| *v == l) {
            acc + l
        } else {
            acc
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 1))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
