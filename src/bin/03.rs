use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result =
        regex
            .captures_iter(input)
            .map(|c| c.extract())
            .fold(0, |acc, (_, [left, right])| {
                let l: i32 = left.parse().unwrap();
                let r: i32 = right.parse().unwrap();

                acc + l * r
            });

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let r_type = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let r_digits = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut is_enabled = true;
    let result = r_type
        .find_iter(input)
        .map(|m| m.as_str())
        .fold(0, |acc, m| match m {
            "do()" => {
                is_enabled = true;
                acc
            }
            "don't()" => {
                is_enabled = false;
                acc
            }
            _ => {
                if is_enabled {
                    let digits = r_digits.captures(m).unwrap();
                    let l: u32 = digits.get(1).unwrap().as_str().parse().unwrap();
                    let r: u32 = digits.get(2).unwrap().as_str().parse().unwrap();

                    acc + l * r
                } else {
                    acc
                }
            }
        });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
