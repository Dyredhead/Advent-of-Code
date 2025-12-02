advent_of_code::solution!(13);
// Imports

pub fn part_one(input: &str) -> Option<usize> {
    let binding = input.lines().collect::<Vec<&str>>();
    let mut lines = binding.iter();

    let mut patterns = Vec::new();
    let mut pattern: Vec<Vec<char>> = Vec::new();

    let tmp = lines.next().unwrap();
    if tmp != &"" {
        pattern.push(tmp.chars().collect())
    }

    while let Some(line) = lines.next() {
        if line == &"" {
            patterns.push(pattern);
            pattern = Vec::new();
        } else {
            pattern.push(line.chars().collect());
        }
    }
    patterns.push(pattern);

    let mut sum = 0;
    for pattern in patterns {
        // vertical:
        let mut sum_v = 0;
        for col in 0..pattern[0].len() - 1 {
            if cols_equal(&pattern, col, col + 1) {
                let mut lo = col as i32;
                let mut hi = col + 1;

                let mut eq = true;
                while eq && 0 <= lo && hi < pattern[0].len() {
                    if cols_equal(&pattern, lo as usize, hi) {
                        lo -= 1;
                        hi += 1;
                    } else {
                        eq = false;
                    }
                }
                if eq {
                    sum_v = col + 1;
                }
            }
        }
        // horizontal:
        let mut sum_h = 0;
        for row in 0..pattern.len() - 1 {
            if rows_equal(&pattern, row, row + 1) {
                let mut lo = row as i32;
                let mut hi = row + 1;

                let mut eq = true;
                while eq && 0 <= lo && hi < pattern.len() {
                    if rows_equal(&pattern, lo as usize, hi) {
                        lo -= 1;
                        hi += 1;
                    } else {
                        eq = false;
                    }
                }
                if eq {
                    sum_h = (row + 1) * 100;
                }
            }
        }
        sum += sum_v + sum_h;
    }
    return Some(sum);
}

fn rows_equal(pattern: &Vec<Vec<char>>, a: usize, b: usize) -> bool {
    return pattern[a]
        .iter()
        .zip(pattern[b].iter())
        .all(|(i, j)| i == j);
}

fn cols_equal(pattern: &Vec<Vec<char>>, a: usize, b: usize) -> bool {
    return pattern.iter().all(|i| i[a] == i[b]);
}

pub fn part_two(input: &str) -> Option<usize> {
    let binding = input.lines().collect::<Vec<&str>>();
    let mut lines = binding.iter();

    let mut patterns = Vec::new();
    let mut pattern: Vec<Vec<char>> = Vec::new();

    let tmp = lines.next().unwrap();
    if tmp != &"" {
        pattern.push(tmp.chars().collect())
    }

    while let Some(line) = lines.next() {
        if line == &"" {
            patterns.push(pattern);
            pattern = Vec::new();
        } else {
            pattern.push(line.chars().collect());
        }
    }
    patterns.push(pattern);

    let mut sum = 0;
    for pattern in patterns {
        // vertical:
        let mut sum_v = 0;
        for col in 0..pattern[0].len() - 1 {
            if let Some(mut s) = cols_semi_equal(&pattern, col, col + 1) {
                let mut lo = col as i32 - 1;
                let mut hi = col + 2;

                let mut eq = true;
                while eq && 0 <= lo && hi < pattern.len() {
                    let k: bool = if s {
                        if let Some(j) = cols_semi_equal(&pattern, lo as usize, hi) {
                            if !j {
                                s = s && j;
                            }
                            true
                        } else {
                            false
                        }
                    } else {
                        cols_equal(&pattern, lo as usize, hi)
                    };
                    if k {
                        lo -= 1;
                        hi += 1;
                    } else {
                        eq = false;
                    }
                }
                if eq {
                    println!("col: {col}");
                    sum_v += col + 1;
                }
            }
        }

        // horizontal:
        let mut sum_h = 0;
        for row in 0..pattern.len() - 1 {
            if let Some(mut s) = rows_semi_equal(&pattern, row, row + 1) {
                let mut lo = (row as i32) - 1;
                let mut hi = row + 2;

                let mut eq = true;
                while eq && 0 <= lo && hi < pattern.len() {
                    let k: bool = if s {
                        if let Some(j) = rows_semi_equal(&pattern, lo as usize, hi) {
                            if !j {
                                s = s && j;
                            }
                            true
                        } else {
                            false
                        }
                    } else {
                        rows_equal(&pattern, lo as usize, hi)
                    };
                    if k {
                        lo -= 1;
                        hi += 1;
                    } else {
                        eq = false;
                    }
                }
                if eq {
                    println!("row: {row}");
                    sum_h += (row + 1) * 100;
                }
            }
        }
        sum += sum_v + sum_h;
    }
    return Some(sum);
}

fn rows_semi_equal(pattern: &Vec<Vec<char>>, a: usize, b: usize) -> Option<bool> {
    let s = pattern[a]
        .iter()
        .zip(pattern[b].iter())
        .fold(0, |s, (i, j)| {
            if i != j {
                return s + 1;
            } else {
                return s;
            }
        });
    if s == 0 {
        return Some(true);
    } else if s == 1 {
        return Some(false);
    } else {
        return None;
    }
}

fn cols_semi_equal(pattern: &Vec<Vec<char>>, a: usize, b: usize) -> Option<bool> {
    let s = pattern.iter().fold(0, |s, i| {
        if i[a] != i[b] {
            return s + 1;
        } else {
            return s;
        }
    });
    if s == 0 {
        return Some(true);
    } else if s == 1 {
        return Some(false);
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
