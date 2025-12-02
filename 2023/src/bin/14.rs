use std::collections::{HashMap, HashSet};

advent_of_code::solution!(14);
// Imports

pub fn part_one(input: &str) -> Option<usize> {
    let mut field: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut sum = 0;
    for r in 0..field.len() {
        for c in 0..field[r].len() {
            if field[r][c] == 'O' {
                field[r][c] = '.';
                let mut i = r;
                while 0 < i && field[i - 1][c] == '.' {
                    i -= 1;
                }
                field[i][c] = 'O';
                sum += field.len() - i;
            }
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut field: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..field.len() {
        for c in 0..field[r].len() {
            if field[r][c] == 'O' {
                field[r][c] = '.';
                rocks.insert((r, c));
            }
        }
    }
    let iterations = 100;
    let mut seen: Vec<HashSet<(usize, usize)>> = Vec::new();
    seen.push(rocks.clone());

    for i in 1..=iterations {
        rocks = north(&field, &mut rocks);
        rocks = west(&field, &mut rocks);
        rocks = south(&field, &mut rocks);
        rocks = east(&field, &mut rocks);
        let mut sum = 0;
        for rock in &rocks {
            let (r, _) = rock;
            sum += field.len() - r;
        }
        println!("{i}: {sum}");
        // println!("{i}: {:?}", rocks);
        // if let Some(j) = seen.iter().position(|_rocks| _rocks == &rocks) {
        //     println!("loops at {i}:{j}");
        // } else {
        //     seen.push(rocks.clone());
        // }
        // println!("\n{}:", i + 1);
        // field
        //     .iter()
        //     .for_each(|row| println!("{}", row.iter().collect::<String>()));
    }

    // let mut sum = 0;
    // for rock in rocks {
    //     let (r, _) = rock;
    //     sum += field.len() - r;
    // }
    return Some(0);
}

fn north(field: &Vec<Vec<char>>, rocks: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut _rocks = rocks.iter().collect::<Vec<&(usize, usize)>>();
    _rocks.sort_by(|a, b| a.0.cmp(&b.0));

    let mut rocks_new = HashSet::with_capacity(rocks.len());
    for rock in _rocks {
        let (r, c) = *rock;
        let mut i = r;
        while 0 < i && field[i - 1][c] != '#' && !rocks.contains(&(i - 1, c)) {
            i -= 1;
        }
        rocks_new.insert((i, c));
    }
    return rocks_new;
}

fn west(field: &Vec<Vec<char>>, rocks: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut _rocks = rocks.iter().collect::<Vec<&(usize, usize)>>();
    _rocks.sort_by(|a, b| a.1.cmp(&b.1));

    let mut rocks_new = HashSet::with_capacity(rocks.len());
    for rock in _rocks {
        let (r, c) = *rock;
        let mut i = c;
        while 0 < i && field[r][i - 1] != '#' && !rocks.contains(&(r, i - 1)) {
            i -= 1;
        }
        rocks_new.insert((r, i));
    }
    return rocks_new;
}
fn south(field: &Vec<Vec<char>>, rocks: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut _rocks = rocks.iter().collect::<Vec<&(usize, usize)>>();

    _rocks.sort_by(|a, b| b.0.cmp(&a.0));
    let mut rocks_new = HashSet::with_capacity(rocks.len());
    for rock in _rocks {
        let (r, c) = *rock;
        let mut i = r;
        while i < field.len() - 1 && field[i + 1][c] != '#' && !rocks.contains(&(i + 1, c)) {
            i += 1;
        }
        rocks_new.insert((i, c));
    }
    return rocks_new;
}
fn east(field: &Vec<Vec<char>>, rocks: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut _rocks = rocks.iter().collect::<Vec<&(usize, usize)>>();

    _rocks.sort_by(|a, b| b.1.cmp(&a.1));
    let mut rocks_new = HashSet::with_capacity(rocks.len());
    for rock in _rocks {
        let (r, c) = *rock;
        let mut i = c;
        while i < field[0].len() - 1 && field[r][i + 1] != '#' && !rocks.contains(&(r, i + 1)) {
            i += 1
        }
        rocks_new.insert((r, i));
    }
    return rocks_new;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
