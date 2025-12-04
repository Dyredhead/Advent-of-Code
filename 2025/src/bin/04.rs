use std::{clone, fmt::Debug};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut counter = 0;
    for (row_index, row_value) in grid.iter().enumerate() {
        for (col_index, col_value) in row_value.iter().enumerate() {
            if *col_value == '.' {
                continue;
            }

            if valid_paperroll(&grid, row_index as isize, col_index as isize) {
                counter += 1;
            }
        }
    }

    return Some(counter);
}

fn valid_paperroll(grid: &Vec<Vec<char>>, row_index: isize, col_index: isize) -> bool {
    let mut sum = 0;
    for row_mod in [-1, 0, 1] {
        for col_mod in [-1, 0, 1] {
            if row_mod == 0 && col_mod == 0 {
                continue;
            }

            if in_bounds(grid, row_index + row_mod, col_index + col_mod)
                && grid[(row_index + row_mod) as usize][(col_index + col_mod) as usize] == '@'
            {
                sum += 1;
            }
        }
    }
    return sum < 4;
}

fn in_bounds(grid: &Vec<Vec<char>>, row_index: isize, col_index: isize) -> bool {
    return row_index >= 0
        && row_index < grid.len() as isize
        && col_index >= 0
        && col_index < grid[row_index as usize].len() as isize;
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut counter = 0;

    loop {
        let mut new_grid = grid.clone();
        let mut flag = false;
        for (row_index, row_value) in grid.iter().enumerate() {
            for (col_index, col_value) in row_value.iter().enumerate() {
                match *col_value {
                    '@' => {
                        if valid_paperroll(&grid, row_index as isize, col_index as isize) {
                            counter += 1;
                            new_grid[row_index][col_index] = 'x';
                            flag |= true;
                        }
                    }
                    _ => continue,
                }
            }
        }
        if flag == false {
            break;
        }
        grid = new_grid;
    }

    return Some(counter);
}

fn print_2dvec<T: Debug>(grid: &Vec<Vec<T>>) {
    grid.iter().for_each(|it| {
        println!("{:?}", it);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
