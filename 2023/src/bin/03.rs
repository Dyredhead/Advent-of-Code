advent_of_code::solution!(3);
// Imports
use std::cmp::{max, min};

fn get_neighbors(engine: &Vec<Vec<char>>, center: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let max_row = engine.len();
    let max_col = engine[0].len();
    let i = center.0;
    let j = center.1;
    for y in max(0, i - 1)..min(i + 1, max_row) + 1 {
        for x in max(0, j - 1)..min(j + 1, max_col) + 1 {
            if (y != i || x != j) && engine[y][x].is_ascii_digit() {
                neighbors.push((y, x));
            }
        }
    }
    return neighbors;
}

pub fn part_one(input: &str) -> Option<usize> {
    fn find_symbol(engine: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for i in 0..engine.len() {
            for j in 0..engine[i].len() {
                if !(engine[i][j].is_ascii_digit() || engine[i][j] == '.' || engine[i][j] == '_') {
                    return Some((i, j));
                }
            }
        }
        return None;
    }

    let mut engine: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        engine.push(line.chars().collect());
    }

    let mut sum: usize = 0;
    while find_symbol(&engine) != None {
        let symbol = find_symbol(&engine).unwrap();
        engine[symbol.0][symbol.1] = '_';

        let neighbors = get_neighbors(&engine, symbol);
        for neighbor in neighbors {
            if engine[neighbor.0][neighbor.1].is_ascii_digit() {
                let mut i = 0;
                for _i in (0..neighbor.1).rev() {
                    if !engine[neighbor.0][_i].is_ascii_digit() {
                        i = _i + 1;
                        break;
                    }
                }
                let mut j = engine[neighbor.0].len();
                for _j in (neighbor.1)..(engine[neighbor.0].len()) {
                    if !engine[neighbor.0][_j].is_ascii_digit() {
                        j = _j;
                        break;
                    }
                }

                let mut number = String::new();
                for k in i..j {
                    number.push(engine[neighbor.0][k]);
                    engine[neighbor.0][k] = '_';
                }
                let number: usize = number.parse().unwrap();
                sum += number;
            }
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<usize> {
    fn find_symbol(engine: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for i in 0..engine.len() {
            for j in 0..engine[i].len() {
                if engine[i][j] == '*' {
                    return Some((i, j));
                }
            }
        }
        return None;
    }

    let mut engine: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        engine.push(line.chars().collect());
    }

    let mut sum = 0;
    while find_symbol(&engine) != None {
        let symbol = find_symbol(&engine).unwrap();
        engine[symbol.0][symbol.1] = '_';

        let neighbors = get_neighbors(&engine, symbol);
        let mut product = 1;
        let mut parts = 0;
        for neighbor in neighbors {
            if engine[neighbor.0][neighbor.1].is_ascii_digit() {
                let mut i = 0;
                for _i in (0..neighbor.1).rev() {
                    if !engine[neighbor.0][_i].is_ascii_digit() {
                        i = _i + 1;
                        break;
                    }
                }
                let mut j = engine[neighbor.0].len();
                for _j in (neighbor.1)..(engine[neighbor.0].len()) {
                    if !engine[neighbor.0][_j].is_ascii_digit() {
                        j = _j;
                        break;
                    }
                }

                let mut number = String::new();
                for k in i..j {
                    number.push(engine[neighbor.0][k]);
                    engine[neighbor.0][k] = '_';
                }

                let number: usize = number.parse().unwrap();
                product *= number;
                parts += 1;
            }
        }
        if parts == 2 {
            sum += product;
        }
    }
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
