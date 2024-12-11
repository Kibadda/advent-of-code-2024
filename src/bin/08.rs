use std::collections::HashMap;

advent_of_code::solution!(8);

type ParsedInput = (HashMap<char, Vec<(usize, usize)>>, i32, i32);

fn parse_input(input: &str) -> ParsedInput {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut width: i32 = 0;
    let height: i32 = input.lines().count() as i32;

    input.lines().enumerate().for_each(|(i, line)| {
        width = line.len() as i32;

        line.chars().enumerate().for_each(|(j, c)| {
            if c != '.' {
                if let Some(a) = antennas.get(&c) {
                    let mut b = a.clone();
                    b.push((i, j));
                    antennas.insert(c, b);
                } else {
                    antennas.insert(c, vec![(i, j)]);
                }
            }
        });
    });

    (antennas, width, height)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennas, width, height) = parse_input(input);

    let mut antinodes: HashMap<String, bool> = HashMap::new();
    antennas.iter().for_each(|(_, a)| {
        for i in 0..a.len() {
            for j in 0..a.len() {
                if i != j {
                    let diff_x = a[i].0 as i32 - a[j].0 as i32;
                    let diff_y = a[i].1 as i32 - a[j].1 as i32;

                    let pos = (a[i].0 as i32 + diff_x, a[i].1 as i32 + diff_y);

                    if pos.0 >= 0 && pos.0 < height && pos.1 >= 0 && pos.1 < width {
                        antinodes.insert(format!("{}|{}", pos.0, pos.1), true);
                    }
                }
            }
        }
    });

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennas, width, height) = parse_input(input);

    let mut antinodes: HashMap<String, bool> = HashMap::new();
    antennas.iter().for_each(|(_, a)| {
        for i in 0..a.len() {
            antinodes.insert(format!("{}|{}", a[i].0, a[i].1), true);

            for j in 0..a.len() {
                if i != j {
                    let mut pos = (a[i].0 as i32, a[i].1 as i32);
                    let diff_x = a[i].0 as i32 - a[j].0 as i32;
                    let diff_y = a[i].1 as i32 - a[j].1 as i32;

                    loop {
                        pos = (pos.0 + diff_x, pos.1 + diff_y);

                        if pos.0 >= 0 && pos.0 < height && pos.1 >= 0 && pos.1 < width {
                            antinodes.insert(format!("{}|{}", pos.0, pos.1), true);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    });

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
