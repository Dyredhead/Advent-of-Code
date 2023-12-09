advent_of_code::solution!(4);
// Imports
use std::cmp::min;

pub fn part_one(input: &str) -> Option<usize> {
    let mut points: usize = 0;
    for line in input.lines() {
        let line: Vec<_> = line.split(":").collect::<Vec<_>>()[1].split("|").collect();

        let numbers_winning: Vec<_> = line[0].split_ascii_whitespace().collect();
        let numbers_yours: Vec<_> = line[1].split_ascii_whitespace().collect();
        let mut matches = 0;
        for num in numbers_winning {
            if numbers_yours.contains(&num) {
                matches += 1;
            }
        }
        if matches != 0 {
            points += usize::pow(2, matches - 1);
        }
    }
    return Some(points);
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines: Vec<_> = input.lines().collect();

    let mut card_count = 0;
    let mut card_counts = vec![1; lines.len()];
    for (i, line) in lines.clone().into_iter().enumerate() {
        card_count += card_counts[i];
        let line: Vec<_> = line.split(":").collect::<Vec<_>>()[1].split("|").collect();
        let numbers_winning: Vec<_> = line[0].split_ascii_whitespace().collect();
        let numbers_yours: Vec<_> = line[1].split_ascii_whitespace().collect();
        let mut matches = 0;
        for num in numbers_winning {
            if numbers_yours.contains(&num) {
                matches += 1;
            }
        }
        for j in i + 1..min(i + matches + 1, lines.len()) {
            card_counts[j] += card_counts[i];
        }
    }
    return Some(card_count);
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
