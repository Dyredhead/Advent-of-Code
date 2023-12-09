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
        let mut min_key: &str = "";
        let mut min_index: usize = line.len() as usize;
        for key in map.keys() {
            let index: usize;
            match line.find(key) {
                None => {
                    continue;
                }
                Some(i) => {
                    index = i as usize;
                }
            }
            if index < min_index {
                min_index = index;
                min_key = key;
            }
        }
        let a: usize = *map.get(min_key).unwrap();

        let mut max_key: &str = "";
        let mut max_index: i32 = -1;
        for key in map.keys() {
            let index: i32;
            match line.rfind(key) {
                None => {
                    continue;
                }
                Some(i) => {
                    index = i as i32;
                }
            }
            if index > max_index {
                max_index = index;
                max_key = key;
            }
        }
        let b: usize = *map.get(max_key).unwrap();

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
