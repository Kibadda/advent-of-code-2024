use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let split = input.split_once("\n\n").unwrap();

    let mut ordering: HashMap<u32, Vec<u32>> = HashMap::new();

    split.0.lines().for_each(|line| {
        let mut s = line.split('|');
        let key = s.next().unwrap().parse().unwrap();
        let value = s.next().unwrap().parse().unwrap();

        match ordering.get(&key) {
            Some(l) => {
                let mut c = l.clone();
                c.push(value);
                ordering.insert(key, c);
            }
            None => {
                ordering.insert(key, vec![value]);
            }
        }
    });

    let updates = split
        .1
        .lines()
        .map(|line| {
            line.split(',')
                .rev()
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    (ordering, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ordering, updates) = parse_input(input);

    Some(
        updates
            .iter()
            .filter(|upd| {
                for i in 0..upd.len() {
                    for j in (i + 1)..upd.len() {
                        if ordering.contains_key(&upd[i]) && ordering[&upd[i]].contains(&upd[j]) {
                            return false;
                        }
                    }
                }

                true
            })
            .map(|upd| upd[upd.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ordering, updates) = parse_input(input);

    Some(
        updates
            .iter()
            .filter(|upd| {
                for i in 0..upd.len() {
                    for j in (i + 1)..upd.len() {
                        if ordering.contains_key(&upd[i]) && ordering[&upd[i]].contains(&upd[j]) {
                            return true;
                        }
                    }
                }

                false
            })
            .map(|upd| {
                let mut c = upd.clone();
                c.sort_by(|a, b| {
                    if ordering.contains_key(a) && ordering[a].contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                c
            })
            .map(|upd| upd[upd.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
