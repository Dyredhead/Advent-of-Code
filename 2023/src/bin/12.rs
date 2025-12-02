use std::collections::HashMap;

advent_of_code::solution!(12);
// Imports

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>().into_iter();
    let mut sum: usize = 0;
    for line in lines {
        let line: Vec<&str> = line.split(" ").collect();
        let springs: Vec<char> = line[0].chars().collect();
        let records: Vec<usize> = line[1]
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let matches = &DFS(&springs, &records);
        sum += matches;
    }

    return Some(sum);
}

// fn DFS(springs: &Vec<char>) -> Vec<Vec<char>> {
//     fn _DFS(combinations: &mut Vec<Vec<char>>, springs: &Vec<char>, i: usize) {
//         let mut springs_a = springs.clone();
//         springs_a[i] = '#';
//         let mut springs_b = springs.clone();
//         springs_b[i] = '.';
//         let next = springs.iter().skip(i + 1).position(|&c| c == '?');
//         if next.is_some_and(|next| next + i + 1 < springs.len()) {
//             _DFS(combinations, &springs_a, next.unwrap() + i + 1);
//             _DFS(combinations, &springs_b, next.unwrap() + i + 1);
//         } else {
//             if combinations
//             combinations.push(springs_a);
//             combinations.push(springs_b);
//         }
//     }

//     let mut combinations = Vec::new();
//     _DFS(
//         &mut combinations,
//         springs,
//         springs.iter().position(|&c| c == '?').unwrap(),
//     );
//     println!("Done Combining!");
//     return combinations;
//}

fn DFS(springs: &Vec<char>, records: &Vec<usize>) -> usize {
    fn _DFS(
        map: &mut HashMap<(usize, usize, usize), usize>,
        springs: &Vec<char>,
        s_i: usize,
        records: &Vec<usize>,
        r_i: usize,
        s_l: usize,
    ) -> usize {
        let mut springs_a = springs.clone();
        springs_a[i] = '#';
        let mut springs_b = springs.clone();
        springs_b[i] = '.';
        let next = springs.iter().skip(i + 1).position(|&c| c == '?');
        if next.is_some_and(|next| next + i + 1 < springs.len()) {
            return _DFS(map, &springs_a, next.unwrap() + i + 1, records)
                + _DFS(map, &springs_b, next.unwrap() + i + 1, records);
        } else {
        }
    }

    let sum = _DFS(
        &mut HashSet::new(),
        springs,
        springs.iter().position(|&c| c == '?').unwrap(),
        records,
    );
    return sum;
}

fn count_matches(combinations: &Vec<Vec<char>>, records: &Vec<usize>) -> usize {
    let mut map_matches = HashSet::new();
    let mut map_non_matches = HashSet::new();
    let mut sum: usize = 0;
    // println!("{:?}", records);
    for spring in combinations {
        let spring = spring.iter().collect::<String>();
        let spring: Vec<String> = spring
            .split(".")
            .filter(|&s| s != "")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if map_matches.contains(&spring) {
            sum += 1;
            continue;
        } else if map_non_matches.contains(&spring) {
            continue;
        } else {
            if &spring.len() == &records.len()
                && (&spring)
                    .iter()
                    .enumerate()
                    .all(|(i, val)| i < records.len() && val.len() == records[i])
            {
                sum += 1;
                map_matches.insert(spring);
            } else {
                map_non_matches.insert(spring);
            }
        }
    }
    println!("Done Matching!");
    return sum;
}

// fn find_contingous_section(springs: &Vec<char>, i: usize) -> Range<usize> {
//     let mut lo = i;
//     while lo > 0 && springs[lo - 1] != '.' {
//         lo -= 1;
//     }

//     let mut hi = i;
//     while hi < springs.len() - 1 && springs[hi + 1] != '.' {
//         hi += 1;
//     }

//     return (lo..hi + 1);
// }

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<&str>>().into_iter();
    let repetitions = 5;

    let mut sum: usize = 0;
    for line in lines {
        let line: Vec<&str> = line.split(" ").collect();
        let springs: Vec<char> = (0..repetitions - 1)
            .fold(String::from(line[0]), |mut springs, _| {
                springs.push_str("?");
                springs.push_str(line[0]);
                return springs;
            })
            .chars()
            .collect();

        let records: Vec<usize> = (0..repetitions - 1)
            .fold(String::from(line[1]), |mut records, _| {
                records.push_str(",");
                records.push_str(line[1]);
                return records;
            })
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let matches = &DFS(&springs, &records);
        sum += matches;
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
