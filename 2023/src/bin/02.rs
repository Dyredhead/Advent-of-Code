advent_of_code::solution!(2);
// Imports
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let map = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut sum: usize = 0;
    for game in input.lines() {
        let game: Vec<&str> = game.split(":").collect();
        let id: usize = game[0].split(" ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let game: Vec<&str> = game[1].split(";").collect();

        let mut valid_game: bool = true;
        for set in game {
            let set: Vec<&str> = set.split(",").collect();
            let mut valid_set: bool = true;
            for cubes in set {
                let cubes: Vec<&str> = cubes[1..].split(" ").collect();
                let n: usize = cubes[0].parse().unwrap();
                let color: &str = cubes[1];
                if n > *map.get(color).unwrap() {
                    valid_set = false;
                    break;
                }
            }
            if !valid_set {
                valid_game = false;
                break;
            }
        }
        if valid_game {
            sum += id;
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum: usize = 0;
    for game in input.lines() {
        let game: Vec<&str> = game.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect();

        let mut map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for set in game {
            let set: Vec<&str> = set.split(",").collect();
            for cubes in set {
                let cubes: Vec<&str> = cubes[1..].split(" ").collect();
                let n: usize = cubes[0].parse().unwrap();
                let color: &str = cubes[1];

                if n > *map.get(color).unwrap() {
                    map.insert(color, n);
                }
            }
        }
        let mut power: usize = 1;
        for val in map.values() {
            power *= val;
        }
        sum += power;
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
