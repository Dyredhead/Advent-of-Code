#![feature(strip_circumfix)]

use std::collections::{HashSet, VecDeque};
advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<(Vec<bool>, Vec<Vec<usize>>)> = input
        .lines()
        .map(|line| line.rsplit_once(" ").unwrap().0.split_once(" ").unwrap())
        .map(|line| {
            let lights: Vec<bool> = line
                .0
                .strip_circumfix('[', ']')
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => unreachable!(),
                })
                .collect();

            let buttons: Vec<Vec<usize>> = line
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

    let mut sum = 0;
    for line in input {
        let start = vec![false; line.0.len()];
        let mut dp: HashSet<Vec<bool>> = HashSet::new();
        let mut queue: VecDeque<(usize, Vec<bool>)> = VecDeque::from_iter([(0, start)]);
        let buttons = line.1;
        let presses: usize = 'outer: loop {
            let state = queue.pop_front().unwrap();
            let (mut presses, lights) = state;
            presses += 1;
            for button in buttons.iter() {
                let mut new_lights = lights.clone();
                for toggle in button {
                    new_lights[*toggle] = !new_lights[*toggle];
                }
                if new_lights == line.0 {
                    break 'outer presses;
                } else {
                    if !dp.contains(&new_lights) {
                        queue.push_back((presses, new_lights.clone()));
                        dp.insert(new_lights);
                    }
                }
            }
        };

        sum += presses;
    }

    return Some(sum);
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Vec<(Vec<usize>, Vec<Vec<usize>>)> = input
        .lines()
        .map(|line| line.split_once(" ").unwrap().1.rsplit_once(" ").unwrap())
        .map(|line| {
            let buttons: Vec<Vec<usize>> = line
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

            (joltages, buttons)
        })
        .collect();

    let mut sum = 0;
    for line in input {
        let (goal, mut buttons) = line;
        buttons.sort_by_key(|key| key.len());
        buttons.reverse();

        let start = vec![0; goal.len()];
        let mut dp: HashSet<Vec<usize>> = HashSet::new();
        let mut queue: VecDeque<(usize, Vec<usize>)> = VecDeque::from_iter([(0, start)]);

        let presses: usize = 'outer: loop {
            let current = queue.pop_front().unwrap();
            let (mut presses, lights) = current;
            presses += 1;
            'inner: for button in buttons.iter() {
                let mut next_lights = lights.clone();
                for toggle in button {
                    next_lights[*toggle] += 1;
                    if next_lights[*toggle] > goal[*toggle] {
                        continue 'inner;
                    }
                }
                if next_lights == goal {
                    break 'outer presses;
                } else {
                    if !dp.contains(&next_lights) {
                        queue.push_back((presses, next_lights.clone()));
                        dp.insert(next_lights);
                    }
                }
            }
        };

        sum += presses;
    }

    return Some(sum);
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
