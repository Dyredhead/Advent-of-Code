advent_of_code::solution!(10);
// Imports
use std::collections::HashMap;

fn get_next(
    field: &Vec<Vec<char>>,
    map_symbol_to_offsets: &HashMap<char, [(i32, i32); 2]>,
    p: (usize, usize),
    pp: (usize, usize),
) -> (usize, usize) {
    let offsets = map_symbol_to_offsets.get(&field[p.0][p.1]).unwrap();
    let neighbors = [
        (
            (p.0 as i32 + offsets[0].0) as usize,
            (p.1 as i32 + offsets[0].1) as usize,
        ),
        (
            (p.0 as i32 + offsets[1].0) as usize,
            (p.1 as i32 + offsets[1].1) as usize,
        ),
    ];

    return *neighbors
        .iter()
        .filter(|&&neighbor| neighbor != pp)
        .collect::<Vec<_>>()[0];
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>().into_iter();

    let map_symbol_to_symbol: HashMap<char, char> = HashMap::from([
        ('|', '║'),
        ('-', '═'),
        ('L', '╚'),
        ('J', '╝'),
        ('7', '╗'),
        ('F', '╔'),
        ('.', '.'),
        ('S', 'S'),
    ]);

    let map_symbol_to_offsets: HashMap<char, [(i32, i32); 2]> = HashMap::from([
        ('║', [(-1, 0), (1, 0)]),
        ('═', [(0, -1), (0, 1)]),
        ('╚', [(-1, 0), (0, 1)]),
        ('╝', [(-1, 0), (0, -1)]),
        ('╗', [(0, -1), (1, 0)]),
        ('╔', [(0, 1), (1, 0)]),
    ]);

    let mut field: Vec<Vec<char>> = lines.map(|line| line.chars().collect::<Vec<_>>()).collect();
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            field[row][col] = *map_symbol_to_symbol.get(&field[row][col]).unwrap();
        }
    }

    let (row, col): (Option<usize>, Option<usize>) =
        (0..field.len()).fold((None, None), |index_start, row| {
            let col = field[row].iter().position(|&c| c == 'S');
            if col.is_some() {
                return (Some(row), col);
            }
            return index_start;
        });

    let start: (usize, usize) = (row.unwrap(), col.unwrap());
    let mut visited: Vec<(usize, usize)> = Vec::from([start]);
    // TODO dynamicly find pointer
    let mut p = (69, 89);
    let mut pp = start;
    while p != start {
        let tmp = p;
        p = get_next(&field, &map_symbol_to_offsets, p, pp);
        visited.push(p);
        pp = tmp;
    }

    return Some(((visited.len()) as f64 / 2.0).ceil() as usize);
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>().into_iter();

    let map_symbol_to_symbol: HashMap<char, char> = HashMap::from([
        ('|', '║'),
        ('-', '═'),
        ('L', '╚'),
        ('J', '╝'),
        ('7', '╗'),
        ('F', '╔'),
        ('.', '.'),
        ('S', 'S'),
    ]);

    let map_symbol_to_offsets: HashMap<char, [(i32, i32); 2]> = HashMap::from([
        ('║', [(-1, 0), (1, 0)]),
        ('═', [(0, -1), (0, 1)]),
        ('╚', [(-1, 0), (0, 1)]),
        ('╝', [(-1, 0), (0, -1)]),
        ('╗', [(0, -1), (1, 0)]),
        ('╔', [(0, 1), (1, 0)]),
    ]);

    let mut field: Vec<Vec<char>> = lines.map(|line| line.chars().collect::<Vec<_>>()).collect();
    for row in 0..field.len() {
        for col in 0..field[row].len() {
            field[row][col] = *map_symbol_to_symbol.get(&field[row][col]).unwrap();
        }
    }

    let (row, col): (Option<usize>, Option<usize>) =
        (0..field.len()).fold((None, None), |index_start, row| {
            let col = field[row].iter().position(|&c| c == 'S');
            if col.is_some() {
                return (Some(row), col);
            }
            return index_start;
        });

    let mut field = field.clone();
    let start: (usize, usize) = (row.unwrap(), col.unwrap());
    let mut visited: Vec<(usize, usize)> = Vec::from([start]);
    let mut p = (69, 89);
    let mut pp = start;
    while p != start {
        let tmp = p;
        field[pp.0][pp.1] = '*';
        // field[pp.0][pp.1] = '🟩';
        p = get_next(&field, &map_symbol_to_offsets, p, pp);
        visited.push(p);
        pp = tmp;
    }

    for row in 0..field.len() {
        for col in 0..field[row].len() {
            if field[row][col] != '*' {
                field[row][col] = '.';
            }
            // if field[row][col] != '🟩' {
            //     field[row][col] = '🟥';
            // }
        }
    }

    let mut area: i32 = 0;
    for i in 0..visited.len() - 1 {
        let a = visited[i];
        let b = visited[i + 1];
        area += (a.1 as i32 * b.0 as i32) - (a.0 as i32 * b.1 as i32);
        // println!("{area}");
    }

    return Some((((area - visited.len() as i32) / 2) + 1) as usize);
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
