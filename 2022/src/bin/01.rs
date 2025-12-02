advent_of_code::solution!(1);
// Imports
use itertools::Itertools;
use std::cmp::Reverse;

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>().into_iter();

    return Some(0);
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>().into_iter();

    let n = lines
        .map(|i| i.parse::<usize>().ok())
        .batching(|i| i.map_while(|j| j).sum1::<usize>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<usize>();

    return Some(0);
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
