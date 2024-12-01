advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse().unwrap());
        right.push(split.next().unwrap().parse().unwrap());
    });

    left.sort();
    right.sort();

    let it = left.iter().zip(right.iter());

    let mut diff = 0;
    it.for_each(|(l, r)| {
        if l > r {
            diff += l - r;
        } else {
            diff += r - l;
        }
    });

    Some(diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse().unwrap());
        right.push(split.next().unwrap().parse().unwrap());
    });

    let mut similarity = 0;

    left.iter().for_each(|l| {
        similarity += l * right.iter().filter(|r| &l == r).count() as u32;
    });

    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
