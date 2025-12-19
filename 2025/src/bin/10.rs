#![feature(strip_circumfix)]

use frozenset::Freeze;
use frozenset::FrozenSet;
use itertools::Itertools;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<(HashSet<usize>, Vec<HashSet<usize>>)> = input
        .lines()
        .map(|line| line.rsplit_once(" ").unwrap().0.split_once(" ").unwrap())
        .map(|line| {
            let lights: HashSet<usize> = line
                .0
                .strip_circumfix('[', ']')
                .unwrap()
                .char_indices()
                .filter(|(_, c)| *c == '#')
                .map(|(i, _)| i)
                .collect();

            let buttons: Vec<HashSet<usize>> = line
                .1
                .split_whitespace()
                .map(|schematic| {
                    schematic
                        .strip_circumfix('(', ')')
                        .unwrap()
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect();

            (lights, buttons)
        })
        .collect();

    fn get_presses(indicators: HashSet<usize>, buttons: Vec<HashSet<usize>>) -> Option<usize> {
        for num_presses in 0..(buttons.len() + 1) {
            for presses in buttons.iter().combinations(num_presses) {
                let mut pattern = HashSet::new();
                for button in presses {
                    pattern = pattern
                        .symmetric_difference(&button)
                        .map(|n| n.to_owned())
                        .collect();
                }
                if pattern == indicators {
                    return Some(num_presses);
                }
            }
        }
        return None;
    }

    return Some(
        input
            .into_iter()
            .fold(0, |acc, line| acc + get_presses(line.0, line.1).unwrap()),
    );
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Vec<(Vec<HashSet<usize>>, Vec<usize>)> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap().1.rsplit_once(" ").unwrap())
        .map(|line| {
            let buttons: Vec<HashSet<usize>> = line
                .0
                .split_whitespace()
                .map(|schematic| {
                    schematic
                        .strip_circumfix('(', ')')
                        .unwrap()
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect();

            let joltages: Vec<usize> = line
                .1
                .strip_circumfix('{', '}')
                .unwrap()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect();

            (buttons, joltages)
        })
        .collect();

    fn get_patterns(
        buttons: Vec<HashSet<usize>>,
    ) -> HashMap<FrozenSet<usize>, Vec<Vec<HashSet<usize>>>> {
        let mut map_patterns: HashMap<FrozenSet<usize>, Vec<Vec<HashSet<usize>>>> = HashMap::new();
        for num_presses in 0..(buttons.len() + 1) {
            for presses in buttons.iter().combinations(num_presses) {
                let mut pattern = HashSet::new();
                for button in &presses {
                    pattern = pattern
                        .symmetric_difference(&button)
                        .map(|n| n.to_owned())
                        .collect();
                }
                let presses = presses.iter().map(|&i| i.to_owned()).collect();
                let frozen_map = pattern.freeze();
                match map_patterns.get_mut(&frozen_map) {
                    Some(patterns) => patterns.push(presses),
                    None => {
                        map_patterns.insert(frozen_map, Vec::from_iter([presses]));
                    }
                }
            }
        }
        return map_patterns;
    }

    fn solve_pattern(
        joltages: Vec<usize>,
        patterns: HashMap<FrozenSet<usize>, Vec<Vec<HashSet<usize>>>>,
    ) -> Option<usize> {
        fn min_presses(
            target_before: Vec<usize>,
            patterns: &HashMap<FrozenSet<usize>, Vec<Vec<HashSet<usize>>>>,
        ) -> Option<usize> {
            if target_before.iter().all(|&i| i == 0) {
                return Some(0);
            }

            let indicators = target_before
                .iter()
                .enumerate()
                .filter(|(_, j)| *j % 2 == 1)
                .map(|(i, _)| i)
                .collect::<FrozenSet<usize>>();

            let mut result: Option<usize> = None;
            'outer: for presses in patterns.get(&indicators).unwrap() {
                let mut target_after = target_before.clone();
                for button in presses {
                    for &joltage_index in button {
                        match target_after[joltage_index].checked_sub(1) {
                            Some(n) => target_after[joltage_index] = n,
                            None => continue 'outer,
                        }
                    }
                }
                let target_half = target_after.iter().map(|joltage| joltage / 2).collect();
                let num_target_half_presses = min_presses(target_half, patterns);
                if num_target_half_presses.is_none() {
                    continue;
                }
                let num_target_half_presses = num_target_half_presses.unwrap();
                let num_presses = presses.len() + 2 * num_target_half_presses;

                if result.is_none() {
                    result = Some(num_presses);
                } else {
                    result = Some(min(result.unwrap(), num_presses));
                }
            }
            return result;
        }
        return min_presses(joltages, &patterns);
    }

    return Some(input.into_iter().fold(0, |acc, line| {
        acc + solve_pattern(line.1, get_patterns(line.0)).unwrap_or(0)
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
