use std::{
    cmp::{max, min},
    collections::HashMap,
    iter::zip,
};

advent_of_code::solution!(1);
// Imports

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>().into_iter();

    let mut left = Vec::new();
    let mut right = Vec::new();

    lines
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .for_each(|line| {
            left.push(line[0].parse::<usize>().unwrap());
            right.push(line[1].parse::<usize>().unwrap());
        });

    left.sort();
    right.sort();

    let mut distance = 0;
    for (i, j) in zip(left, right) {
        distance += max(i, j) - min(i, j);
    }

    return Some(distance);
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>().into_iter();

    let mut hashmap: HashMap<usize, usize> = HashMap::new();

    lines
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .for_each(|line| hashmap.insert(&line[0].parse::<usize>().unwrap()) += 1);

    return Some(0);
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
        assert_eq!(result, None);
    }
}
