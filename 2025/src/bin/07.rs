use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = (0, input[0].iter().position(|&c| c == 'S').unwrap());

    let mut sum = 0;
    let mut current: HashSet<(usize, usize)> = HashSet::from([start]);
    loop {
        let mut next: HashSet<(usize, usize)> = HashSet::new();
        for (row, col) in current.iter() {
            if row + 1 < input.len() {
                let char = input[row + 1][*col];
                match char {
                    '.' => {
                        next.insert((row + 1, *col));
                    }
                    '^' => {
                        next.insert((row + 1, col - 1));
                        next.insert((row + 1, col + 1));
                        sum += 1;
                    }
                    _ => unreachable!(),
                }
            }
        }
        if next.is_empty() {
            break;
        }
        current = next;
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<usize> {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = (0, input[0].iter().position(|&c| c == 'S').unwrap());

    let mut dp: HashMap<(usize, usize), usize> = HashMap::new();

    let time_lines = calc_timelines(&input, start, &mut dp);
    return Some(time_lines);
}

fn calc_timelines(
    grid: &Vec<Vec<char>>,
    position: (usize, usize),
    dp: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(timelines) = dp.get(&position) {
        return *timelines;
    }

    if position.0 + 1 >= grid.len() {
        return 1;
    }

    let timelines = match grid[position.0 + 1][position.1] {
        '.' => calc_timelines(grid, (position.0 + 1, position.1), dp),
        '^' => {
            calc_timelines(grid, (position.0 + 1, position.1 - 1), dp)
                + calc_timelines(grid, (position.0 + 1, position.1 + 1), dp)
        }
        _ => unreachable!(),
    };

    dp.insert(position, timelines);
    return timelines;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
