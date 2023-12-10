advent_of_code::solution!(1);
// Imports
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    // let mut sum: usize = 0;
    let sum = input.lines().fold(0, |sum: usize, line: &str| -> usize {
        let a: usize = line.chars().find(|c| c.is_ascii_digit()).unwrap() as usize - '0' as usize;
        let b: usize = line.chars().rfind(|c| c.is_ascii_digit()).unwrap() as usize - '0' as usize;

        return sum + a * 10 + b;
    });
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let sum: usize = input.lines().fold(0, |sum: usize, line: &str| {
        let (key_min, _): (&str, usize) =
            map.keys()
                .fold(("", line.len()), |(key_min, index_min), key| {
                    let index: Option<usize> = line.find(key);
                    if index.is_some_and(|index| index < index_min) {
                        return (key, index.unwrap());
                    }

                    return (key_min, index_min);
                });

        let a: usize = *map.get(key_min).unwrap();

        let (key_max, _): (&str, i32) = map.keys().fold(("", -1), |(key_max, index_max), key| {
            let index: Option<usize> = line.rfind(key);
            if index.is_some_and(|index| index as i32 > index_max) {
                return (key, index.unwrap() as i32);
            }

            return (key_max, index_max);
        });
        let b: usize = *map.get(key_max).unwrap();

        return sum + a * 10 + b;
    });

    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
