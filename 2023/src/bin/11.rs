advent_of_code::solution!(11);
// Imports
use std::cmp::{max, min};

pub fn part_one(input: &str) -> Option<usize> {
    let universe: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut universe_new: Vec<Vec<char>> = Vec::new();
    for row in 0..universe.len() {
        if universe[row].contains(&'#') {
            universe_new.push(universe[row].clone());
        } else {
            universe_new.push(universe[row].clone());
            universe_new.push(universe[row].clone());
        }
    }
    let universe = universe_new;
    universe_new = Vec::from(vec![Vec::new(); universe.len()]);
    for col in 0..universe[0].len() {
        if universe.iter().any(|row| row[col] == '#') {
            for row in 0..universe_new.len() {
                universe_new[row].push(universe[row][col]);
            }
        } else {
            for row in 0..universe_new.len() {
                universe_new[row].push('.');
                universe_new[row].push('.');
            }
        }
    }
    let universe = universe_new;

    // universe
    //     .iter()
    //     .for_each(|row| println!("{:?}", row.iter().collect::<String>()));

    let mut galaxies = Vec::new();
    for row in 0..universe.len() {
        for col in 0..universe[row].len() {
            if universe[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let mut sum: usize = 0;
    for a in &galaxies {
        for b in &galaxies {
            if a != b {
                sum += ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize;
            }
        }
    }
    return Some(sum / 2);

    // return Some(0);
}

pub fn part_two(input: &str) -> Option<usize> {
    let multiplier: usize = 1_000_000;

    let universe: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut galaxies = Vec::new();
    for row in 0..universe.len() {
        for col in 0..universe[row].len() {
            if universe[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    let mut universe_1: Vec<Vec<usize>> = Vec::new();
    for row in 0..universe.len() {
        if universe[row].contains(&'#') {
            universe_1.push(vec![1; universe[row].len()]);
        } else {
            universe_1.push(vec![multiplier; universe[row].len()]);
        }
    }
    let mut universe_2 = Vec::from(vec![Vec::new(); universe.len()]);
    for col in 0..universe[0].len() {
        if universe.iter().any(|row| row[col] == '#') {
            for row in 0..universe.len() {
                universe_2[row].push(universe_1[row][col]);
            }
        } else {
            for row in 0..universe.len() {
                universe_2[row].push(multiplier);
            }
        }
    }
    let universe = universe_2;

    // universe.iter().for_each(|row| println!("{:?}", row));

    let mut sum: usize = 0;
    for a in &galaxies {
        for b in &galaxies {
            if a != b {
                let rows = (min(a.0, b.0)..max(a.0, b.0))
                    .fold(0, |_sum: usize, i| _sum + universe[i][a.1]);
                let cols = (min(a.1, b.1)..max(a.1, b.1))
                    .fold(0, |_sum: usize, i| _sum + universe[a.0][i]);

                sum += rows + cols;
            }
        }
    }
    return Some(sum / 2);
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
